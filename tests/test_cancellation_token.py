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
import gc
import time
import weakref

import zenoh
from zenoh import CancellationToken, Reply


def is_handler_closed(handler: zenoh.Handler[Reply]) -> bool:
    try:
        handler.try_recv()
        return False
    except:
        return True


def test_cancellation_get(session_pair):
    keyexpr = "test/query_cancellation"
    session1, session2 = session_pair()
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

    on_reply.session = session2
    callback_ref = weakref.ref(on_reply)
    session2.get(keyexpr, on_reply, cancellation_token=cancellation_token)
    del on_reply
    assert callback_ref() is not None
    q = queryable.recv()
    q.reply(keyexpr, "ok")
    q.drop()
    time.sleep(1)

    assert not cancellation_token.is_cancelled
    cancellation_token.cancel()
    assert n == 10
    assert cancellation_token.is_cancelled
    gc.collect()
    assert callback_ref() is None, "Reply callback retained after cancellation"

    print("Test that cancelled token cancels operation automatically")
    try:
        replies = session2.get(keyexpr, cancellation_token=cancellation_token)
        assert False, "Expected exception"
    except:
        pass


def test_cancellation_querier(session_pair):
    keyexpr = "test/query_cancellation"
    session1, session2 = session_pair()
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

    on_reply.resources = (session2, querier)
    callback_ref = weakref.ref(on_reply)
    querier.get(on_reply, cancellation_token=cancellation_token)
    del on_reply
    assert callback_ref() is not None
    q = queryable.recv()
    q.reply(keyexpr, "ok")
    q.drop()
    time.sleep(1)

    assert not cancellation_token.is_cancelled
    cancellation_token.cancel()
    assert n == 10
    assert cancellation_token.is_cancelled
    gc.collect()
    assert callback_ref() is None, "Querier callback retained after cancellation"

    print("Test that cancelled token cancels operation automatically")
    try:
        replies = querier.get(cancellation_token=cancellation_token)
        assert False, "Expected exception"
    except:
        pass


def test_cancellation_liveliness_get(session_pair):
    keyexpr = "test/liveliness_query_cancellation"
    session1, session2 = session_pair()
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

    on_reply.session = session2
    callback_ref = weakref.ref(on_reply)
    session2.liveliness().get(keyexpr, on_reply, cancellation_token=cancellation_token)
    del on_reply
    assert callback_ref() is not None
    time.sleep(1)

    assert not cancellation_token.is_cancelled
    cancellation_token.cancel()
    assert n == 10
    assert cancellation_token.is_cancelled
    gc.collect()
    assert callback_ref() is None, "Liveliness callback retained after cancellation"

    print("Test that cancelled token cancels operation automatically")
    try:
        replies = session2.liveliness().get(
            keyexpr, cancellation_token=cancellation_token
        )
        assert False, "Expected exception"
    except:
        pass
