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
from zenoh import ZBytes


def main():
    # Raw bytes
    input = b"raw bytes"
    payload = ZBytes(input)
    output = payload.to_bytes()  # equivalent to `bytes(payload)`
    assert input == output
    # Corresponding encoding to be used in operations like `.put()`, `.reply()`, etc.
    # encoding = Encoding.ZENOH_BYTES;

    # Raw utf8 bytes, i.e. string
    input = "raw bytes"
    payload = ZBytes(input)
    output = payload.to_string()  # equivalent to `str(payload)`
    assert input == output
    # Corresponding encoding to be used in operations like `.put()`, `.reply()`, etc.
    # encoding = Encoding.ZENOH_STRING;

    # JSON
    import json

    input = {"name": "John Doe", "age": 43, "phones": ["+44 1234567", "+44 2345678"]}
    payload = ZBytes(json.dumps(input))
    output = json.loads(payload.to_string())
    assert input == output
    # Corresponding encoding to be used in operations like `.put()`, `.reply()`, etc.
    # encoding = Encoding.APPLICATION_JSON;

    # Protobuf
    try:
        import entity_pb2

        input = entity_pb2.Entity(id=1234, name="John Doe")
        payload = ZBytes(input.SerializeToString())
        output = entity_pb2.Entity()
        output.ParseFromString(payload.to_bytes())
        assert input == output
        # Corresponding encoding to be used in operations like `.put()`, `.reply()`, etc.
        # encoding = Encoding.APPLICATION_PROTOBUF;
    except ImportError:
        # You must install protobuf and generate the protobuf classes from the schema with
        # $ pip install protobuf
        # $ protoc --python_out=. --pyi_out=. examples/entity.proto
        pass

    # zenoh.ext serialization
    from zenoh.ext import UInt32, z_deserialize, z_serialize

    if True:
        # Numeric: UInt8, UInt16, Uint32, UInt64, UInt128, Int8, Int16, Int32, Int64,
        # Int128, int (handled as int32), Float32, Float64, float (handled as Float64)
        input = UInt32(1234)
        payload = z_serialize(input)
        output = z_deserialize(UInt32, payload)
        assert input == output

        # list
        input = [0.0, 1.5, 42.0]  # all items must have the same type
        payload = z_serialize(input)
        output = z_deserialize(list[float], payload)
        assert input == output

        # dict
        input = {0: "abc", 1: "def"}
        payload = z_serialize(input)
        output = z_deserialize(dict[int, str], payload)
        assert input == output

        # tuple
        input = (0.42, "string")
        payload = z_serialize(input)
        output = z_deserialize(tuple[float, str], payload)
        assert input == output


if __name__ == "__main__":
    main()
