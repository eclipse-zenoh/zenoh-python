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

# Open session
session = zenoh.open(zenoh.Config())

# Test support: send data in background


def send_data():
    time.sleep(0.1)
    for i in range(3):
        session.put("key/expr", f"sample_{i}")


threading.Thread(target=send_data, daemon=True).start()

# [channels_fifo]
# Explicit FIFO channel with custom capacity
subscriber = session.declare_subscriber("key/expr", zenoh.handlers.FifoChannel(100))
sample = subscriber.try_recv()
# [channels_fifo]
if sample:
    print(f">> Received: {sample.payload.to_string()}")
else:
    print(">> No sample available yet")

# Wait for more data and try again
time.sleep(0.2)
sample = subscriber.recv()
print(f">> Received via recv(): {sample.payload.to_string()}")

# Clean up
subscriber.undeclare()
session.close()
