import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Declare a querier for multiple queries
querier = session.declare_querier("room/temperature/history")

# Send a query with parameters
replies = querier.get(parameters="?day=2023-03-15")
for reply in replies:
    if reply.ok:
        print(f">> Temperature is {reply.ok.payload.to_string()}")
    else:
        print(f">> Error: {reply.err.payload.to_string()}")
