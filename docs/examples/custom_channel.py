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
import queue
import threading
import time
from collections.abc import Callable
from typing import Generic, TypeVar, Union

import zenoh

_T = TypeVar("_T")

# Open session
session = zenoh.open(zenoh.Config())


# Test support: send data in background
def send_data():
    time.sleep(1)
    for i in range(2):
        session.put("key/expression", f"sample_{i}")


threading.Thread(target=send_data, daemon=True).start()


# [custom_channel]
class PriorityChannel(Generic[_T]):
    def __init__(self, maxsize=100):
        self.queue: queue.PriorityQueue[tuple[zenoh.Priority, _T]] = (
            queue.PriorityQueue(maxsize)
        )

    def recv(self) -> _T:
        return self.queue.get()[1]

    def __iter__(self):
        return self

    def __next__(self) -> _T:
        sample = self.recv()
        if sample is None:
            raise StopIteration
        return sample

    def put(self, priority: zenoh.Priority, sample: _T):
        """Called by the callback to store samples"""
        self.queue.put((priority, sample))

    def count(self) -> int:
        """Return number of stored samples"""
        return self.queue.qsize()


def create_priority_channel(
    maxsize: int = 100,
) -> tuple[Callable[[zenoh.Sample], None], PriorityChannel[zenoh.Sample]]:
    """Factory function that returns (callback, handler) pair"""
    channel: PriorityChannel[zenoh.Sample] = PriorityChannel(maxsize)

    def on_sample(sample: zenoh.Sample) -> None:
        channel.put(sample.priority, sample)

    return (on_sample, channel)


# [custom_channel]

# [custom_channel_usage]
subscriber = session.declare_subscriber(
    "key/expression", create_priority_channel(maxsize=50)
)
sample = subscriber.handler.recv()
print(f">> Received: {sample.payload.to_string()}")
# Access to custom channel methods via handler
print(f">> Samples currently stored in channel: {subscriber.handler.count()}")
# [custom_channel_usage]

# We consumed 1 sample so should have 1 remaining
assert subscriber.handler.count() == 1

# Clean up
subscriber.undeclare()
session.close()
