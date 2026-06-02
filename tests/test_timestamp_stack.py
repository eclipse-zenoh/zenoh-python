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
import time
from typing import List, Optional

import pytest

import zenoh
from zenoh import InterceptionPoint, TimestampInstrumentation, TimestampStack

SLEEP = 0.2


def peer_config() -> zenoh.Config:
    cfg = zenoh.Config()
    cfg.insert_json5("scouting/multicast/enabled", "false")
    return cfg


# ── helpers ───────────────────────────────────────────────────────────────────


def collect_one(key: str, action, timeout: float = SLEEP) -> Optional[zenoh.Sample]:
    received: List[zenoh.Sample] = []
    with zenoh.open(peer_config()) as session:
        with session.declare_subscriber(key, lambda s: received.append(s)):
            time.sleep(0.05)
            action(session)
            time.sleep(timeout)
    return received[0] if received else None


# ── test_no_instrumentation ───────────────────────────────────────────────────


def test_no_instrumentation():
    """Without instrumentation the stack should be None."""
    def put(session):
        session.put("test/ts/none", b"hello")

    sample = collect_one("test/ts/none", put)
    assert sample is not None
    assert sample.timestamp_stack is None


# ── test_put_subscribe_send_receive ──────────────────────────────────────────


def test_put_subscribe_send_receive():
    """A put with send+receive instrumentation produces SEND and RECEIVE records."""
    instr = TimestampInstrumentation(send=True, receive=True)

    def put(session):
        session.put("test/ts/put", b"hello", timestamp_instrumentation=instr)

    sample = collect_one("test/ts/put", put)
    assert sample is not None
    stack = sample.timestamp_stack
    assert stack is not None
    points = [r.point for r in stack.records]
    assert InterceptionPoint.SEND in points
    assert InterceptionPoint.RECEIVE in points


# ── test_send_only ────────────────────────────────────────────────────────────


def test_send_only():
    """send=True, receive=False → only SEND record."""
    instr = TimestampInstrumentation(send=True, receive=False)

    def put(session):
        session.put("test/ts/send_only", b"x", timestamp_instrumentation=instr)

    sample = collect_one("test/ts/send_only", put)
    assert sample is not None
    stack = sample.timestamp_stack
    assert stack is not None
    points = [r.point for r in stack.records]
    assert InterceptionPoint.SEND in points
    assert InterceptionPoint.RECEIVE not in points


# ── test_receive_only ─────────────────────────────────────────────────────────


def test_receive_only():
    """receive=True, send=False → only RECEIVE record."""
    instr = TimestampInstrumentation(send=False, receive=True)

    def put(session):
        session.put("test/ts/recv_only", b"x", timestamp_instrumentation=instr)

    sample = collect_one("test/ts/recv_only", put)
    assert sample is not None
    stack = sample.timestamp_stack
    assert stack is not None
    points = [r.point for r in stack.records]
    assert InterceptionPoint.RECEIVE in points
    assert InterceptionPoint.SEND not in points


# ── test_publisher_default ────────────────────────────────────────────────────


def test_publisher_default():
    """Publisher-level default instrumentation applies to all puts."""
    instr = TimestampInstrumentation(send=True, receive=True)
    received: List[zenoh.Sample] = []

    with zenoh.open(peer_config()) as session:
        with session.declare_subscriber(
            "test/ts/pub_default", lambda s: received.append(s)
        ):
            with session.declare_publisher(
                "test/ts/pub_default", timestamp_instrumentation=instr
            ) as pub:
                time.sleep(0.05)
                pub.put(b"data")
                time.sleep(SLEEP)

    assert len(received) == 1
    stack = received[0].timestamp_stack
    assert stack is not None
    points = [r.point for r in stack.records]
    assert InterceptionPoint.SEND in points
    assert InterceptionPoint.RECEIVE in points


# ── test_publisher_per_put_override ──────────────────────────────────────────


def test_publisher_per_put_override():
    """Per-put override takes precedence over publisher default."""
    default_instr = TimestampInstrumentation(send=True, receive=True)
    override_instr = TimestampInstrumentation(send=True, receive=False)
    received: List[zenoh.Sample] = []

    with zenoh.open(peer_config()) as session:
        with session.declare_subscriber(
            "test/ts/pub_override", lambda s: received.append(s)
        ):
            with session.declare_publisher(
                "test/ts/pub_override", timestamp_instrumentation=default_instr
            ) as pub:
                time.sleep(0.05)
                pub.put(b"data", timestamp_instrumentation=override_instr)
                time.sleep(SLEEP)

    assert len(received) == 1
    points = [r.point for r in received[0].timestamp_stack.records]
    assert InterceptionPoint.SEND in points
    assert InterceptionPoint.RECEIVE not in points


# ── test_as_timestamp ─────────────────────────────────────────────────────────


def test_as_timestamp():
    """Standard HLC records decode via as_timestamp(); returns a Timestamp object."""
    instr = TimestampInstrumentation(send=True, receive=True)

    def put(session):
        session.put("test/ts/as_ts", b"t", timestamp_instrumentation=instr)

    sample = collect_one("test/ts/as_ts", put)
    assert sample is not None
    for r in sample.timestamp_stack.records:
        if not r.is_custom:
            ts = r.as_timestamp()
            assert ts is not None


# ── test_is_custom_false ──────────────────────────────────────────────────────


def test_is_custom_false():
    """Standard (non-callback) records have is_custom == False."""
    instr = TimestampInstrumentation(send=True, receive=True)

    def put(session):
        session.put("test/ts/not_custom", b"x", timestamp_instrumentation=instr)

    sample = collect_one("test/ts/not_custom", put)
    assert sample is not None
    for r in sample.timestamp_stack.records:
        assert not r.is_custom


# ── test_custom_callback ──────────────────────────────────────────────────────


def test_custom_callback():
    """A session-level timestamp callback produces custom records with the returned bytes."""
    MARKER = b"custom-ts-bytes"

    def my_callback(ctx):
        return MARKER

    instr = TimestampInstrumentation(send=True, receive=True)
    received: List[zenoh.Sample] = []

    with zenoh.open(peer_config(), timestamp_callback=my_callback) as session:
        with session.declare_subscriber(
            "test/ts/custom_cb", lambda s: received.append(s)
        ):
            time.sleep(0.05)
            session.put("test/ts/custom_cb", b"x", timestamp_instrumentation=instr)
            time.sleep(SLEEP)

    assert len(received) == 1
    stack = received[0].timestamp_stack
    assert stack is not None
    custom_records = [r for r in stack.records if r.is_custom]
    assert len(custom_records) > 0
    for r in custom_records:
        assert r.timestamp() == MARKER
        assert r.as_timestamp() is None  # custom bytes don't decode as UHLC


# ── test_invalid_instrumentation ─────────────────────────────────────────────


def test_invalid_instrumentation():
    """All-false instrumentation should raise (at least one point required)."""
    with pytest.raises(Exception):
        TimestampInstrumentation(send=False, route=False, receive=False)
