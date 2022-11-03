#
# Copyright (c) 2022 ZettaScale Technology
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
from .zenoh import init_logger, scout as _scout
from .keyexpr import IntoKeyExpr, IntoSelector, KeyExpr, Selector
from .config import Config
from .session import Session, Publisher, Subscriber, PullSubscriber, Info
from .enums import CongestionControl, Encoding, Priority, QueryConsolidation, QueryTarget, Reliability, SampleKind
from .value import Hello, Value, IntoValue, IValue, Sample, IntoSample, ZenohId, Timestamp, Reply
from .closures import Closure, IClosure, IntoClosure, Handler, IHandler, IntoHandler, ListCollector, Queue
from .queryable import Queryable, Query
from typing import Any

def open(*args, **kwargs):
    return Session(*args, **kwargs)

class Scout:
    def __init__(self, inner, receiver):
        self._inner_ = inner
        self.receiver = receiver
    
    def stop(self):
        self._inner_ = None

def scout(handler: IntoHandler[Hello, Any, Any] = None, what: str = None, config: Config = None, timeout=None):
    from threading import Timer
    if handler is None:
        handler = ListCollector()
    handler = Handler(handler, lambda x: Hello._upgrade_(x))
    scout = _scout(handler.closure, config, what)
    scout = Scout(scout, handler.receiver)
    if timeout:
        Timer(timeout, lambda: scout.stop()).start()
    return scout