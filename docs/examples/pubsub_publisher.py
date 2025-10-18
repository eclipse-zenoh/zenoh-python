import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# DOC_EXAMPLE_START
# Declare a publisher and publish data
publisher = session.declare_publisher("key/expression")
publisher.put("value")
# DOC_EXAMPLE_END
