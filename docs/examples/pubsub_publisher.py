import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# [pubsub_publisher]
# Declare a publisher and publish data
publisher = session.declare_publisher("key/expression")
publisher.put("value")
# [pubsub_publisher]

session.close()
