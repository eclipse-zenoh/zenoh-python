import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Declare a key expression for optimized routing
declared_ke = session.declare_keyexpr("robot/sensor/temperature")

# Use the declared key expression
publisher = session.declare_publisher(declared_ke)
