import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# [pubsub_session_direct]
# Direct put operation
session.put("key/expression", "value")

# Direct delete operation
session.delete("key/expression")
# [pubsub_session_direct]

session.close()
