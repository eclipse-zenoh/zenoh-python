from _test_support import isolated_config

import zenoh

config = isolated_config()

# Open session
session = zenoh.open(config)

# [liveliness_token]
# Declare a liveliness token
token = session.liveliness().declare_token("node/A")
# [liveliness_token]

session.close()
