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
import json

from zenoh import UInt32, ZBytes


def main():
    # Numeric: UInt8, UInt16, Uint32, UInt64, UInt128, Int8, Int16, Int32, Int64,
    # Int128, Float32, Float64, int (handled as Int64), float (handled as Float64)
    input = UInt32(1234)
    payload = ZBytes.serialize(input)
    output = payload.deserialize(UInt32)
    assert input == output
    # Corresponding encoding to be used in operations like `.put()`, `.reply()`, etc.
    # encoding = Encoding.ZENOH_UINT32;

    # str
    input = "test"
    payload = ZBytes.serialize(input)
    output = payload.deserialize(str)
    assert input == output
    # Corresponding encoding to be used in operations like `.put()`, `.reply()`, etc.
    # encoding = Encoding.ZENOH_STRING;

    # bytes, bytearray
    input = b"test"
    payload = ZBytes.serialize(input)
    output = payload.deserialize(bytes)
    assert input == output
    # Corresponding encoding to be used in operations like `.put()`, `.reply()`, etc.
    # encoding = Encoding.ZENOH_STRING;

    # tuple
    input = 1234, "test"
    payload = ZBytes.serialize(input)
    output = payload.deserialize(tuple[int, str])
    assert input == output

    # list
    input = [1, 2, 3, 4]
    payload = ZBytes.serialize(input)
    output = payload.deserialize(list[int])
    assert input == output

    # dict
    input = {0: "abc", 1: "def"}
    payload = ZBytes.serialize(input)
    output = payload.deserialize(dict[int, str])
    assert input == output

    # JSON
    input = {"name": "John Doe", "age": 43, "phones": ["+44 1234567", "+44 2345678"]}
    payload = ZBytes.serialize(json.dumps(input))
    output = json.loads(payload.deserialize(str))
    assert input == output
    # Corresponding encoding to be used in operations like `.put()`, `.reply()`, etc.
    # encoding = Encoding.APPLICATION_JSON;

    # Other formats like protobuf can be used the same way as JSON, i.e. dumps to bytes/str before serializing to ZBytes, and loads from ZBytes deserialized to str/bytes.


if __name__ == "__main__":
    main()
