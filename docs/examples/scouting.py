import sys
import threading

from _test_support import discovery_config

import zenoh

config = discovery_config()
session = zenoh.open(config)
expected = str(session.info.zid())
found = set()

# [scouting]
scout = zenoh.scout(config=config, what="peer|router")
threading.Timer(1.0, lambda: scout.stop()).start()
for hello in scout:
    print(hello)
    # [scouting]
    found.add(str(hello.zid))
session.close()
assert found <= {expected}, f"Discovered unrelated sessions: {found - {expected}}"
if expected not in found:
    print("Loopback multicast unavailable: no Hello from the test session.")
    sys.exit(77)
