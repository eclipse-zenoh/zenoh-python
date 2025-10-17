import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Test support: provide queryable in background
import time
import threading
def provide_queryable():
    time.sleep(0.1)
    queryable = session.declare_queryable("room/temperature/history")
    for query in queryable:
        query.reply("room/temperature/history", "22.5°C")
        break
threading.Thread(target=provide_queryable, daemon=True).start()

# DOC_EXAMPLE_START
# Declare a querier for multiple queries
querier = session.declare_querier("room/temperature/history")

# Send a query with parameters
replies = querier.get(parameters="?day=2023-03-15")
for reply in replies:
    if reply.ok:
        print(f">> Temperature is {reply.ok.payload.to_string()}")
    else:
        print(f">> Error: {reply.err.payload.to_string()}")
# DOC_EXAMPLE_END
    break  # Exit after first reply for testing
