import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Test support: declare liveliness token in background
import time
import threading
def provide_token():
    time.sleep(0.1)
    session.liveliness().declare_token("node/A")
threading.Thread(target=provide_token, daemon=True).start()

# DOC_EXAMPLE_START
# Check if a liveliness token is present and subscribe to changes
subscriber = session.liveliness().declare_subscriber("node/A", history=True)
for sample in subscriber:
    if sample.kind == zenoh.SampleKind.PUT:
        print(f"Alive token ('{sample.key_expr}')")
    elif sample.kind == zenoh.SampleKind.DELETE:
        print(f"Dropped token ('{sample.key_expr}')")
# DOC_EXAMPLE_END
    break  # Exit after first sample for testing
