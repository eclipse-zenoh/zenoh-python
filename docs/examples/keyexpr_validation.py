import zenoh
from zenoh import KeyExpr

# Test verification
valid_created = False
error_caught = False

# DOC_EXAMPLE_START
# Example: Key expression validation
# The KeyExpr constructor validates the syntax and raises ZError on invalid input

try:
    # Valid key expressions
    valid_ke = KeyExpr("robot/sensor/temperature")
    print(f"Valid: {valid_ke}")
# DOC_EXAMPLE_END
    valid_created = True
# DOC_EXAMPLE_START

    # Invalid key expression (double slash without proper canonization)
    invalid_ke = KeyExpr("robot//sensor")
    print(f"Created: {invalid_ke}")
except zenoh.ZError as e:
    print(f"Validation error: {e}")
# DOC_EXAMPLE_END
    error_caught = True

# Test verification: ensure both valid creation and error handling work
assert valid_created, "Expected valid KeyExpr to be created"
assert error_caught, "Expected ZError to be raised for invalid KeyExpr"
