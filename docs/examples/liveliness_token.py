import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# [liveliness_token]
# Declare a liveliness token
token = session.liveliness().declare_token("node/A")
# [liveliness_token]

session.close()
