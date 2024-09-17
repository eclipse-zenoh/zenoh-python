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

from zenoh import ZBytes, deserializer, serializer


def test_override_serializer():
    assert ZBytes.serialize(42) != ZBytes.serialize("42")

    @deserializer
    def deserialize_int_from_str(zbytes: ZBytes) -> int:
        return int(zbytes.deserialize(str))

    @serializer
    def serialize_int_as_str(foo: int) -> ZBytes:
        return ZBytes.serialize(str(foo))

    assert ZBytes.serialize(42).deserialize(int) == 42
    assert ZBytes.serialize(42) == ZBytes.serialize("42")
