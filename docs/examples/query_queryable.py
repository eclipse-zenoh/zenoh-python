import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Sample data
temperature_data = {
    "2023-03-15": "22.5°C",
    "2023-03-16": "23.1°C",
}

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
