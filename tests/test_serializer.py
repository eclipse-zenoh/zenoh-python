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

from zenoh import ZBytes
from zenoh.ext import (
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
    z_deserialize,
    z_serialize,
)

default_serializer_tests = [
    (ZBytes, ZBytes(b"foo")),
    (bytes, b"foo"),
    (bytearray, bytearray(b"foo")),
    (str, "foo"),
    *(
        (tp, tp(i))
        for i in (-42, 42)
        for tp in (int, Int8, Int16, Int32, Int64, Int128)
    ),
    *((tp, tp(42)) for tp in (UInt8, UInt16, UInt32, UInt64, UInt128)),
    (float, 0.5),
    (Float64, Float64(0.5)),
    (Float32, Float32(0.5)),
    (bool, True),
    (list[int], [0, 1, 2]),
    (tuple[int, int], (0, 1)),
    (dict[str, str], {"foo": "bar"}),
    (set[int], {0, 1, 2}),
    (frozenset[int], frozenset([0, 1, 2])),
    (list[tuple[float, Float32]], [(0.0, Float32(0.5)), (1.5, Float32(2))]),
]


@pytest.mark.parametrize("tp, value", default_serializer_tests)
def test_default_serializer(tp, value):
    zbytes = z_serialize(value)
    assert z_deserialize(tp, zbytes) == value
