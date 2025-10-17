import zenoh

# Publish a key/value pair onto Zenoh
with zenoh.open(zenoh.Config()) as session:
    session.put("demo/example/hello", "Hello World!")
