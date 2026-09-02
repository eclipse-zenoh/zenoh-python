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


# Test support: send data in background
def send_data(session: zenoh.Session):
    time.sleep(3)
    for i in range(2):
        session.put("key/expression", f"sample_{i}")


# [custom_channel]
class PriorityChannel:
    def __init__(self, maxsize=100):
        self.queue: queue.PriorityQueue = queue.PriorityQueue(maxsize)
        # Counter to preserve FIFO order for samples with same priority
        self._counter = 0

    def recv(self) -> zenoh.Sample:
        return self.queue.get()[2]

    def send(self, sample: zenoh.Sample):
        self.queue.put((sample.priority, self._counter, sample))
        self._counter += 1

    # [custom_channel]
    def count(self) -> int:
        return self.queue.qsize()


# [custom_channel_usage]
with zenoh.open(zenoh.Config()) as session:
    channel = PriorityChannel(maxsize=50)
    subscriber = session.declare_subscriber("key/expression", (channel.send, channel))
    # Start publishing only after the session and subscriber are ready.
    sender = threading.Thread(target=send_data, args=(session,))
    sender.start()
    sample = subscriber.handler.recv()
    print(f">> Received: {sample.payload.to_string()}")
    # [custom_channel_usage]
    # One sample should remain in the channel. Wait for asynchronous delivery
    # instead of relying on a fixed sleep.
    deadline = time.monotonic() + 5
    while channel.count() < 1 and time.monotonic() < deadline:
        time.sleep(0.01)
    assert channel.count() == 1  # verify that one sample is still in the channel
    sender.join(timeout=5)
    assert not sender.is_alive()
