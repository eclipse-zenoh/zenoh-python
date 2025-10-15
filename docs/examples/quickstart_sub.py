import zenoh
import time

def listener(sample):
    print(f"{sample.key_expr} => {sample.payload.to_string()}")

# Subscribe to a set of keys with Zenoh
with zenoh.open() as session:
    with session.declare_subscriber('demo/example/**', listener) as subscriber:
        time.sleep(60)
