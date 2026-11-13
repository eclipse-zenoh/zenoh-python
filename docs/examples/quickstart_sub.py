import threading
import time

from _test_support import connecting_config, open_listener

import zenoh


def send_data(send_session):
    for _ in range(5):
        time.sleep(0.2)
        send_session.put("demo/example/foo/bar", "test data")


# Use a private loopback connection for the background test publisher.
send_session, endpoint = open_listener()
config = connecting_config(endpoint)
sender = threading.Thread(target=send_data, args=(send_session,))
sender.start()
received = False

# DOC_EXAMPLE_START
# Subscribe to a set of keys with Zenoh
with zenoh.open(config) as session:
    with session.declare_subscriber("demo/example/**") as subscriber:
        for sample in subscriber:
            print(f"{sample.key_expr} => {sample.payload.to_string()}")
            # DOC_EXAMPLE_END
            received = True
            break

sender.join(timeout=5)
send_session.close()
assert not sender.is_alive()
assert received, "Did not receive any sample within the timeout"
