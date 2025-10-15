import zenoh

# Open session
session = zenoh.open(zenoh.Config())

# Check if a liveliness token is present and subscribe to changes
with session.liveliness().declare_subscriber("node/A", history=True) as sub:
    for sample in sub:
        if sample.kind == zenoh.SampleKind.PUT:
            print(f"Alive token ('{sample.key_expr}')")
        elif sample.kind == zenoh.SampleKind.DELETE:
            print(f"Dropped token ('{sample.key_expr}')")
