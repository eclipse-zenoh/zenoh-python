#
# Copyright (c) 2025 ZettaScale Technology
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
from typing import List, Tuple, TypeVar

import zenoh
from zenoh import CancellationToken, Reply, Session


def open_sessions(endpoints: List[str]) -> Tuple[Session, Session]:
    # listen peer
    conf = zenoh.Config()
    conf.insert_json5("listen/endpoints", json.dumps(endpoints))
    conf.insert_json5("scouting/multicast/enabled", "false")
    peer01 = zenoh.open(conf)

    # connect peer
    conf = zenoh.Config()
    conf.insert_json5("connect/endpoints", json.dumps(endpoints))
    conf.insert_json5("scouting/multicast/enabled", "false")
    peer02 = zenoh.open(conf)

    return (peer01, peer02)


def is_handler_closed(handler: zenoh.Handler[Reply]) -> bool:
    try:
        handler.try_recv()
        return False
    except:
        return True


def test_cancellation_get():
    keyexpr = "test/query_cancellation"
    session1, session2 = open_sessions(["tcp/127.0.0.1:50001"])
    queryable = session1.declare_queryable(keyexpr)
    time.sleep(1)

    print("Test that cancel drops callback")

    cancellation_token = CancellationToken()

    replies = session2.get(keyexpr, cancellation_token=cancellation_token)

    time.sleep(1)

    assert not is_handler_closed(replies)
    cancellation_token.cancel()
    assert is_handler_closed(replies)
    queryable.recv().drop()
    time.sleep(1)

    print("Test that cancel blocks until callback execution ends")
    cancellation_token = CancellationToken()
    n = 0

    def on_reply(reply: zenoh.Reply):
        print("Received")
        time.sleep(5)
        nonlocal n
        n += 10

    session2.get(keyexpr, on_reply, cancellation_token=cancellation_token)
    q = queryable.recv()
    q.reply(keyexpr, "ok")
    q.drop()
    time.sleep(1)

    assert not cancellation_token.is_cancelled
    cancellation_token.cancel()
    assert n == 10
    assert cancellation_token.is_cancelled

    print("Test that cancelled token cancels operation automatically")
    replies = session2.get(keyexpr, cancellation_token=cancellation_token)
    assert is_handler_closed(replies)


def test_cancellation_querier():
    keyexpr = "test/query_cancellation"
    session1, session2 = open_sessions(["tcp/127.0.0.1:50002"])
    queryable = session1.declare_queryable(keyexpr)
    querier = session2.declare_querier(keyexpr)
    time.sleep(1)

    print("Test that cancel drops callback")

    cancellation_token = CancellationToken()

    replies = querier.get(cancellation_token=cancellation_token)

    time.sleep(1)

    assert not is_handler_closed(replies)
    cancellation_token.cancel()
    assert is_handler_closed(replies)
    queryable.recv().drop()
    time.sleep(1)

    print("Test that cancel blocks until callback execution ends")
    cancellation_token = CancellationToken()
    n = 0

    def on_reply(reply: zenoh.Reply):
        print("Received")
        time.sleep(5)
        nonlocal n
        n += 10

    querier.get(on_reply, cancellation_token=cancellation_token)
    q = queryable.recv()
    q.reply(keyexpr, "ok")
    q.drop()
    time.sleep(1)

    assert not cancellation_token.is_cancelled
    cancellation_token.cancel()
    assert n == 10
    assert cancellation_token.is_cancelled

    print("Test that cancelled token cancels operation automatically")
    replies = querier.get(cancellation_token=cancellation_token)
    assert is_handler_closed(replies)


def test_cancellation_liveliness_get():
    keyexpr = "test/liveliness_query_cancellation"
    session1, session2 = open_sessions(["tcp/127.0.0.1:50003"])
    token = session1.liveliness().declare_token(keyexpr)
    time.sleep(1)

    print("Test that cancel blocks until callback execution ends")
    cancellation_token = CancellationToken()
    n = 0

    def on_reply(reply: zenoh.Reply):
        print("Received")
        time.sleep(5)
        nonlocal n
        n += 10

    session2.liveliness().get(keyexpr, on_reply, cancellation_token=cancellation_token)
    time.sleep(1)

    assert not cancellation_token.is_cancelled
    cancellation_token.cancel()
    assert n == 10
    assert cancellation_token.is_cancelled

    print("Test that cancelled token cancels operation automatically")
    replies = session2.liveliness().get(keyexpr, cancellation_token=cancellation_token)
    assert is_handler_closed(replies)
