import zenoh

# Open session
# DOC_EXAMPLE_START
session = zenoh.open(zenoh.Config())
# DOC_EXAMPLE_END

# DOC_EXAMPLE_START
# Declare a key expression for optimized routing
declared_ke = session.declare_keyexpr("robot/sensor/temperature")

# Use the declared key expression
publisher = session.declare_publisher(declared_ke)
# DOC_EXAMPLE_END
