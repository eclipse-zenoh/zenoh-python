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
import sys
from dataclasses import dataclass

import pytest

from zenoh import (
    Float32,
    Float64,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    ZBytes,
    deserializer,
    serializer,
)

default_serializer_tests = [
    (ZBytes, ZBytes.serialize(b"foo")),
    (bytes, b"foo"),
    (bytearray, bytearray(b"foo")),
    (str, "foo"),
    (int, -42),
    *((tp, tp(-42)) for tp in (Int8, Int16, Int32, Int64, Int128)),
    (int, 42),
    *((tp, tp(42)) for tp in (UInt8, UInt16, UInt32, UInt64, UInt128)),
    (float, 0.5),
    (Float64, Float64(0.5)),
    (Float32, Float32(0.5)),
    (bool, True),
    (list, [ZBytes.serialize(0), ZBytes.serialize(1)]),
    (tuple, (ZBytes.serialize(0), ZBytes.serialize(1))),
    (dict, {ZBytes.serialize("foo"): ZBytes.serialize("bar")}),
    (set, {ZBytes.serialize(0), ZBytes.serialize(1)}),
    (frozenset, frozenset([ZBytes.serialize(0), ZBytes.serialize(1)])),
]
if sys.version_info >= (3, 9):
    default_serializer_tests = [
        *default_serializer_tests,
        (list[int], [0, 1, 2]),
        (tuple[int, int], (0, 1)),
        (tuple[int, ...], (0, 1, 2)),
        (dict[str, str], {"foo": "bar"}),
        (set[int], {0, 1, 2}),
        (frozenset[int], frozenset([0, 1, 2])),
        (list[tuple[float, Float32]], [(0.0, Float32(0.5)), (1.5, Float32(2))]),
    ]


@pytest.mark.parametrize("tp, value", default_serializer_tests)
def test_default_serializer(tp, value):
    assert ZBytes.serialize(value).deserialize(tp) == value


def test_registered_serializer():
    @dataclass
    class Foo:
        bar: int

    @deserializer
    def deserialize_foo(zbytes: ZBytes) -> Foo:
        return Foo(zbytes.deserialize(int))

    @serializer
    def serialize_foo(foo: Foo) -> ZBytes:
        return ZBytes.serialize(foo.bar)

    foo = Foo(42)
    assert ZBytes.serialize(foo).deserialize(Foo) == foo


def test_registered_serializer_with_target():
    @dataclass
    class Foo:
        bar: int

    @deserializer(target=Foo)
    def deserialize_foo(zbytes):
        return Foo(zbytes.deserialize(int))

    @serializer(target=Foo)
    def serialize_foo(foo):
        return ZBytes.serialize(foo.bar)

    foo = Foo(42)
    assert ZBytes.serialize(foo).deserialize(Foo) == foo
