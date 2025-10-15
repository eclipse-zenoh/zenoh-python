import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Request temperature for a specific day
replies = session.get("room/temperature/history?day=2023-03-15")
for reply in replies:
    if reply.ok:
        print(f">> Temperature is {reply.ok.payload.to_string()}")
    else:
        print(f">> Error: {reply.err.payload.to_string()}")
