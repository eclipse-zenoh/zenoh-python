from _test_support import isolated_config

import zenoh

config = isolated_config()

# Open session
session = zenoh.open(config)

# [pubsub_publisher]
# Declare a publisher and publish data
publisher = session.declare_publisher("key/expression")
publisher.put("value")
# [pubsub_publisher]

session.close()
