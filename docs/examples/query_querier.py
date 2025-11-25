import zenoh

# Open session
session = zenoh.open(zenoh.Config())

import threading

# Test support: provide queryables in background (one success, one error)
import time


def provide_queryable_ok():
    time.sleep(0.05)
    queryable = session.declare_queryable("room/temperature/history")
    for query in queryable:
        query.reply("room/temperature/history", "22.5°C")
        break


def provide_queryable_err():
    time.sleep(0.05)
    queryable = session.declare_queryable("room/temperature/history")
    for query in queryable:
        query.reply_err("sensor malfunction")
        break


threading.Thread(target=provide_queryable_ok, daemon=True).start()
threading.Thread(target=provide_queryable_err, daemon=True).start()
time.sleep(0.1)  # Wait for queryables to be ready

# Test verification counters
ok_count = 0
err_count = 0

# [query_querier]
# Declare a querier for multiple queries
querier = session.declare_querier("room/temperature/history")

# Send a query with parameters
replies = querier.get(parameters="?day=2023-03-15")
for reply in replies:
    if reply.ok:
        print(f">> Temperature is {reply.ok.payload.to_string()}")
    else:
        print(f">> Error: {reply.err.payload.to_string()}")
    # [query_querier]
    # Test verification
    if reply.ok:
        ok_count += 1
    else:
        err_count += 1

assert ok_count > 0, "Expected at least one OK reply"
assert err_count > 0, "Expected at least one error reply"

session.close()
