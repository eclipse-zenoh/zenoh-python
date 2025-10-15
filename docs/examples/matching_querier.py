import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Declare a matching listener for a querier
querier = session.declare_querier("service/endpoint")
listener = querier.declare_matching_listener()
for status in listener:
    if status.matching:
        print(">> Querier has at least one matching queryable")
    else:
        print(">> Querier has no matching queryables")
