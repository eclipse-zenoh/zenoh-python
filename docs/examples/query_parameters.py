import zenoh

# Open session
session = zenoh.open(zenoh.Config())

import threading
import time


def provide_queryable():
    time.sleep(0.05)
    queryable = session.declare_queryable("room/temperature/history")
    for query in queryable:
        # Access query parameters
        day = query.parameters.get("day", "unknown")
        format = query.parameters.get("format", "celsius")
        query.reply("room/temperature/history", f"22.5°C on {day} ({format})")
        break


threading.Thread(target=provide_queryable, daemon=True).start()
time.sleep(0.1)  # Wait for queryable to be ready

# Test verification
reply_count = 0

# [query_parameters]
# Create parameters from a dictionary
params = zenoh.Parameters({"day": "2023-03-15", "format": "celsius"})

# Create a selector from key expression and parameters
selector = zenoh.Selector("room/temperature/history", params)

# Request data using the selector
replies = session.get(selector)
for reply in replies:
    if reply.ok:
        print(f">> {reply.ok.payload.to_string()}")
    # [query_parameters]
    # Test verification
    if reply.ok:
        reply_count += 1
        assert "2023-03-15" in reply.ok.payload.to_string()
        assert "celsius" in reply.ok.payload.to_string()

assert reply_count > 0, "Expected at least one reply"

session.close()
