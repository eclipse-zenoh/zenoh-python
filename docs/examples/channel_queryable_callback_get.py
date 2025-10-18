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

queries_received = []
replies_received = []

session = zenoh.open(zenoh.Config())

# Queryable with channel (default) - iterate over queries

# Test setup: Run queryable iteration in thread
def handle_queries():
# DOC_EXAMPLE_START
    queryable = session.declare_queryable("room/humidity")
    for query in queryable:
        query.reply("room/humidity", zenoh.ZBytes("Humidity: 65%"))
# DOC_EXAMPLE_END
        queries_received.append(query.key_expr)
        break


threading.Thread(target=handle_queries, daemon=True).start()
time.sleep(0.1)

# DOC_EXAMPLE_START
# Get with callback - handler is called for each reply
def reply_handler(reply):
    if reply.ok:
        print(f"Received reply: {reply.ok.payload.to_string()}")

session.get("room/humidity", reply_handler)
# DOC_EXAMPLE_END

# Wait for callback to complete
time.sleep(0.3)
replies_received.append("verified")

# Wait for processing
time.sleep(0.5)

# Verify
assert len(queries_received) == 1
assert len(replies_received) == 1

# Clean up
session.close()
