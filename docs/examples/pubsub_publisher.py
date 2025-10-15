import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Declare a publisher and publish data
publisher = session.declare_publisher("key/expression")
publisher.put("value")
