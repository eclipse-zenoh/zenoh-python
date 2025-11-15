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

# [channels]
# Default channel
subscriber_default = session.declare_subscriber("key/expr")

# Explicit FIFO channel with custom capacity
subscriber_fifo = session.declare_subscriber(
    "key/expr", zenoh.handlers.FifoChannel(100)
)

# Ring channel (drops oldest when full)
subscriber_ring = session.declare_subscriber("key/expr", zenoh.handlers.RingChannel(50))
# [channels]

session.close()
