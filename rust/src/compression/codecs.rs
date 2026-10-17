//! Synchronous streaming I/O adapter. Reader/writer errors propagate cancellation.
use super::{Codec, Limits};
use std::io::{self, Read, Write};

pub fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn copy_bounded(mut input: impl Read, mut output: impl Write, max: u64) -> io::Result<u64> {
    let mut total = 0u64;
    let mut buffer = [0u8; 16 * 1024];
    loop {
        let read = input.read(&mut buffer)?;
        if read == 0 {
            return Ok(total);
        }
        total = total
            .checked_add(read as u64)
            .ok_or_else(|| invalid("Size overflow"))?;
        if total > max {
            return Err(invalid("Compression byte limit exceeded"));
        }
        output.write_all(&buffer[..read])?;
    }
}
struct BoundedRead<R> {
    inner: R,
    left: u64,
}
impl<R: Read> Read for BoundedRead<R> {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        let size = bytes
            .len()
            .min(self.left.saturating_add(1).min(usize::MAX as u64) as usize);
        let n = self.inner.read(&mut bytes[..size])?;
        if n as u64 > self.left {
            return Err(invalid("Compression input limit exceeded"));
        }
        self.left -= n as u64;
        Ok(n)
    }
}
struct Bounded<W> {
    inner: W,
    left: u64,
}
impl<W: Write> Write for Bounded<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() as u64 > self.left {
            return Err(invalid("Compression byte limit exceeded"));
        }
        let n = self.inner.write(bytes)?;
        self.left -= n as u64;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
pub fn compress_to(
    codec: Codec,
    input: impl Read,
    output: impl Write,
    limits: Limits,
) -> io::Result<()> {
    let input = BoundedRead {
        inner: input,
        left: limits.input_bytes,
    };
    let output = Bounded {
        inner: output,
        left: limits.output_bytes,
    };
    match codec {
        Codec::Gzip => {
            let mut encoder = flate2::write::GzEncoder::new(output, flate2::Compression::new(6));
            copy_bounded(input, &mut encoder, limits.input_bytes)?;
            encoder.finish()?;
        }
        Codec::Brotli => {
            let encoder = brotli::CompressorReader::new(input, 16 * 1024, 5, 22);
            copy_bounded(encoder, output, limits.output_bytes)?;
        }
        Codec::Zstd => {
            let mut encoder = zstd::stream::write::Encoder::new(output, 3)?;
            encoder.include_checksum(true)?;
            encoder.window_log(23)?;
            copy_bounded(input, &mut encoder, limits.input_bytes)?;
            encoder.finish()?;
        }
    }
    Ok(())
}
pub fn decompress_to(
    codec: Codec,
    input: impl Read,
    output: impl Write,
    limits: Limits,
) -> io::Result<()> {
    let bounded = BoundedRead {
        inner: input,
        left: limits.input_bytes,
    };
    match codec {
        Codec::Gzip => {
            copy_bounded(
                flate2::read::MultiGzDecoder::new(bounded),
                output,
                limits.output_bytes,
            )?;
        }
        Codec::Brotli => {
            copy_bounded(
                brotli::Decompressor::new(bounded, 16 * 1024),
                output,
                limits.output_bytes,
            )?;
        }
        Codec::Zstd => {
            let mut decoder = zstd::stream::read::Decoder::new(bounded)?;
            decoder.window_log_max(23)?;
            copy_bounded(decoder, output, limits.output_bytes)?;
        }
    }
    Ok(())
}
pub fn compress(codec: Codec, input: &[u8], limits: Limits) -> io::Result<Vec<u8>> {
    if input.len() as u64 > limits.input_bytes {
        return Err(invalid("Input limit exceeded"));
    }
    let mut output = Vec::new();
    compress_to(codec, input, &mut output, limits)?;
    Ok(output)
}
pub fn decompress(codec: Codec, input: &[u8], limits: Limits) -> io::Result<Vec<u8>> {
    if input.len() as u64 > limits.input_bytes {
        return Err(invalid("Input limit exceeded"));
    }
    let mut output = Vec::new();
    decompress_to(codec, input, &mut output, limits)?;
    Ok(output)
}
