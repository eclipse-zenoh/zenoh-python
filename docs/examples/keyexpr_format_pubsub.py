import threading
import time

import zenoh

# [keyexpr_format_pubsub]
fmt = zenoh.KeFormat("robot/${sensor_id:*}/reading")

session = zenoh.open(zenoh.Config())
received_sensor_ids = []


def publisher_task():
    time.sleep(0.1)
    for sensor_id in ("temperature", "humidity", "pressure"):
        formatter = fmt.formatter()
        formatter.set("sensor_id", sensor_id)
        session.put(formatter.build(), f"reading from {sensor_id}")


subscriber = session.declare_subscriber("robot/*/reading")
threading.Thread(target=publisher_task, daemon=True).start()
for sample in subscriber:
    parsed = fmt.parse(sample.key_expr)
    received_sensor_ids.append(parsed.get("sensor_id"))
    if len(received_sensor_ids) >= 3:
        break

session.close()
assert set(received_sensor_ids) == {"temperature", "humidity", "pressure"}
# [keyexpr_format_pubsub]
