import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Declare a liveliness token
token = session.liveliness().declare_token("node/A")
