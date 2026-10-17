# Lossless compression — COMPRESS-001

Rust: enable the optional `compression` feature. `compression::codecs` provides
bounded reader/writer adapters for gzip (level 6), Brotli (quality 5/window 22) and
zstd (level 3/window 23/checksum). Default maximum input and output are 64 MiB;
callers can supply smaller limits. Rust streaming adapters propagate read/write
errors and use fixed copy buffers; they do not buffer a whole stream. Sink output
can be partial on failure: use a temporary sink before publishing it.

Python: install `miilo-adapter-sdk[compression]`, then import the explicit
`miilo_adapter_sdk.compression` module. It supports the same three codecs/bundle
format with bounded decoded output. Its byte-oriented APIs hold the input/result
in memory; they are not a substitute for the Rust streaming I/O adapter.

TypeScript exports codec-neutral DTOs and a host-adapter port, with no Node or
native dependency in the browser contract package. The Node CLI reuses the
explicit native-helper boundary for exact codec parity and older Node support.

## Bundle format 1

A single selected byte payload; no filenames, directories, secrets or filesystem
instructions are embedded. Bytes: ASCII `MIILOB1\n` (8 bytes), unsigned big-endian
u32 header length (maximum 4096), UTF-8 JSON manifest, then compressed payload.
Manifest fields: `formatVersion` (1), `codec` (`gzip`, `brotli`, `zstd`),
`uncompressedBytes`, `compressedBytes`, `sha256` and `compressedSha256`.
Both digests are lowercase SHA-256 hex strings. Encoded length/hash are checked
before decode; decoded length/hash are checked before restore publication.
Unsupported versions, invalid headers, trailing bytes, damaged/truncated payloads
and over-limit output fail. Bundle encode/decode are bounded in-memory helpers.
Use streaming codec adapters for larger controlled I/O; do not mistake the bundle
helpers for constant-memory archive operations.

Digests detect accidental corruption; they do not authenticate the sender.
Compression is not encryption, redaction, consent, or semantic compaction.
Do not compress secret-bearing HTTP responses together with reflected input.
Already compressed media and low-latency SSE/NDJSON are excluded from negotiated
HTTP compression. Request-body compression is not implicitly enabled.

## Validation

`make test` covers Rust and Python codec/bundle round trips, Unicode/binary/empty
payloads, corruption/truncation and limits. Consumer CLI tests prove atomic
no-clobber restore. Test real proxy/client latency before changing live stream
policy; gzip/Brotli/zstd ratios depend on corpus and compression settings.
