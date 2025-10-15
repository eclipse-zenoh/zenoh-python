import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Declare a subscriber and receive data
subscriber = session.declare_subscriber("key/expression")
for sample in subscriber:
    print(f">> Received {sample.payload.to_string()}")
