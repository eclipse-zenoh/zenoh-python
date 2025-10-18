import zenoh
from zenoh import KeyExpr

# [keyexpr_operations]
# Create a key expression with validation
sensor_ke = KeyExpr("robot/sensor")
assert str(sensor_ke) == "robot/sensor"

# Join with another segment
temp_ke = sensor_ke.join("temp")
assert str(temp_ke) == "robot/sensor/temp"

# Create a wildcard expression
all_sensors = sensor_ke.join("**")
assert str(all_sensors) == "robot/sensor/**"

# Check inclusion
assert all_sensors.includes(temp_ke)
assert not temp_ke.includes(all_sensors)

# Check intersection
assert all_sensors.intersects(temp_ke)
assert not sensor_ke.intersects(KeyExpr("robot/actuator"))
# [keyexpr_operations]
