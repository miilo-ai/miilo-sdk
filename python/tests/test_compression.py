import pytest
import brotli
import zstandard
from miilo_adapter_sdk.compression import Limits, compress, decompress, encode_bundle, decode_bundle

@pytest.mark.parametrize("codec", ["gzip", "brotli", "zstd"])
@pytest.mark.parametrize("data", [b"", "café 🦉\n".encode()*1000, bytes(range(256))*500], ids=["empty", "unicode", "binary"])
def test_exact_bundles_and_fail_closed(codec, data):
    packed = encode_bundle(codec, data)
    assert decode_bundle(packed)[1] == data
    for broken in [packed[:-1], packed + b"x", packed[:12], packed[:-1] + bytes([packed[-1] ^ 1])]:
        with pytest.raises((ValueError, OSError)):
            decode_bundle(broken)

@pytest.mark.parametrize("codec", ["gzip", "brotli", "zstd"])
def test_output_bomb_and_input_limits(codec):
    packed = compress(codec, b"x"*100000)
    with pytest.raises((ValueError, OSError, brotli.error, zstandard.ZstdError)):
        decompress(codec, packed, Limits(output_bytes=16))
    with pytest.raises(ValueError):
        compress(codec, b"xx", Limits(input_bytes=1))
