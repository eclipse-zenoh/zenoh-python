import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# DOC_EXAMPLE_START
# Declare a liveliness token
token = session.liveliness().declare_token("node/A")
# DOC_EXAMPLE_END
