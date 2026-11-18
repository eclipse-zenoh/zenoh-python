from _test_support import isolated_config

import zenoh

config = isolated_config()

# Open session
# [session_creation]
session = zenoh.open(config)
# [session_creation]

# [keyexpr_declare]
# Declare a key expression for optimized routing
declared_ke = session.declare_keyexpr("robot/sensor/temperature")

# Use the declared key expression
publisher = session.declare_publisher(declared_ke)
# [keyexpr_declare]

session.close()
