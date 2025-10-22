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
    # Send more samples than ring capacity to test dropping behavior
    for i in range(55):
        session.put("key/expr", f"sample_{i}")


threading.Thread(target=send_data, daemon=True).start()

# [channels_ring]
# Ring channel (drops oldest when full)
subscriber = session.declare_subscriber("key/expr", zenoh.handlers.RingChannel(50))
# [channels_ring]

# Wait for samples to arrive
time.sleep(0.3)

# Read a few samples to verify it works
count = 0
sample = subscriber.try_recv()
while sample and count < 10:
    print(f">> Received: {sample.payload.to_string()}")
    sample = subscriber.try_recv()
    count += 1

print(f">> Successfully received {count} samples")

# Clean up
subscriber.undeclare()
session.close()
