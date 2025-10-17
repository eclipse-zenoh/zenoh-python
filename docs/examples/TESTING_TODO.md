# Documentation Examples - Branch Coverage TODO

This file tracks which documentation examples have untested code branches and need enhancement.

## Examples with Untested Branches

### 1. query_session_get.py (lines 20-24)
**Status**: ⚠️ Incomplete branch coverage

**Code**:
```python
if reply.ok:
    print(f">> Temperature is {reply.ok.payload.to_string()}")
else:
    print(f">> Error: {reply.err.payload.to_string()}")
```

**Coverage**:
- ✅ Tested: `reply.ok` branch
- ❌ Untested: `else` branch (reply.err)

**TODO**: Add test query that triggers error reply to test both branches

---

### 2. query_querier.py (lines 23-27)
**Status**: ⚠️ Incomplete branch coverage

**Code**:
```python
if reply.ok:
    print(f">> Temperature is {reply.ok.payload.to_string()}")
else:
    print(f">> Error: {reply.err.payload.to_string()}")
```

**Coverage**:
- ✅ Tested: `reply.ok` branch
- ❌ Untested: `else` branch (reply.err)

**TODO**: Add test queryable that sends error reply to test both branches

---

### 3. liveliness_subscriber.py (lines 17-21)
**Status**: ⚠️ Incomplete branch coverage

**Code**:
```python
if sample.kind == zenoh.SampleKind.PUT:
    print(f"Alive token ('{sample.key_expr}')")
elif sample.kind == zenoh.SampleKind.DELETE:
    print(f"Dropped token ('{sample.key_expr}')")
```

**Coverage**:
- ✅ Tested: `PUT` branch (token appears)
- ❌ Untested: `DELETE` branch (token disappears)

**TODO**: Add test that declares token, then undeclares it to trigger both branches

---

### 4. matching_publisher.py (lines 13-17)
**Status**: ⚠️ Incomplete branch coverage

**Code**:
```python
if status.matching:
    print(">> Publisher has at least one matching subscriber")
else:
    print(">> Publisher has no matching subscribers")
```

**Coverage**:
- ✅ Tested: `matching=True` branch
- ❌ Untested: `else` branch (matching=False)

**TODO**: Consider if testing no-match case is valuable for documentation

---

### 5. matching_querier.py (lines 13-17)
**Status**: ⚠️ Incomplete branch coverage

**Code**:
```python
if status.matching:
    print(">> Querier has at least one matching queryable")
else:
    print(">> Querier has no matching queryables")
```

**Coverage**:
- ✅ Tested: `matching=True` branch
- ❌ Untested: `else` branch (matching=False)

**TODO**: Consider if testing no-match case is valuable for documentation

---

## Examples with Complete Coverage

- ✅ **query_queryable.py** - Tests all 3 branches (reply, reply_del, reply_err)
- ✅ **pubsub_subscriber.py** - No conditional branches
- ✅ **pubsub_publisher.py** - No conditional branches
- ✅ **pubsub_session_direct.py** - No conditional branches
- ✅ **keyexpr_operations.py** - No conditional branches
- ✅ **keyexpr_declare.py** - No conditional branches
- ✅ **scouting.py** - No conditional branches
- ✅ **liveliness_token.py** - No conditional branches
- ✅ **liveliness_get.py** - No conditional branches (has if/else but only tests one path)

---

## Priority

**High Priority** (should test all branches):
1. liveliness_subscriber.py - Important to show both PUT and DELETE behavior
2. query_session_get.py - Important to show error handling
3. query_querier.py - Important to show error handling

**Low Priority** (optional):
4. matching_publisher.py - Mostly demonstrates positive matching
5. matching_querier.py - Mostly demonstrates positive matching

---

## Notes

- All examples currently pass execution tests
- Branch coverage is about completeness, not correctness
- Some examples may intentionally only demonstrate the "happy path" for documentation purposes
