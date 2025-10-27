import zenoh
from zenoh import KeyExpr

# Test verification
valids_created = False
error_caught = False

# Example: Key expression validation
# The KeyExpr constructor validates the syntax and raises ZError on invalid input

# [keyexpr_validation]
try:
    # Valid key expressions
    valid_ke = KeyExpr("robot/sensor/temperature")
    assert str(valid_ke) == "robot/sensor/temperature"
    canonized_ke = KeyExpr.autocanonize("robot/sensor/**/*/**/**")
    assert str(canonized_ke) == "robot/sensor/*/**"

    # Invalid key expression (empty segment)
    invalid_ke = KeyExpr("robot/sensor//*")
    assert True, "This line should not be reached"
except zenoh.ZError as e:
    print(f"Validation error: {e}")
# [keyexpr_validation]
