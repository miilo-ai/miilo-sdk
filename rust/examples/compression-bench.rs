//! Synthetic inputs only. Emit repeated raw measurements, not fabricated savings.
use miilo_adapter_sdk::compression::{Codec, Limits, bundle, codecs};
use std::time::Instant;
fn main() {
    let corpus = std::env::args().nth(1).expect("input path");
    let input = std::fs::read(corpus).unwrap();
    for codec in [Codec::Gzip, Codec::Brotli, Codec::Zstd] {
        for run in 0..7 {
            let t = Instant::now();
            let encoded = codecs::compress(codec, &input, Limits::default()).unwrap();
            let encode = t.elapsed();
            let t = Instant::now();
            let decoded = codecs::decompress(codec, &encoded, Limits::default()).unwrap();
            let decode = t.elapsed();
            assert_eq!(decoded, input);
            let archive = bundle::encode(codec, &input, Limits::default()).unwrap();
            println!(
                "{}",
                serde_json::json!({"codec":codec,"run":run,"rawBytes":input.len(),"compressedBytes":encoded.len(),"bundleBytes":archive.len(),"encodeMicros":encode.as_micros(),"decodeMicros":decode.as_micros(),"sha256":bundle::hash(&input)})
            );
        }
    }
}
