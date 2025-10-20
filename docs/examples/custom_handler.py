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
    for i in range(5):
        session.put("key/expression", f"sample_{i}")
        time.sleep(0.05)


threading.Thread(target=send_data, daemon=True).start()

# [custom_handler]


class CustomHandler:
    def __init__(self, max_size=100):
        self.samples = []
        self.max_size = max_size
        self.received_count = 0

    def try_recv(self):
        """Non-blocking receive"""
        return self.samples.pop(0) if self.samples else None

    def recv(self):
        """Blocking receive"""
        while not self.samples:
            time.sleep(0.01)
        return self.samples.pop(0)

    def __iter__(self):
        return self

    def __next__(self):
        sample = self.recv()
        if sample is None:
            raise StopIteration
        return sample

    def add_sample(self, sample):
        """Called by the callback to store samples"""
        self.samples.append(sample)
        self.received_count += 1
        # Maintain max size
        if len(self.samples) > self.max_size:
            self.samples.pop(0)


def on_sample(sample):
    # Store sample in the custom handler
    my_handler.add_sample(sample)


my_handler = CustomHandler(max_size=50)
subscriber = session.declare_subscriber("key/expression", (on_sample, my_handler))
# [custom_handler]

# Wait for samples to arrive
time.sleep(0.5)

# [custom_handler_usage]
# Access handler directly (type-safe)
sample = subscriber.handler.try_recv()
if sample:
    print(f">> Received via handler.try_recv(): {sample.payload.to_string()}")

# Or call on subscriber (works at runtime, but type checker may complain)
sample = subscriber.recv()  # type: ignore
print(f">> Received via subscriber.recv(): {sample.payload.to_string()}")

# Check statistics
print(f">> Total received: {subscriber.handler.received_count}")
# [custom_handler_usage]

# Verify
assert my_handler.received_count >= 2
assert my_handler.try_recv() is not None

# Clean up
subscriber.undeclare()
session.close()
