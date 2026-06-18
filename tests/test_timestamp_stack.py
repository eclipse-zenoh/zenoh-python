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
import json
import time

import zenoh
from zenoh import (
    InterceptionPoint,
    Sample,
    TimestampInstrumentation,
    TimestampInstrumentationBuilder,
    TimestampStack,
    TimestampContext,
)

SLEEP = 1


def open_session(endpoints: list[str]) -> tuple[zenoh.Session, zenoh.Session]:
    conf = zenoh.Config()
    conf.insert_json5("listen/endpoints", json.dumps(endpoints))
    conf.insert_json5("scouting/multicast/enabled", "false")
    peer01 = zenoh.open(conf)

    conf = zenoh.Config()
    conf.insert_json5("connect/endpoints", json.dumps(endpoints))
    conf.insert_json5("scouting/multicast/enabled", "false")
    peer02 = zenoh.open(conf)

    return (peer01, peer02)


def close_session(peer01: zenoh.Session, peer02: zenoh.Session):
    peer01.close()
    peer02.close()


def test_timestamp_instrumentation_builder():
    """Test TimestampInstrumentationBuilder and TimestampInstrumentation."""
    builder = TimestampInstrumentationBuilder()
    assert builder is not None

    # Build with all points enabled
    instr = builder.set_send(True).set_route(True).set_receive(True).build()
    assert instr is not None
    assert isinstance(instr, TimestampInstrumentation)
    assert instr.is_instrumented(InterceptionPoint.SEND)
    assert instr.is_instrumented(InterceptionPoint.ROUTE)
    assert instr.is_instrumented(InterceptionPoint.RECEIVE)

    # Build with only send enabled
    instr2 = TimestampInstrumentationBuilder().set_send(True).build()
    assert instr2.is_instrumented(InterceptionPoint.SEND)
    assert not instr2.is_instrumented(InterceptionPoint.ROUTE)
    assert not instr2.is_instrumented(InterceptionPoint.RECEIVE)

    # Build with only route and receive
    instr3 = TimestampInstrumentationBuilder().set_route(True).set_receive(True).build()
    assert not instr3.is_instrumented(InterceptionPoint.SEND)
    assert instr3.is_instrumented(InterceptionPoint.ROUTE)
    assert instr3.is_instrumented(InterceptionPoint.RECEIVE)


def test_timestamp_instrumentation_builder_empty():
    """Test that building with no points raises an error."""
    try:
        TimestampInstrumentationBuilder().build()
        assert False, "Expected ZError for empty instrumentation"
    except zenoh.ZError:
        pass


def test_pubsub_timestamp_stack():
    """Test publishing with timestamp_instrumentation and reading from sample."""
    zenoh.try_init_log_from_env()
    peer01, peer02 = open_session(["tcp/127.0.0.1:17448"])

    keyexpr = "test/timestamp_stack"
    msg = b"hello with timestamps"

    received_sample = None

    def sub_callback(sample: Sample):
        nonlocal received_sample
        received_sample = sample

    publisher = peer01.declare_publisher(keyexpr)
    subscriber = peer02.declare_subscriber(keyexpr, sub_callback)
    time.sleep(SLEEP)

    # Test with timestamp_instrumentation
    instr = TimestampInstrumentationBuilder().set_send(True).set_receive(True).build()
    publisher.put(msg, timestamp_instrumentation=instr)

    time.sleep(SLEEP)
    assert received_sample is not None
    assert received_sample.timestamp_stack is not None
    assert isinstance(received_sample.timestamp_stack, TimestampStack)

    stack = received_sample.timestamp_stack
    assert stack.instrumentation is not None
    assert isinstance(stack.instrumentation, TimestampInstrumentation)
    assert stack.instrumentation.is_instrumented(InterceptionPoint.SEND)
    assert stack.instrumentation.is_instrumented(InterceptionPoint.RECEIVE)

    assert len(stack.records) > 0
    for record in stack.records:
        assert record.point in [
            InterceptionPoint.SEND,
            InterceptionPoint.ROUTE,
            InterceptionPoint.RECEIVE,
        ]
        # timestamp() returns either Timestamp or bytes
        ts = record.timestamp()
        assert ts is not None
        if record.is_custom:
            assert isinstance(ts, bytes)
        else:
            assert isinstance(ts, zenoh.Timestamp)

    # Test without timestamp_instrumentation - should be None
    received_sample = None
    publisher.put(msg)
    time.sleep(SLEEP)
    assert received_sample is not None
    assert received_sample.timestamp_stack is None

    publisher.undeclare()
    subscriber.undeclare()
    close_session(peer01, peer02)


