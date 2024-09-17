#
# Copyright (c) 2024 ZettaScale Technology
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
#
from .zenoh import *

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
    def __new__(cls, i: int):
        assert _INT8_MIN <= i < _INT8_MAX, f"{i} too big for Int8"
        return int.__new__(cls, i)


class Int16(int):
    def __new__(cls, i: int):
        assert _INT16_MIN <= i < _INT16_MAX, f"{i} too big for Int16"
        return int.__new__(cls, i)


class Int32(int):
    def __new__(cls, i: int):
        assert _INT32_MIN <= i < _INT32_MAX, f"{i} too big for Int32"
        return int.__new__(cls, i)


class Int64(int):
    def __new__(cls, i: int):
        assert _INT64_MIN <= i < _INT64_MAX, f"{i} too big for Int64"
        return int.__new__(cls, i)


class Int128(int):
    def __new__(cls, i: int):
        assert _INT128_MIN <= i < _INT128_MAX, f"{i} too big for Int128"
        return int.__new__(cls, i)


class UInt8(int):
    def __new__(cls, i: int):
        assert 0 <= i < _UINT8_MAX, f"{i} too big for UInt8"
        return int.__new__(cls, i)


class UInt16(int):
    def __new__(cls, i: int):
        assert 0 <= i < _UINT16_MAX, f"{i} too big for UInt16"
        return int.__new__(cls, i)


class UInt32(int):
    def __new__(cls, i: int):
        assert 0 <= i < _UINT32_MAX, f"{i} too big for UInt32"
        return int.__new__(cls, i)


class UInt64(int):
    def __new__(cls, i: int):
        assert 0 <= i < _UINT64_MAX, f"{i} too big for UInt64"
        return int.__new__(cls, i)


class UInt128(int):
    def __new__(cls, i: int):
        assert 0 <= i < _UINT128_MAX, f"{i} too big for UInt128"
        return int.__new__(cls, i)


class Float32(float):
    pass


class Float64(float):
    pass
