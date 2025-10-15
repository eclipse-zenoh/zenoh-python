import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Get currently present liveliness tokens
replies = session.liveliness().get("node/A", timeout=5)
for reply in replies:
    if reply.ok:
        print(f"Alive token ('{reply.ok.key_expr}')")
    else:
        print(f"Received (ERROR: '{reply.err.payload.to_string()}')")
