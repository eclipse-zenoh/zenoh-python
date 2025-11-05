import threading
import time

import zenoh


def send_data():
    send_session = zenoh.open(zenoh.Config())
    for _ in range(5):
        time.sleep(0.2)
        send_session.put("demo/example/foo/bar", "test data")
    send_session.close()


threading.Thread(target=send_data, daemon=True).start()
received = False

# DOC_EXAMPLE_START
# Subscribe to a set of keys with Zenoh
with zenoh.open(zenoh.Config()) as session:
    with session.declare_subscriber("demo/example/**") as subscriber:
        for sample in subscriber:
            print(f"{sample.key_expr} => {sample.payload.to_string()}")
            # DOC_EXAMPLE_END
            received = True
            break

assert received, "Did not receive any sample within the timeout"
