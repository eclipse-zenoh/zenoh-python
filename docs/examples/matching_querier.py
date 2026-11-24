import zenoh

# Open session
session = zenoh.open(zenoh.Config())

import threading

# Test support: declare queryable, then undeclare to trigger both states
import time

queryable = session.declare_queryable("service/endpoint")


def undeclare_queryable():
    time.sleep(0.2)  # Let matching=True be received first
    queryable.undeclare()  # Trigger matching=False


threading.Thread(target=undeclare_queryable, daemon=True).start()

# Test verification counters
matching_true_count = 0
matching_false_count = 0

# [matching_querier]
# Declare a matching listener for a querier
querier = session.declare_querier("service/endpoint")
listener = querier.declare_matching_listener()
for status in listener:
    if status.matching:
        print(">> Querier has at least one matching queryable")
    else:
        print(">> Querier has no matching queryables")
    # [matching_querier]
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
