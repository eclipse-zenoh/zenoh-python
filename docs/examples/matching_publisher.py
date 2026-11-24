import zenoh

# Open session
session = zenoh.open(zenoh.Config())

import threading

# Test support: declare subscriber, then undeclare to trigger both states
import time

subscriber = session.declare_subscriber("key/expression")


def undeclare_subscriber():
    time.sleep(0.2)  # Let matching=True be received first
    subscriber.undeclare()  # Trigger matching=False


threading.Thread(target=undeclare_subscriber, daemon=True).start()

# Test verification counters
matching_true_count = 0
matching_false_count = 0

# [matching_publisher]
# Declare a matching listener for a publisher
publisher = session.declare_publisher("key/expression")
listener = publisher.declare_matching_listener()
for status in listener:
    if status.matching:
        print(">> Publisher has at least one matching subscriber")
    else:
        print(">> Publisher has no matching subscribers")
    # [matching_publisher]
    # Test verification
    if status.matching:
        matching_true_count += 1
    else:
        matching_false_count += 1

    # Exit after receiving both events
    if matching_true_count > 0 and matching_false_count > 0:
        break

assert matching_true_count > 0, "Expected at least one matching=True status"
assert matching_false_count > 0, "Expected at least one matching=False status"

session.close()
