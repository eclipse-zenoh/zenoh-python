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
from typing import Union, Any, List

from .zenoh import _Session, _Config, _Publisher, _Subscriber, _PullSubscriber

from .keyexpr import KeyExpr, IntoKeyExpr, Selector, IntoSelector
from .config import Config
from .closures import IntoHandler, Handler, Receiver
from .enums import *
from .value import IntoValue, Value, Sample, Reply, ZenohId
from .queryable import Queryable, Query


class Publisher:
    "Use `Publisher`s (constructed with `Session.declare_publisher`) when you want to send values often for the same key expression, as declaring them informs Zenoh that this is you intent, and optimizations will be set up to do so."

    def __init__(self, p: _Publisher):
        self._inner_ = p

    def put(self, value: IntoValue, encoding: Encoding = None):
        "An optimised version of `session.put(self.key_expr, value, encoding=encoding)"
        self._inner_.put(Value(value, encoding))

    def delete(self):
        "An optimised version of `session.delete(self.key_expr)"
        self._inner_.delete()

    @property
    def key_expr(self) -> KeyExpr:
        "This `Publisher`'s key expression"
        return KeyExpr(self._inner_.key_expr)

    def undeclare(self):
        "Stops the publisher."
        self._inner_ = None


class Subscriber:
    """
    A handle to a subscription.

    Its main purpose is to keep the subscription active as long as it exists.

    When constructed through `Session.declare_subscriber(session, keyexpr, handler)`, it exposes `handler`'s receiver
    through `self.receiver`.
    """

    def __init__(self, s: _Subscriber, receiver=None):
        self._subscriber_ = s
        self.receiver = receiver

    def undeclare(self):
        "Undeclares the subscription"
        self._subscriber_ = None


class PullSubscriber:
    """
    A handle to a pull subscription.

    Its main purpose is to keep the subscription active as long as it exists.

    When constructed through `Session.declare_pull_subscriber(session, keyexpr, handler)`, it exposes `handler`'s receiver
    through `self.receiver`.

    Calling `self.pull()` will prompt the Zenoh network to send a new sample when available.
    """

    def __init__(self, s: _PullSubscriber, receiver=None):
        self._subscriber_ = s
        self.receiver = receiver

    def pull(self):
        """
        Prompts the Zenoh network to send a new sample if available.
        Note that this sample will not be returned by this function, but provided to the handler's callback.
        """
        self._subscriber_.pull()

    def undeclare(self):
        "Undeclares the subscription"
        self._subscriber_ = None


