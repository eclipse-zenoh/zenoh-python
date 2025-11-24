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
import zenoh

# Open session
session = zenoh.open(zenoh.Config())


# [callback_simple]
def on_sample(sample):
    print(sample.payload.to_string())


# Subscriber runs in background mode
subscriber = session.declare_subscriber("key/expr", on_sample)
# The subscriber remains active even if 'subscriber' variable is not used
# [callback_simple]

session.close()
