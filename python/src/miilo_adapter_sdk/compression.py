"""Optional lossless I/O adapter. Install miilo-adapter-sdk[compression]."""
import gzip
import hashlib
import io
import json
import struct
from dataclasses import dataclass
from typing import Literal
import brotli
import zstandard

Codec = Literal["gzip", "brotli", "zstd"]
MAGIC = b"MIILOB1\n"
DEFAULT_LIMIT = 64 * 1024 * 1024

@dataclass(frozen=True)
class Limits:
    input_bytes: int = DEFAULT_LIMIT
    output_bytes: int = DEFAULT_LIMIT


def _hash(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def compress(codec: Codec, data: bytes, limits: Limits = Limits()) -> bytes:
    if len(data) > limits.input_bytes:
        raise ValueError("Input limit exceeded")
    if codec == "gzip":
        result = gzip.compress(data, compresslevel=6, mtime=0)
    elif codec == "brotli":
        result = brotli.compress(data, quality=5, lgwin=22)
    elif codec == "zstd":
        result = zstandard.ZstdCompressor(level=3, write_checksum=True).compress(data)
    else:
        raise ValueError("Unsupported codec")
    if len(result) > limits.output_bytes:
        raise ValueError("Output limit exceeded")
    return result


def decompress(codec: Codec, data: bytes, limits: Limits = Limits()) -> bytes:
    if len(data) > limits.input_bytes:
        raise ValueError("Input limit exceeded")
    if codec == "gzip":
        with gzip.GzipFile(fileobj=io.BytesIO(data)) as reader:
            result = reader.read(limits.output_bytes + 1)
    elif codec == "brotli":
        decoder = brotli.Decompressor()
        result = bytearray()
        chunk = data
        while True:
            decoded = decoder.process(chunk, output_buffer_limit=min(16384, limits.output_bytes - len(result) + 1))
            result.extend(decoded)
            if len(result) > limits.output_bytes:
                raise ValueError("Output limit exceeded")
            if decoder.is_finished():
                break
            if not decoded:
                raise ValueError("Truncated Brotli frame")
            chunk = b""
        result = bytes(result)
    elif codec == "zstd":
        size = zstandard.frame_content_size(data)
        if size not in (zstandard.CONTENTSIZE_UNKNOWN, zstandard.CONTENTSIZE_ERROR) and size > limits.output_bytes:
            raise ValueError("Output limit exceeded")
        result = zstandard.ZstdDecompressor(max_window_size=8192).decompress(
            data, max_output_size=max(limits.output_bytes, 1), allow_extra_data=False)
    else:
        raise ValueError("Unsupported codec")
    if len(result) > limits.output_bytes:
        raise ValueError("Output limit exceeded")
    return result


def encode_bundle(codec: Codec, data: bytes, limits: Limits = Limits()) -> bytes:
    payload = compress(codec, data, limits)
    manifest = {"formatVersion": 1, "codec": codec, "uncompressedBytes": len(data),
                "compressedBytes": len(payload), "sha256": _hash(data), "compressedSha256": _hash(payload)}
    header = json.dumps(manifest, separators=(",", ":")).encode()
    return MAGIC + struct.pack(">I", len(header)) + header + payload


def inspect_bundle(data: bytes, limits: Limits = Limits()) -> tuple[dict, bytes]:
    if len(data) < 12 or data[:8] != MAGIC:
        raise ValueError("Not a Miilo bundle")
    size = struct.unpack(">I", data[8:12])[0]
    if size > 4096 or len(data) < 12 + size:
        raise ValueError("Invalid bundle header length")
    manifest = json.loads(data[12:12+size])
    keys = {"formatVersion", "codec", "uncompressedBytes", "compressedBytes", "sha256", "compressedSha256"}
    if not isinstance(manifest, dict) or set(manifest) != keys:
        raise ValueError("Invalid manifest")
    payload = data[12+size:]
    if (type(manifest["formatVersion"]) is not int or manifest["formatVersion"] != 1
        or manifest["codec"] not in ("gzip", "brotli", "zstd")
        or type(manifest["uncompressedBytes"]) is not int or not 0 <= manifest["uncompressedBytes"] <= limits.output_bytes
        or type(manifest["compressedBytes"]) is not int or manifest["compressedBytes"] != len(payload)
        or len(payload) > limits.input_bytes or not isinstance(manifest["sha256"], str)
        or len(manifest["sha256"]) != 64 or manifest["compressedSha256"] != _hash(payload)):
        raise ValueError("Bundle integrity, version or byte limit check failed")
    return manifest, payload


def decode_bundle(data: bytes, limits: Limits = Limits()) -> tuple[dict, bytes]:
    manifest, payload = inspect_bundle(data, limits)
    restored = decompress(manifest["codec"], payload, Limits(limits.input_bytes, manifest["uncompressedBytes"]))
    if len(restored) != manifest["uncompressedBytes"] or _hash(restored) != manifest["sha256"]:
        raise ValueError("Restored payload integrity check failed")
    return manifest, restored
