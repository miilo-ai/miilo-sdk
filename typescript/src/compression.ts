/** Codec-neutral artifact contracts; codec implementations belong in host adapters. */
export type CompressionCodec = "gzip" | "brotli" | "zstd";
export interface CompressionLimits { inputBytes: number; outputBytes: number }
export interface BundleManifest {
  formatVersion: 1;
  codec: CompressionCodec;
  uncompressedBytes: number;
  compressedBytes: number;
  sha256: string;
  compressedSha256: string;
}
export interface LosslessCodecPort {
  compress(codec: CompressionCodec, data: Uint8Array, limits: CompressionLimits): Promise<Uint8Array>;
  decompress(codec: CompressionCodec, data: Uint8Array, limits: CompressionLimits): Promise<Uint8Array>;
}
