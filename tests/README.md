<!-- Copyright (c) 2026 ZettaScale Technology -->

# Test networks

Network tests use the helpers in `examples/common/testing.py`, following the
Zenoh C and C++ test harnesses. Keep each test on its own network, even when it
uses a unique key: matching, discovery, and peer-information checks can also be
affected by unrelated sessions.

- Use the `session_pair` fixture for two connected sessions. It disables multicast
  and gossip, clears default endpoints, binds a loopback listener, and connects
  its peer explicitly. The fixture closes both sessions after the test, including
  on assertion failures. Pass listener options such as `timestamp_callback` to
  the fixture's factory when needed.
- Listener helpers retry actual binds on random ports in 20000–32767, at most
  16 times. Keep the successful listener open while connecting. Do not probe and
  release a free port before asking Zenoh to bind it. Connectors must establish
  their link within five seconds or fail.
- Command-line data-exchange tests use the `pyrun` fixture. Each test gets a
  private loopback router; its child examples disable discovery and connect only
  to that router in client mode. This explicitly tests routed data exchange;
  the unit-test session pairs retain peer mode. The bytes example needs no
  network, and the scout example uses a dedicated discovery configuration
  instead. Child process groups are stopped
  and reaped during teardown.
- Documentation examples share the same configuration helpers through
  `docs/examples/_test_support.py`. Most only need an isolated local session;
  the subscriber quickstart uses an explicit loopback listener and connector.
- Tests that exercise discovery use `discovery_config()`: multicast stays on,
  restricted to loopback with a separate multicast group and port. Verify the
  expected session's identity rather than accepting arbitrary discovered peers.
  Discovery checks skip when loopback multicast cannot return the test session's
  Hello, as in the C++ locator tests.

With a built binding installed in the active Python environment:

```sh
pytest -n auto --import-mode=append
pytest tests/examples_check.py -n 4 -v --import-mode=append
```

The isolation regression tests deliberately reuse keys across independent
networks, check the peer identities, exercise occupied-port recovery and bounded
failure, and check that two discovery domains see only their own session.
