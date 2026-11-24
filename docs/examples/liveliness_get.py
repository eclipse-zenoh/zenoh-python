import zenoh

# Open session
session = zenoh.open(zenoh.Config())

import threading

# Test support: declare liveliness token in background
import time


def provide_token():
    time.sleep(0.1)
    token = session.liveliness().declare_token("node/A")
    time.sleep(0.5)


threading.Thread(target=provide_token, daemon=True).start()
time.sleep(0.2)  # Wait for token to be declared

# [liveliness_get]
# Get currently present liveliness tokens
replies = session.liveliness().get("node/A", timeout=5)
for reply in replies:
    if reply.ok:
        print(f"Alive token ('{reply.ok.key_expr}')")
    else:
        print(f"Received (ERROR: '{reply.err.payload.to_string()}')")
    # [liveliness_get]
    break  # Exit after first reply for testing

session.close()
