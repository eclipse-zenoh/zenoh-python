import time

import zenoh


# DOC_EXAMPLE_START
def listener(sample):
    print(f"{sample.key_expr} => {sample.payload.to_string()}")
# DOC_EXAMPLE_END
TIMEOUT = 1
# DOC_EXAMPLE_START
# Subscribe to a set of keys with Zenoh
with zenoh.open(zenoh.Config()) as session:
    with session.declare_subscriber("demo/example/**", listener) as subscriber:
        time.sleep(TIMEOUT)
# DOC_EXAMPLE_END
