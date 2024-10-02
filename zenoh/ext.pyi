from typing import Any, TypeVar

from zenoh import ZBytes

_T = TypeVar("_T")

class Int8(int):
    """int subclass enabling to (de)serialize 8bit signed integer."""

class Int16(int):
    """int subclass enabling to (de)serialize 16bit signed integer."""

class Int32(int):
    """int subclass enabling to (de)serialize 32bit signed integer."""

class Int64(int):
    """int subclass enabling to (de)serialize 64bit signed integer."""

class Int128(int):
    """int subclass enabling to (de)serialize 128bit signed integer."""

class UInt8(int):
    """int subclass enabling to (de)serialize 8bit unsigned integer."""

class UInt16(int):
    """int subclass enabling to (de)serialize 16bit unsigned integer."""

class UInt32(int):
    """int subclass enabling to (de)serialize 32bit unsigned integer."""

class UInt64(int):
    """int subclass enabling to (de)serialize 64bit unsigned integer."""

class UInt128(int):
    """int subclass enabling to (de)serialize 128bit unsigned integer."""

class Float32(float):
    """float subclass enabling to (de)serialize 32bit floating point numbers."""

class Float64(float):
    """float subclass enabling to (de)serialize 64bit floating point numbers."""

class ZDeserializeError(Exception):
    pass

def z_serialize(obj: Any) -> ZBytes: ...
def z_deserialize(tp: type[_T], zbytes: ZBytes) -> _T: ...
