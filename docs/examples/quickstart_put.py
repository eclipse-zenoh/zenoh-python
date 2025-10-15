import zenoh

# Publish a key/value pair onto Zenoh
with zenoh.open() as session:
    session.put("demo/example/hello", "Hello World!")
