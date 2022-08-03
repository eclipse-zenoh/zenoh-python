from .zenoh import init_logger, scout as _scout
from .keyexpr import IntoKeyExpr, IntoSelector, KeyExpr, Selector
from .config import Config
from .session import Session, Publisher, Subscriber, PullSubscriber, Info
from .enums import CongestionControl, Encoding, Priority, QueryConsolidation, QueryTarget, Reliability, SampleKind
from .value import Hello, Value, IntoValue, IValue, Sample, IntoSample, ZenohId, Timestamp, Reply
from .closures import Closure, IClosure, IntoClosure, Handler, IHandler, IntoHandler, ListCollector, Queue
from .queryable import Queryable, Query

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
    handler = Handler(handler, lambda x: Hello._upgrade_)
    scout = _scout(handler.closure, config, what)
    scout = Scout(scout, handler.receiver)
    if timeout:
        Timer(timeout, lambda: scout.stop()).start()
    return scout