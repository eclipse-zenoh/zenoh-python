from _test_support import isolated_config

import zenoh

config = isolated_config()

# Open session
session = zenoh.open(config)

# [pubsub_session_direct]
# Direct put operation
session.put("key/expression", "value")

# Direct delete operation
session.delete("key/expression")
# [pubsub_session_direct]

session.close()
