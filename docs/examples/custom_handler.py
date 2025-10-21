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
from typing import Callable

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

# [custom_channel]


class CustomChannel:
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


def create_custom_channel(
    max_size: int = 100,
) -> tuple[Callable[[zenoh.Sample], None], CustomChannel]:
    """Factory function that returns (callback, handler) pair"""
    channel = CustomChannel(max_size)

    def on_sample(sample: zenoh.Sample) -> None:
        # Store sample in the custom channel
        channel.add_sample(sample)

    return (on_sample, channel)


# [custom_channel]

# [custom_channel_usage]
subscriber = session.declare_subscriber("key/expression", create_custom_channel(max_size=50))
# Subscriber delegates to handler's recv() method via duck typing
sample = subscriber.recv()  # type: ignore[var-annotated]
print(f">> Received via subscriber.recv(): {sample.payload.to_string()}")

sample = subscriber.handler.try_recv()
if sample:
    print(f">> Received via handler.try_recv(): {sample.payload.to_string()}")

# Iteration also works (demonstrates __iter__ and __next__)
print(">> Reading remaining samples via iteration:")
count = 0
for sample in subscriber:
    print(f"   - {sample.payload.to_string()}")
    count += 1
    # Break after reading a few samples to avoid blocking
    if count >= 2:
        break

# Check statistics
print(f">> Total received: {subscriber.handler.received_count}")
# [custom_channel_usage]

# Verify
assert subscriber.handler.received_count >= 4
# We consumed 4 samples (1 via try_recv, 1 via recv, 2 via iteration)
# so should have 1 remaining
remaining = subscriber.handler.try_recv()
assert remaining is not None
print(f">> Remaining sample: {remaining.payload.to_string()}")

# Clean up
subscriber.undeclare()
session.close()
