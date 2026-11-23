import time
from concurrent.futures import ThreadPoolExecutor
from queue import Empty

from _test_support import connecting_config, open_listener

import zenoh


def send_data(send_session):
    with send_session.declare_publisher("demo/example/foo/bar") as publisher:
        deadline = time.monotonic() + 5
        while not publisher.matching_status.matching:
            if time.monotonic() >= deadline:
                raise TimeoutError("No matching subscriber within 5 seconds")
            time.sleep(0.01)
        publisher.put("test data")


def subscribe(config):
    # DOC_EXAMPLE_START
    from queue import SimpleQueue

    samples = SimpleQueue()
    with zenoh.open(config) as session:
        with session.declare_subscriber("demo/example/**", samples.put):
            sample = samples.get(timeout=5)
            print(f"{sample.key_expr} => {sample.payload.to_string()}")
    # DOC_EXAMPLE_END


# Use a private loopback connection for the background test publisher.
send_session, endpoint = open_listener()
with send_session, ThreadPoolExecutor(max_workers=1) as executor:
    sender = executor.submit(send_data, send_session)
    try:
        subscribe(connecting_config(endpoint))
    except Empty as error:
        sender.result()
        raise TimeoutError("No sample received within 5 seconds") from error
    sender.result()
