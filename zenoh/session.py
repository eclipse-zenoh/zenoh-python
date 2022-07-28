from typing import Union, Any
import sys

from .zenoh import _Session, _Config, _Publisher, _Subscriber, _PullSubscriber

from .keyexpr import KeyExpr, IntoKeyExpr
from .config import Config
from .closures import IntoHandler, Handler, Receiver
from .enums import *
from .value import IntoValue, Value, Sample, Reply
from .queryable import Queryable, Query

class Publisher:
	def __init__(self, p: _Publisher):
		self._inner_ = p
	
	def put(self, value: IntoValue, encoding: Encoding = None):
		self._inner_.put(Value(value, encoding))

	def delete(self):
		self._inner_.delete()
	
	@property
	def key_expr(self) -> KeyExpr:
		return KeyExpr(self._inner_.key_expr)
	
	def undeclare(self):
		self._inner_ = None

class Subscriber:
	def __init__(self, s:_Subscriber, receiver = None):
		self._subscriber_ = s
		self.receiver = receiver
	
	def undeclare(self):
		self._subscriber_ = None

class PullSubscriber:
	def __init__(self, s:_PullSubscriber, receiver = None):
		self._subscriber_ = s
		self.receiver = receiver
	
	def pull(self):
		self._subscriber_.pull()
	
	def undeclare(self):
		self._subscriber_ = None

class Session(_Session):
	def __new__(cls, config: Union[Config, Any] = None):
		if config is None:
			return super().__new__(cls)
		elif isinstance(config, _Config):
			return super().__new__(cls, config)
		else:
			return super().__new__(cls, Config.from_obj(config))
	
	def put(self, keyexpr: IntoKeyExpr, value: IntoValue,
			priority: Priority = None, congestion_control: CongestionControl = None,
			local_routing: bool = None, sample_kind: SampleKind = None):
		value = Value.autoencode(value)
		keyexpr = KeyExpr(keyexpr)
		kwargs = dict()
		if priority is not None:
			kwargs['priority'] = priority
		if congestion_control is not None:
			kwargs['congestion_control'] = congestion_control
		if local_routing is not None:
			kwargs['local_routing'] = local_routing
		if sample_kind is not None:
			kwargs['sample_kind'] = sample_kind
		return super().put(keyexpr, value, **kwargs)
	
	def delete(self, keyexpr: IntoKeyExpr):
		return super().delete(KeyExpr(keyexpr))
	
	def get(self, selector: str, handler: IntoHandler, local_routing: bool = None, consolidation: QueryConsolidation = None, target: QueryTarget = None) -> Receiver:
		handler = Handler(handler, lambda x: Reply(x))
		kwargs = dict()
		if local_routing is not None:
			kwargs["local_routing"] = local_routing
		if consolidation is not None:
			kwargs["conconsolidation"] =consolidation
		if target is not None:
			kwargs["target"] = target
		super().get(selector, handler.closure, **kwargs)
		return handler.receiver
	
	def declare_keyexpr(self, keyexpr: IntoKeyExpr) -> KeyExpr:
		return KeyExpr(super().declare_keyexpr(KeyExpr(keyexpr)))
	
	def declare_queryable(self, keyexpr: IntoKeyExpr, handler: IntoHandler, complete: bool = None):
		handler = Handler(handler, lambda x: Query(x))
		kwargs = dict()
		if complete is not None:
			kwargs['complete'] = complete
		inner = super().declare_queryable(KeyExpr(keyexpr), handler.closure, **kwargs)
		return Queryable(inner, handler.receiver)
	
	def declare_publisher(self, keyexpr: IntoKeyExpr, 
			priority: Priority = None, congestion_control: CongestionControl = None,
			local_routing: bool = None):
		kwargs = dict()
		if priority is not None:
			kwargs['priority'] = priority
		if congestion_control is not None:
			kwargs['congestion_control'] = congestion_control
		if local_routing is not None:
			kwargs['local_routing'] = local_routing
		return Publisher(super().declare_publisher(KeyExpr(keyexpr), **kwargs))
	
	def declare_subscriber(self, keyexpr: IntoKeyExpr, handler: IntoHandler, reliability: Reliability = None, local: bool = None) -> Subscriber:
		handler = Handler(handler, lambda x: Sample._upgrade_(x))
		kwargs = dict()
		if reliability is not None:
			kwargs['reliability'] = reliability
		if local is not None:
			kwargs['local'] = local
		s = super().declare_subscriber(KeyExpr(keyexpr), handler.closure, **kwargs)
		return Subscriber(s, handler.receiver)
	
	def declare_pull_subscriber(self, keyexpr: IntoKeyExpr, handler: IntoHandler, reliability: Reliability=None, local=None) -> PullSubscriber:
		handler = Handler(handler, lambda x: Sample._upgrade_(x))
		kwargs = dict()
		if reliability is not None:
			kwargs['reliability'] = reliability
		if local is not None:
			kwargs['local'] = local
		s = super().declare_pull_subscriber(KeyExpr(keyexpr), handler.closure, **kwargs)
		return PullSubscriber(s, handler.receiver)
	
	def close(self):
		pass