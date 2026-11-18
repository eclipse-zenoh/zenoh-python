from _test_support import isolated_config

import zenoh

config = isolated_config()

# DOC_EXAMPLE_START
# Get keys/values from zenoh
with zenoh.open(config) as session:
    for response in session.get("demo/example/**"):
        response = response.ok
        print(f"{response.key_expr} => {response.payload.to_string()}")
# DOC_EXAMPLE_END
