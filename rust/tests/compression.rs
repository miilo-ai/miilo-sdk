#![cfg(feature = "compression")]
use miilo_adapter_sdk::compression::{Codec, Limits, bundle, codecs};
use std::io::{self, Read};
fn codecs() -> [Codec; 3] {
    [Codec::Gzip, Codec::Brotli, Codec::Zstd]
}
#[test]
fn exact_roundtrips_and_corruption_rejection() {
    for codec in codecs() {
        for input in [
            Vec::new(),
            "Unicode café 🦉\n".repeat(1000).into_bytes(),
            (0..=255).cycle().take(100000).collect(),
        ] {
            let encoded = bundle::encode(codec, &input, Limits::default()).unwrap();
            assert_eq!(
                bundle::decode(&encoded, Limits::default()).unwrap().1,
                input
            );
            for n in [0, 8, 12, encoded.len() - 1] {
                assert!(bundle::decode(&encoded[..n], Limits::default()).is_err());
            }
            let mut damaged = encoded.clone();
            *damaged.last_mut().unwrap() ^= 1;
            assert!(bundle::decode(&damaged, Limits::default()).is_err());
            let mut trailing = encoded;
            trailing.push(0);
            assert!(bundle::decode(&trailing, Limits::default()).is_err());
        }
    }
}
#[test]
fn input_output_and_manifest_limits() {
    let input = vec![b'x'; 100000];
    for codec in codecs() {
        let data = codecs::compress(codec, &input, Limits::default()).unwrap();
        assert!(
            codecs::decompress(
                codec,
                &data,
                Limits {
                    input_bytes: 100000,
                    output_bytes: 32
                }
            )
            .is_err()
        );
        assert!(
            codecs::compress(
                codec,
                &input,
                Limits {
                    input_bytes: 32,
                    output_bytes: 100000
                }
            )
            .is_err()
        );
        assert!(
            codecs::compress(
                codec,
                &input,
                Limits {
                    input_bytes: 100000,
                    output_bytes: 1
                }
            )
            .is_err()
        );
        let encoded = bundle::encode(codec, &input, Limits::default()).unwrap();
        assert!(
            bundle::decode(
                &encoded,
                Limits {
                    input_bytes: 100000,
                    output_bytes: 32
                }
            )
            .is_err()
        );
        let mut unsupported = encoded;
        unsupported[8..12].copy_from_slice(&u32::MAX.to_be_bytes());
        assert!(bundle::decode(&unsupported, Limits::default()).is_err());
    }
}
struct Cancelled;
impl Read for Cancelled {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Interrupted, "cancelled"))
    }
}
#[test]
fn streaming_propagates_input_and_sink_failures() {
    for codec in codecs() {
        // Use a non-retryable error: std readers may intentionally retry Interrupted.
        struct Failed;
        impl Read for Failed {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::other("cancelled"))
            }
        }
        assert!(codecs::compress_to(codec, Failed, Vec::new(), Limits::default()).is_err());
        let limits = Limits {
            input_bytes: 1,
            output_bytes: 100000,
        };
        assert!(codecs::compress_to(codec, b"too long".as_slice(), Vec::new(), limits).is_err());
    }
    let _ = Cancelled;
}
