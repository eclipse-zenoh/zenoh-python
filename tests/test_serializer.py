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

from zenoh import ZBytes, deserializer, serializer

default_serializer_tests = [
    (bytes, b"foo"),
    (str, "foo"),
    (int, 42),
    (float, 0.5),
    (bool, True),
    (ZBytes, ZBytes.serialize(b"foo")),
    (list, [ZBytes.serialize(0), ZBytes.serialize(1)]),
    (dict, {ZBytes.serialize("foo"): ZBytes.serialize("bar")}),
]
if sys.version_info >= (3, 9):
    default_serializer_tests = [
        *default_serializer_tests,
        (list[int], [0, 1, 2]),
        (dict[str, str], {"foo": "bar"}),
        (tuple[int, int], (0, 1)),
        (list[tuple[int, int]], [(0, 1), (2, 3)]),
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
