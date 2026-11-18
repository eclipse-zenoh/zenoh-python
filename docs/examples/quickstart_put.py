from _test_support import isolated_config

import zenoh

config = isolated_config()

# DOC_EXAMPLE_START
# Publish a key/value pair onto Zenoh
with zenoh.open(config) as session:
    session.put("demo/example/hello", "Hello World!")
# DOC_EXAMPLE_END