def test_session_put_timestamp_stack():
    """Test Session.put() with timestamp_instrumentation."""
    zenoh.try_init_log_from_env()
    peer01, peer02 = open_session(["tcp/127.0.0.1:17449"])

    keyexpr = "test/session_timestamp_stack"
    msg = b"session put with timestamps"

    received_sample = None

    def sub_callback(sample: Sample):
        nonlocal received_sample
        received_sample = sample

    subscriber = peer02.declare_subscriber(keyexpr, sub_callback)
    time.sleep(SLEEP)

    instr = TimestampInstrumentationBuilder().set_send(True).set_receive(True).build()
    peer01.put(keyexpr, msg, timestamp_instrumentation=instr)

    time.sleep(SLEEP)
    assert received_sample is not None
    assert received_sample.timestamp_stack is not None
    assert isinstance(received_sample.timestamp_stack, TimestampStack)

    subscriber.undeclare()
    close_session(peer01, peer02)


def test_session_get_timestamp_stack():
    """Test Session.get() with timestamp_instrumentation."""
    zenoh.try_init_log_from_env()
    peer01, peer02 = open_session(["tcp/127.0.0.1:17450"])

    keyexpr = "test/get_timestamp_stack"

    def queryable_callback(query):
        # The query should have a timestamp_stack when instrumentation is enabled
        query.reply(keyexpr, b"reply")

    queryable = peer01.declare_queryable(keyexpr, queryable_callback)
    time.sleep(SLEEP)

    instr = TimestampInstrumentationBuilder().set_send(True).set_receive(True).build()
    replies = peer02.get(keyexpr, timestamp_instrumentation=instr)
    for reply in replies:
        sample = reply.ok
        if sample:
            assert sample.timestamp_stack is not None
            assert isinstance(sample.timestamp_stack, TimestampStack)
            stack = sample.timestamp_stack
            assert stack.instrumentation is not None
            assert isinstance(stack.instrumentation, TimestampInstrumentation)
            assert stack.instrumentation.is_instrumented(InterceptionPoint.SEND)
            assert stack.instrumentation.is_instrumented(InterceptionPoint.RECEIVE)
            assert len(stack.records) == 4

    queryable.undeclare()
    close_session(peer01, peer02)


def test_delete_timestamp_stack():
    """Test Publisher.delete() with timestamp_instrumentation."""
    zenoh.try_init_log_from_env()
    peer01, peer02 = open_session(["tcp/127.0.0.1:17451"])

    keyexpr = "test/delete_timestamp_stack"

    received_sample = None

    def sub_callback(sample: Sample):
        nonlocal received_sample
        received_sample = sample

    publisher = peer01.declare_publisher(keyexpr)
    subscriber = peer02.declare_subscriber(keyexpr, sub_callback)
    time.sleep(SLEEP)

    instr = TimestampInstrumentationBuilder().set_send(True).set_receive(True).build()
    publisher.delete(timestamp_instrumentation=instr)

    time.sleep(SLEEP)
    assert received_sample is not None
    assert received_sample.kind == zenoh.SampleKind.DELETE
    assert received_sample.timestamp_stack is not None
    assert isinstance(received_sample.timestamp_stack, TimestampStack)

    publisher.undeclare()
    subscriber.undeclare()
    close_session(peer01, peer02)


