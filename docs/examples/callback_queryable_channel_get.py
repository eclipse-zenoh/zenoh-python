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
import threading
import time

import zenoh

replies_received = []

# DOC_EXAMPLE_START
# Example 1: Queryable with callback, Get with channel

# Queryable with callback - handler is called for each query
def query_handler(query):
    print(f"Received query: {query.key_expr}")
    query.reply(query.key_expr, zenoh.ZBytes("Temperature: 23.5°C"))


session = zenoh.open(zenoh.Config())
queryable = session.declare_queryable("room/temperature", query_handler)

# Get with channel (default) - iterate over replies
replies = session.get("room/temperature")
for reply in replies:
    if reply.ok:
        print(f"Received reply: {reply.ok.payload.to_string()}")
        replies_received.append(reply.ok.payload.to_string())  # For verification
        break
# DOC_EXAMPLE_END

# Verify
assert len(replies_received) == 1
assert "Temperature" in replies_received[0]

# Clean up
queryable.undeclare()
session.close()
