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

# [raw_data]
# Raw bytes
payload = zenoh.ZBytes(b"Hello, World!")
data = payload.to_bytes()
assert isinstance(data, bytes)
assert data == b"Hello, World!"

# String data
payload = zenoh.ZBytes("Hello, World!")
text = payload.to_string()
assert isinstance(text, str)
assert text == "Hello, World!"
# [raw_data]

# [serialized_data]
# Using zenoh.ext for serialization
from zenoh.ext import z_deserialize, z_serialize

# Serialize a dictionary
data = {"temperature": 25.5, "humidity": 60.0}
payload = z_serialize(data)
assert isinstance(payload, zenoh.ZBytes)

# Deserialize back
received = z_deserialize(dict[str, float], payload)
assert isinstance(received, dict)
assert received == {"temperature": 25.5, "humidity": 60.0}
# [serialized_data]
