import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Direct put operation
session.put("key/expression", "value")

# Direct delete operation
session.delete("key/expression")
