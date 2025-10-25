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

# [string_operations]
encoding = zenoh.Encoding("text/plain")
text = str(encoding)
assert text == "text/plain"
# [string_operations]

# [schema]
encoding1 = zenoh.Encoding("text/plain;utf-8")
encoding2 = zenoh.Encoding.TEXT_PLAIN.with_schema("utf-8")
assert encoding1 == encoding2
assert str(encoding1) == "text/plain;utf-8"
assert str(encoding2) == "text/plain;utf-8"
# [schema]
