import zenoh
from zenoh import KeyExpr

# DOC_EXAMPLE_START
# Create a key expression with validation
sensor_ke = KeyExpr("robot/sensor")

# Join with another segment
temp_ke = sensor_ke.join("temp")

# Create a wildcard expression
all_sensors = sensor_ke.join("**")
# DOC_EXAMPLE_END
