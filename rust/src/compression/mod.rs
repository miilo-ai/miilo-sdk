//! Portable lossless artifact contracts. Codecs are optional adapter implementations.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Codec {
    Gzip,
    Brotli,
    Zstd,
}
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub input_bytes: u64,
    pub output_bytes: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            input_bytes: 64 * 1024 * 1024,
            output_bytes: 64 * 1024 * 1024,
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Manifest {
    pub format_version: u32,
    pub codec: Codec,
    pub uncompressed_bytes: u64,
    pub compressed_bytes: u64,
    pub sha256: String,
    pub compressed_sha256: String,
}
#[cfg(feature = "compression")]
pub mod bundle;
#[cfg(feature = "compression")]
pub mod codecs;
