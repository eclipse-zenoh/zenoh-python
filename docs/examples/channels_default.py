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

# [channels_default]
# Default channel
subscriber = session.declare_subscriber("key/expr")
for sample in subscriber:
    print(sample.payload.to_string())
    # [channels_default]
    break  # Exit after first sample for testing

# Clean up
subscriber.undeclare()
session.close()