class Session(_Session):
    """
    A Zenoh Session, the core interraction point with a Zenoh network.
    """
    def __new__(cls, config: Union[Config, Any] = None):
        if config is None:
            return super().__new__(cls)
        elif isinstance(config, _Config):
            return super().__new__(cls, config)
        else:
            return super().__new__(cls, Config.from_obj(config))

    def put(self, keyexpr: IntoKeyExpr, value: IntoValue, encoding=None,
            priority: Priority = None, congestion_control: CongestionControl = None,
            sample_kind: SampleKind = None):
        """
        Sends a value over Zenoh.
        """
        value = Value(value, encoding)
        keyexpr = KeyExpr(keyexpr)
        kwargs = dict()
        if priority is not None:
            kwargs['priority'] = priority
        if congestion_control is not None:
            kwargs['congestion_control'] = congestion_control
        if sample_kind is not None:
            kwargs['sample_kind'] = sample_kind
        return super().put(keyexpr, value, **kwargs)

    def config(self) -> Config:
        """
        Returns a configuration object that can be used to alter the session's configuration at runtime.

        Note that in Python specifically, the config you passed to the session becomes the result of this
        function if you passed one, letting you keep using it.
        """
        return super().config()

    def delete(self, keyexpr: IntoKeyExpr,
               priority: Priority = None, congestion_control: CongestionControl = None):
        """
        Deletes a value.
        """
        keyexpr = KeyExpr(keyexpr)
        kwargs = dict()
        if priority is not None:
            kwargs['priority'] = priority
        if congestion_control is not None:
            kwargs['congestion_control'] = congestion_control
        return super().delete(keyexpr, **kwargs)

    def get(self, selector: IntoSelector, handler: IntoHandler[Reply, Any, Receiver], consolidation: QueryConsolidation = None, target: QueryTarget = None, value: IntoValue = None) -> Receiver:
        """
        Emits a query.
        """
        handler = Handler(handler, lambda x: Reply(x))
        kwargs = dict()
        if consolidation is not None:
            kwargs["conconsolidation"] = consolidation
        if target is not None:
            kwargs["target"] = target
        if value is not None:
            kwargs["value"] = Value(value)
        super().get(Selector(selector), handler.closure, **kwargs)
        return handler.receiver

    def declare_keyexpr(self, keyexpr: IntoKeyExpr) -> KeyExpr:
        """
        Informs Zenoh that you intend to use the provided Key Expression repeatedly.

        This function returns an optimized representation of the passed `keyexpr`.
        """
        return KeyExpr(super().declare_keyexpr(KeyExpr(keyexpr)))

    def declare_queryable(self, keyexpr: IntoKeyExpr, handler: IntoHandler[Query, Any, Any], complete: bool = None):
        """
        Declares a queryable, which will receive queries intersecting with `keyexpr`.

        These queries are passed to the `handler` as instances of the `Query`class, which lets you respond when applicatble.

        The `handler`'s receiver is returned as the `receiver` field of the return value.

        IMPORTANT: due to how RAII and Python work, you MUST bind this function's return value to a variable in order for it to function as expected.
        This is because as soon as a value is no longer referenced in Python, that value's destructor will run, which will undeclare your queryable, stopping it immediately.
        """
        handler = Handler(handler, lambda x: Query(x))
        kwargs = dict()
        if complete is not None:
            kwargs['complete'] = complete
        inner = super().declare_queryable(KeyExpr(keyexpr), handler.closure, **kwargs)
        return Queryable(inner, handler.receiver)

    def declare_publisher(self, keyexpr: IntoKeyExpr, priority: Priority = None, congestion_control: CongestionControl = None):
        """
        Declares a publisher, which you may use to send values repeatedly onto a same key expression.
        """
        kwargs = dict()
        if priority is not None:
            kwargs['priority'] = priority
        if congestion_control is not None:
            kwargs['congestion_control'] = congestion_control
        return Publisher(super().declare_publisher(KeyExpr(keyexpr), **kwargs))

    def declare_subscriber(self, keyexpr: IntoKeyExpr, handler: IntoHandler[Sample, Any, Any], reliability: Reliability = None) -> Subscriber:
        """
        Declares a subscriber, which will receive any published sample with a key expression intersecting `keyexpr`.

        These samples are passed to the `handler`'s closure as instances of the `Sample` class.

        The `handler`'s receiver is returned as the `receiver` field of the return value.

        IMPORTANT: due to how RAII and Python work, you MUST bind this function's return value to a variable in order for it to function as expected.
        This is because as soon as a value is no longer referenced in Python, that value's destructor will run, which will undeclare your subscriber, deactivating the subscription immediately.
        """
        handler = Handler(handler, lambda x: Sample._upgrade_(x))
        kwargs = dict()
        if reliability is not None:
            kwargs['reliability'] = reliability
        s = super().declare_subscriber(KeyExpr(keyexpr), handler.closure, **kwargs)
        return Subscriber(s, handler.receiver)

    def declare_pull_subscriber(self, keyexpr: IntoKeyExpr, handler: IntoHandler[Sample, Any, Any], reliability: Reliability = None) -> PullSubscriber:
        """
        Declares a pull-mode subscriber, which will receive a single published sample with a key expression intersecting `keyexpr` any time its `pull` method is called.

        These samples are passed to the `handler`'s closure as instances of the `Sample` class.

        The `handler`'s receiver is returned as the `receiver` field of the return value.
        """
        handler = Handler(handler, lambda x: Sample._upgrade_(x))
        kwargs = dict()
        if reliability is not None:
            kwargs['reliability'] = reliability
        s = super().declare_pull_subscriber(KeyExpr(keyexpr), handler.closure, **kwargs)
        return PullSubscriber(s, handler.receiver)

    def close(self):
        "Closes the Session"
        pass

    def info(self):
        "Returns an accessor for informations about this Session"
        return Info(self)


class Info:
    def __init__(self, session: _Session):
        self.session = session

    def zid(self) -> ZenohId:
        "Returns this Zenoh Session's identifier"
        return ZenohId._upgrade_(self.session.zid())

    def routers_zid(self) -> List[ZenohId]:
        "Returns the neighbooring routers' identifiers"
        return [ZenohId._upgrade_(id) for id in self.session.routers_zid()]

    def peers_zid(self) -> List[ZenohId]:
        "Returns the neighbooring peers' identifiers"
        return [ZenohId._upgrade_(id) for id in self.session.peers_zid()]
