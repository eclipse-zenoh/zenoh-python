import zenoh

# Open session
session = zenoh.open(zenoh.Config())

import threading

# Test support: send data in background
import time


def send_data():
    time.sleep(0.1)
    session.put("key/expression", "test data")


threading.Thread(target=send_data, daemon=True).start()

# [pubsub_subscriber]
# Declare a subscriber and receive data
subscriber = session.declare_subscriber("key/expression")
for sample in subscriber:
    print(f">> Received {sample.payload.to_string()}")
    # [pubsub_subscriber]
    break  # Exit after first sample for testing

session.close()
