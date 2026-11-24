import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Sample data
temperature_data = {"2023-03-15": "22.5°C", "2023-03-16": "23.1°C"}

import threading

# Test support: send all 3 query variants in background
import time


def send_queries():
    time.sleep(0.2)  # Wait for queryable to be ready

    # Send all 3 queries with callback (non-blocking)
    def callback(reply):
        pass

    # Query 1: has data (reply)
    session.get("room/temperature/history?day=2023-03-15", callback)
    # Query 2: no data (reply_del)
    session.get("room/temperature/history?day=2023-03-17", callback)
    # Query 3: missing parameter (reply_err)
    session.get("room/temperature/history", callback)


send_thread = threading.Thread(target=send_queries)
send_thread.start()

# [query_queryable]
# Queryable that replies with temperature data for a given day
queryable = session.declare_queryable("room/temperature/history")
query_count = 0
for query in queryable:
    if "day" in query.selector.parameters:
        day = query.selector.parameters["day"]
        if day in temperature_data:
            query.reply("room/temperature/history", temperature_data[day])
        else:
            query.reply_del("room/temperature/history")
    else:
        query.reply_err("missing day parameter")
    # [query_queryable]
    query_count += 1
    if query_count >= 3:  # Exit after handling all 3 queries
        break

send_thread.join()
session.close()
