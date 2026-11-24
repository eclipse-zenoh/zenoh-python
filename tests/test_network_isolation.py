#
# Copyright (c) 2026 ZettaScale Technology
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
#
import itertools
import socket
import time
from contextlib import ExitStack

import network_config as testing
import pytest

import zenoh


def wait_for(predicate):
    deadline = time.monotonic() + 5
    while not predicate():
        assert time.monotonic() < deadline, "Timed out waiting for test peers"
        time.sleep(0.01)


def receive(subscriber):
    deadline = time.monotonic() + 5
    while (sample := subscriber.try_recv()) is None:
        assert time.monotonic() < deadline, "Timed out waiting for a sample"
        time.sleep(0.01)
    return sample


def test_same_key_on_independent_networks(session_pair):
    first_pub, first_sub = session_pair()
    second_pub, second_sub = session_pair()
    key = "test/isolation/shared-key"
    with ExitStack() as stack:
        first = stack.enter_context(
            first_sub.declare_subscriber(key, zenoh.handlers.FifoChannel(32))
        )
        second = stack.enter_context(
            second_sub.declare_subscriber(key, zenoh.handlers.FifoChannel(32))
        )
        pub1 = stack.enter_context(
            first_pub.declare_publisher(
                key, congestion_control=zenoh.CongestionControl.BLOCK
            )
        )
        pub2 = stack.enter_context(
            second_pub.declare_publisher(
                key, congestion_control=zenoh.CongestionControl.BLOCK
            )
        )
        wait_for(
            lambda: pub1.matching_status.matching and pub2.matching_status.matching
        )
        for number in range(10):
            pub1.put(f"first-{number}")
            pub2.put(f"second-{number}")
        for number in range(10):
            assert receive(first).payload.to_string() == f"first-{number}"
            assert receive(second).payload.to_string() == f"second-{number}"
        assert first.try_recv() is None
        assert second.try_recv() is None

    for listener, connector in ((first_pub, first_sub), (second_pub, second_sub)):
        assert [str(zid) for zid in listener.info.peers_zid()] == [
            str(connector.info.zid())
        ]
        assert [str(zid) for zid in connector.info.peers_zid()] == [
            str(listener.info.zid())
        ]
        assert not listener.info.routers_zid()
        assert not connector.info.routers_zid()


def test_occupied_listener_port_is_retried(monkeypatch):
    with socket.socket() as occupied:
        occupied.bind(("127.0.0.1", 0))
        occupied.listen()
        port = occupied.getsockname()[1]
        choose_port = testing.random.randrange
        ports = itertools.chain([port], (choose_port(20000, 32768) for _ in range(15)))
        monkeypatch.setattr(testing.random, "randrange", lambda *_: next(ports))
        listener, endpoint = testing.open_listener()
        with listener:
            assert endpoint != f"tcp/127.0.0.1:{port}"


def test_occupied_listener_ports_fail_after_bounded_retries(monkeypatch):
    with socket.socket() as occupied:
        occupied.bind(("127.0.0.1", 0))
        occupied.listen()
        port = occupied.getsockname()[1]
        monkeypatch.setattr(testing.random, "randrange", lambda *_: port)
        with pytest.raises(RuntimeError, match="after 16 attempts"):
            testing.open_listener()


def test_windows_port_collision_is_retried(monkeypatch):
    open_session = zenoh.open
    attempts = 0

    def open_with_collision(config, **options):
        nonlocal attempts
        attempts += 1
        if attempts == 1:
            raise zenoh.ZError("Only one usage of each socket address (os error 10048)")
        return open_session(config, **options)

    monkeypatch.setattr(zenoh, "open", open_with_collision)
    listener, _ = testing.open_listener()
    with listener:
        assert attempts >= 2


def test_discovery_networks_do_not_mix():
    first_config = testing.discovery_config()
    second_config = testing.discovery_config()
    with zenoh.open(first_config) as first, zenoh.open(second_config) as second:
        first_ids, second_ids = set(), set()
        first_scout = zenoh.scout(
            lambda hello: first_ids.add(str(hello.zid)), config=first_config
        )
        try:
            second_scout = zenoh.scout(
                lambda hello: second_ids.add(str(hello.zid)), config=second_config
            )
            try:
                time.sleep(1.5)
            finally:
                second_scout.stop()
        finally:
            first_scout.stop()
        expected_first = str(first.info.zid())
        expected_second = str(second.info.zid())
        assert first_ids == {expected_first}
        assert second_ids == {expected_second}
