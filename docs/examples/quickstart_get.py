import zenoh

# Get keys/values from zenoh
with zenoh.open(zenoh.Config()) as session:
    for response in session.get("demo/example/**"):
        response = response.ok
        print(f"{response.key_expr} => {response.payload.to_string()}")
