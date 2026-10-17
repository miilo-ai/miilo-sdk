//! Versioned single-payload bundle adapter; never extracts paths or executes content.
use super::{
    Codec, Limits, Manifest,
    codecs::{compress, decompress, invalid},
};
use sha2::{Digest, Sha256};
use std::io;
pub const MAGIC: &[u8; 8] = b"MIILOB1\n";
pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn encode(codec: Codec, bytes: &[u8], limits: Limits) -> io::Result<Vec<u8>> {
    let compressed = compress(codec, bytes, limits)?;
    let manifest = Manifest {
        format_version: 1,
        codec,
        uncompressed_bytes: bytes.len() as u64,
        compressed_bytes: compressed.len() as u64,
        sha256: hash(bytes),
        compressed_sha256: hash(&compressed),
    };
    let header = serde_json::to_vec(&manifest)?;
    let mut bundle = MAGIC.to_vec();
    bundle.extend_from_slice(&(header.len() as u32).to_be_bytes());
    bundle.extend(header);
    bundle.extend(compressed);
    Ok(bundle)
}
pub fn inspect(bytes: &[u8], limits: Limits) -> io::Result<(Manifest, &[u8])> {
    if bytes.len() < 12 || &bytes[..8] != MAGIC {
        return Err(invalid("Not a Miilo bundle"));
    }
    let n = u32::from_be_bytes(
        bytes[8..12]
            .try_into()
            .map_err(|_| invalid("Invalid header"))?,
    ) as usize;
    if n > 4096 || bytes.len() < 12 + n {
        return Err(invalid("Invalid bundle header length"));
    }
    let manifest: Manifest = serde_json::from_slice(&bytes[12..12 + n])?;
    let payload = &bytes[12 + n..];
    if manifest.format_version != 1
        || manifest.compressed_bytes != payload.len() as u64
        || manifest.compressed_bytes > limits.input_bytes
        || manifest.uncompressed_bytes > limits.output_bytes
        || manifest.sha256.len() != 64
        || manifest.compressed_sha256 != hash(payload)
    {
        return Err(invalid(
            "Bundle integrity, version or byte limit check failed",
        ));
    }
    Ok((manifest, payload))
}
pub fn decode(bytes: &[u8], limits: Limits) -> io::Result<(Manifest, Vec<u8>)> {
    let (manifest, payload) = inspect(bytes, limits)?;
    let output = decompress(
        manifest.codec,
        payload,
        Limits {
            input_bytes: limits.input_bytes,
            output_bytes: manifest.uncompressed_bytes.min(limits.output_bytes),
        },
    )?;
    if output.len() as u64 != manifest.uncompressed_bytes || hash(&output) != manifest.sha256 {
        return Err(invalid("Restored payload integrity check failed"));
    }
    Ok((manifest, output))
}
