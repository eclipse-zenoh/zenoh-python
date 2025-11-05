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
from collections.abc import Callable
from typing import Generic, TypeVar

import zenoh

_T = TypeVar("_T")

# Open session
session = zenoh.open(zenoh.Config())


# Test support: send data in background
def send_data():
    time.sleep(1)
    for i in range(5):
        session.put("key/expression", f"sample_{i}")


threading.Thread(target=send_data, daemon=True).start()


# [custom_channel]
class CustomChannel(Generic[_T]):
    def __init__(self, max_size=100):
        self.samples: list[_T] = []
        self.max_size = max_size
        self.condition = threading.Condition()

    def try_recv(self) -> _T | None:
        """Non-blocking receive"""
        with self.condition:
            return self.samples.pop(0) if self.samples else None

    def recv(self) -> _T:
        """Blocking receive"""
        with self.condition:
            while not self.samples:
                self.condition.wait()
            return self.samples.pop(0)

    def __iter__(self):
        return self

    def __next__(self) -> _T:
        sample = self.recv()
        if sample is None:
            raise StopIteration
        return sample

    def send(self, sample: _T):
        """Called by the callback to store samples"""
        with self.condition:
            self.samples.append(sample)
            # Maintain max size
            if len(self.samples) > self.max_size:
                self.samples.pop(0)
            # Notify one waiting thread that a sample is available
            self.condition.notify()

    def count(self) -> int:
        """Return number of stored samples"""
        with self.condition:
            return len(self.samples)


def create_custom_channel(
    max_size: int = 100,
) -> tuple[Callable[[zenoh.Sample], None], CustomChannel[zenoh.Sample]]:
    """Factory function that returns (callback, handler) pair"""
    channel: CustomChannel[zenoh.Sample] = CustomChannel(max_size)

    def on_sample(sample: zenoh.Sample) -> None:
        # Store sample in the custom channel
        channel.send(sample)

    return (on_sample, channel)


# [custom_channel]

count = 0
# [custom_channel_usage]
subscriber = session.declare_subscriber(
    "key/expression", create_custom_channel(max_size=50)
)

# Subscriber delegates to handler's recv() and try_recv() methods via duck typing
# but it's recommended to access them via handler attribute for type safety
sample = subscriber.recv()  # type: ignore[misc]
print(f">> Received via recv(): {sample.payload.to_string()}")
time.sleep(0.1)  # Give some time for more samples to arrive
sample = subscriber.handler.try_recv()
if sample:
    print(f">> Received via try_recv(): {sample.payload.to_string()}")

# Access to custom channel methods via handler
print(f">> Samples currently stored in channel: {subscriber.handler.count()}")

# Iteration also works (demonstrates __iter__ and __next__)
print(">> Reading remaining samples via iteration:")
for sample in subscriber.handler:
    print(f"   - {sample.payload.to_string()}")
    # [custom_channel_usage]
    count += 1
    # Break after reading a few samples to avoid blocking
    if count >= 2:
        break

# We consumed 4 samples (1 via try_recv, 1 via recv, 2 via iteration)
# so should have 1 remaining
remaining = subscriber.handler.try_recv()
assert remaining is not None
print(f">> Remaining sample: {remaining.payload.to_string()}")
# verify count is zero now
assert subscriber.handler.count() == 0

# Clean up
subscriber.undeclare()
session.close()
