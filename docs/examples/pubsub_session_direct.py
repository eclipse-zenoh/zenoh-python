import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# DOC_EXAMPLE_START
# Direct put operation
session.put("key/expression", "value")

# Direct delete operation
session.delete("key/expression")
# DOC_EXAMPLE_END
