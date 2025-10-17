import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Test support: declare subscriber first, then matching will be immediate
subscriber = session.declare_subscriber("key/expression")

# DOC_EXAMPLE_START
# Declare a matching listener for a publisher
publisher = session.declare_publisher("key/expression")
listener = publisher.declare_matching_listener()
for status in listener:
    if status.matching:
        print(">> Publisher has at least one matching subscriber")
    else:
        print(">> Publisher has no matching subscribers")
# DOC_EXAMPLE_END
    break  # Exit after first status for testing
