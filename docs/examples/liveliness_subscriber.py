import zenoh

# Open session
session = zenoh.open(zenoh.Config())

import threading

# Test support: declare and undeclare liveliness token in background
import time


def provide_token():
    time.sleep(0.1)
    token = session.liveliness().declare_token("node/A")
    time.sleep(0.2)  # Keep token alive briefly
    token.undeclare()  # Trigger DELETE


threading.Thread(target=provide_token, daemon=True).start()

# Test verification counters
put_count = 0
delete_count = 0

# [liveliness_subscriber]
# Check if a liveliness token is present and subscribe to changes
subscriber = session.liveliness().declare_subscriber("node/A", history=True)
for sample in subscriber:
    if sample.kind == zenoh.SampleKind.PUT:
        print(f"Alive token ('{sample.key_expr}')")
    elif sample.kind == zenoh.SampleKind.DELETE:
        print(f"Dropped token ('{sample.key_expr}')")
    # [liveliness_subscriber]
    # Test verification
    if sample.kind == zenoh.SampleKind.PUT:
        put_count += 1
    elif sample.kind == zenoh.SampleKind.DELETE:
        delete_count += 1

    # Exit after receiving both events
    if put_count > 0 and delete_count > 0:
        break

assert put_count > 0, "Expected at least one PUT sample"
assert delete_count > 0, "Expected at least one DELETE sample"

session.close()
