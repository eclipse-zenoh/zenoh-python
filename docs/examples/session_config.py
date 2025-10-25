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

# [session_context_manager]
# Recommended: Using context manager
# The session is automatically closed when exiting the 'with' block
with zenoh.open(zenoh.Config()) as session:
    # Use the session
    session.put("demo/example/hello", "Hello World!")
# [session_context_manager]

# [session_explicit_close]
# Alternative: Explicit open and close
# You must explicitly close the session before script exit
session = zenoh.open(zenoh.Config())
try:
    # Use the session
    session.put("demo/example/hello", "Hello World!")
finally:
    # Always close the session
    session.close()
# [session_explicit_close]