def test_querier_get_timestamp_stack():
    """Test Querier.get() with timestamp_instrumentation."""
    zenoh.try_init_log_from_env()
    peer01, peer02 = open_session(["tcp/127.0.0.1:17452"])

    keyexpr = "test/querier_timestamp_stack"

    def queryable_callback(query):
        query.reply(keyexpr, b"reply from querier test")

    queryable = peer01.declare_queryable(keyexpr, queryable_callback)
    time.sleep(SLEEP)

    querier = peer02.declare_querier(keyexpr)
    time.sleep(SLEEP)

    instr = TimestampInstrumentationBuilder().set_send(True).set_receive(True).build()
    replies = querier.get(timestamp_instrumentation=instr)
    for reply in replies:
        sample = reply.ok
        if sample:
            assert sample.timestamp_stack is not None
            assert isinstance(sample.timestamp_stack, TimestampStack)
            stack = sample.timestamp_stack
            assert stack.instrumentation is not None
            assert isinstance(stack.instrumentation, TimestampInstrumentation)
            assert stack.instrumentation.is_instrumented(InterceptionPoint.SEND)
            assert stack.instrumentation.is_instrumented(InterceptionPoint.RECEIVE)
            assert len(stack.records) == 4

    # Test without timestamp_instrumentation - should be None
    replies = querier.get()
    for reply in replies:
        sample = reply.ok
        if sample:
            assert sample.timestamp_stack is None

    querier.undeclare()
    queryable.undeclare()
    close_session(peer01, peer02)


def test_timestamp_callback():
    """Test Session open with a timestamp callback."""
    zenoh.try_init_log_from_env()

    contexts = []
    custom_timestamp = b"\xde\xad\xbe\xef"

    def timestamp_callback(ctx: TimestampContext):
        contexts.append(
            {
                "zid": str(ctx.zid),
                "whatami": ctx.whatami,
            }
        )
        return custom_timestamp

    conf = zenoh.Config()
    conf.insert_json5("listen/endpoints", json.dumps(["tcp/127.0.0.1:17453"]))
    conf.insert_json5("scouting/multicast/enabled", "false")
    peer01 = zenoh.open(conf, timestamp_callback=timestamp_callback)

    conf = zenoh.Config()
    conf.insert_json5("connect/endpoints", json.dumps(["tcp/127.0.0.1:17453"]))
    conf.insert_json5("scouting/multicast/enabled", "false")
    peer02 = zenoh.open(conf)

    keyexpr = "test/timestamp_callback"
    msg = b"hello with custom timestamps"

    received_sample = None

    def sub_callback(sample: Sample):
        nonlocal received_sample
        received_sample = sample

    publisher = peer01.declare_publisher(keyexpr)
    subscriber = peer02.declare_subscriber(keyexpr, sub_callback)
    time.sleep(SLEEP)

    instr = TimestampInstrumentationBuilder().set_send(True).set_receive(True).build()
    publisher.put(msg, timestamp_instrumentation=instr)

    time.sleep(SLEEP)
    assert received_sample is not None
    assert received_sample.timestamp_stack is not None
    assert isinstance(received_sample.timestamp_stack, TimestampStack)

    stack = received_sample.timestamp_stack
    assert stack.instrumentation is not None
    assert stack.instrumentation.is_instrumented(InterceptionPoint.SEND)
    assert stack.instrumentation.is_instrumented(InterceptionPoint.RECEIVE)

    assert len(stack.records) > 0

    # The callback was set on peer01, so timestamps generated on peer01
    # (Send and possibly Route) must be custom. The Receive timestamp is
    # generated on peer02, which has no callback, so it remains UHLC.
    custom_records = [r for r in stack.records if r.is_custom]
    assert len(custom_records) > 0
    for record in custom_records:
        assert record.timestamp() == custom_timestamp

    # The callback should have been invoked once per custom timestamp.
    assert len(contexts) == len(custom_records)
    for ctx in contexts:
        assert ctx["whatami"] == zenoh.WhatAmI.PEER

    publisher.undeclare()
    subscriber.undeclare()
    peer01.close()
    peer02.close()
