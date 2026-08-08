import zenoh

# [keyexpr_format]
fmt = zenoh.KeFormat("robot/${sensor_id:*}/reading")

formatter = fmt.formatter()
formatter.set("sensor_id", "temperature")
key = formatter.build()
assert str(key) == "robot/temperature/reading"

parsed = fmt.parse("robot/humidity/reading")
sensor_id = parsed.get("sensor_id")
assert sensor_id == "humidity"
# [keyexpr_format]
