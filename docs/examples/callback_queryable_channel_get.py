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

global query_received
query_received = False
reply_received = False

session = zenoh.open(zenoh.Config())

# [queryable_callback]
def query_handler(query):
    print(f"Received query: {query.key_expr}")
    query.reply("room/temperature", zenoh.ZBytes("Temperature: 23.5°C"))
# [queryable_callback]
    global query_received
    query_received = True

# [declare_queryable]
queryable = session.declare_queryable("room/temperature", query_handler)
# [declare_queryable]

# Wait for processing
time.sleep(0.5)

# [get_channel]
replies = session.get("room/temperature")
for reply in replies:
    if reply.ok:
        print(f"Received reply: {reply.ok.payload.to_string()}")
# [get_channel]
        reply_received = True
        break

# Verify
assert query_received
assert reply_received

# Clean up
queryable.undeclare()
session.close()
