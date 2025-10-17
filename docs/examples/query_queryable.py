import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Sample data
temperature_data = {"2023-03-15": "22.5°C", "2023-03-16": "23.1°C"}

# Test support: send query in background
import time
import threading
def send_query():
    time.sleep(0.1)
    for _ in session.get("room/temperature/history?day=2023-03-15"):
        pass
threading.Thread(target=send_query, daemon=True).start()

# DOC_EXAMPLE_START
# Queryable that replies with temperature data for a given day
queryable = session.declare_queryable("room/temperature/history")
for query in queryable:
    if "day" in query.selector.parameters:
        day = query.selector.parameters["day"]
        if day in temperature_data:
            query.reply("room/temperature/history", temperature_data[day])
        else:
            query.reply_del("no data for this day")
    else:
        query.reply_err("missing day parameter")
# DOC_EXAMPLE_END
    break  # Exit after first query
