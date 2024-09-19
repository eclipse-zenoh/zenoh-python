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
import zenoh
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
    import json

    input = {"name": "John Doe", "age": 43, "phones": ["+44 1234567", "+44 2345678"]}
    payload = ZBytes.serialize(json.dumps(input))
    output = json.loads(payload.deserialize(str))
    assert input == output
    # Corresponding encoding to be used in operations like `.put()`, `.reply()`, etc.
    # encoding = Encoding.APPLICATION_JSON;

    # Protobuf
    try:
        import entity_pb2

        input = entity_pb2.Entity(id=1234, name="John Doe")
        payload = ZBytes.serialize(input.SerializeToString())
        output = entity_pb2.Entity()
        output.ParseFromString(payload.deserialize(bytes))
        assert input == output
        # Corresponding encoding to be used in operations like `.put()`, `.reply()`, etc.
        # encoding = Encoding.APPLICATION_PROTOBUF;
    except ImportError:
        # You must install protobuf and generate the protobuf classes from the schema with
        # $ pip install protobuf
        # $ protoc --python_out=. --pyi_out=. examples/entity.proto
        pass

    # arbitrary type
    import struct
    from dataclasses import dataclass

    @dataclass
    class Coordinates:
        x: float
        y: float
        z: float

    @zenoh.serializer  # input type is retrieved from serializer signature
    def serialize_coordinates(c: Coordinates) -> ZBytes:
        return ZBytes(struct.pack("<fff", c.x, c.y, c.z))

    @zenoh.deserializer  # output type is retrieved from deserializer signature
    def deserialize_coordinates(zbytes: ZBytes) -> Coordinates:
        return Coordinates(*struct.unpack("<fff", bytes(zbytes)))

    input = Coordinates(42, 1.5, 0)
    payload = ZBytes.serialize(input)
    output = payload.deserialize(Coordinates)
    assert input == output


if __name__ == "__main__":
    main()
