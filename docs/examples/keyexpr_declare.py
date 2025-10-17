import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# DOC_EXAMPLE_START
# Declare a key expression for optimized routing
declared_ke = session.declare_keyexpr("robot/sensor/temperature")

# Use the declared key expression
publisher = session.declare_publisher(declared_ke)
# DOC_EXAMPLE_END
