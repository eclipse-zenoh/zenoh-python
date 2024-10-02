try:
    from zenoh._ext import *
except ImportError:
    import warnings

    raise ModuleNotFoundError(
        "No module named 'zenoh.ext'.\nzenoh must be built wit zenoh-ext feature to enable it."
    ) from None

_INT8_MIN = -(1 << 7)
_INT16_MIN = -(1 << 15)
_INT32_MIN = -(1 << 31)
_INT64_MIN = -(1 << 63)
_INT128_MIN = -(1 << 127)

_INT8_MAX = 1 << 7
_INT16_MAX = 1 << 15
_INT32_MAX = 1 << 31
_INT64_MAX = 1 << 63
_INT128_MAX = 1 << 127

_UINT8_MAX = 1 << 8
_UINT16_MAX = 1 << 16
_UINT32_MAX = 1 << 32
_UINT64_MAX = 1 << 64
_UINT128_MAX = 1 << 128


class Int8(int):
    """int subclass enabling to (de)serialize 8bit signed integer."""

    def __new__(cls, i: int):
        assert _INT8_MIN <= i < _INT8_MAX, f"{i} too big for Int8"
        return int.__new__(cls, i)


class Int16(int):
    """int subclass enabling to (de)serialize 16bit signed integer."""

    def __new__(cls, i: int):
        assert _INT16_MIN <= i < _INT16_MAX, f"{i} too big for Int16"
        return int.__new__(cls, i)


class Int32(int):
    """int subclass enabling to (de)serialize 32bit signed integer."""

    def __new__(cls, i: int):
        assert _INT32_MIN <= i < _INT32_MAX, f"{i} too big for Int32"
        return int.__new__(cls, i)


class Int64(int):
    """int subclass enabling to (de)serialize 64bit signed integer."""

    def __new__(cls, i: int):
        assert _INT64_MIN <= i < _INT64_MAX, f"{i} too big for Int64"
        return int.__new__(cls, i)


class Int128(int):
    """int subclass enabling to (de)serialize 128bit signed integer."""

    def __new__(cls, i: int):
        assert _INT128_MIN <= i < _INT128_MAX, f"{i} too big for Int128"
        return int.__new__(cls, i)


class UInt8(int):
    """int subclass enabling to (de)serialize 8bit unsigned integer."""

    def __new__(cls, i: int):
        assert 0 <= i < _UINT8_MAX, f"{i} too big for UInt8"
        return int.__new__(cls, i)


class UInt16(int):
    """int subclass enabling to (de)serialize 16bit unsigned integer."""

    def __new__(cls, i: int):
        assert 0 <= i < _UINT16_MAX, f"{i} too big for UInt16"
        return int.__new__(cls, i)


class UInt32(int):
    """int subclass enabling to (de)serialize 32bit unsigned integer."""

    def __new__(cls, i: int):
        assert 0 <= i < _UINT32_MAX, f"{i} too big for UInt32"
        return int.__new__(cls, i)


class UInt64(int):
    """int subclass enabling to (de)serialize 64bit unsigned integer."""

    def __new__(cls, i: int):
        assert 0 <= i < _UINT64_MAX, f"{i} too big for UInt64"
        return int.__new__(cls, i)


class UInt128(int):
    """int subclass enabling to (de)serialize 128bit unsigned integer."""

    def __new__(cls, i: int):
        assert 0 <= i < _UINT128_MAX, f"{i} too big for UInt128"
        return int.__new__(cls, i)


class Float32(float):
    """float subclass enabling to (de)serialize 32bit floating point numbers."""


class Float64(float):
    """float subclass enabling to (de)serialize 64bit floating point numbers."""
