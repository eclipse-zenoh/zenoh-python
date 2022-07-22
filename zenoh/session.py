from typing import Union, Any

from .zenoh import _Session

from .keyexpr import KeyExpr
from .config import Config
from .enums import Priority, SampleKind, CongestionControl

class Session:
	def __init__(self, config: Union[Config, Any] = None):
		if config is None:
			self.inner = _Session()
		elif isinstance(config, Config):
			self.inner = _Session(config)
		else:
			self.inner = _Session(Config.from_obj(config).inner)
	
	def put(self, keyexpr: Union[str, KeyExpr], value, priority: Priority = None, congestion_control: CongestionControl = None, local_routing: bool = None, sample_kind: SampleKind = None):
		self.inner.put(KeyExpr(keyexpr).inner, value, priority=priority, congestion_control=congestion_control, local_routing=local_routing, sample_kind=sample_kind)
	
	def delete(self, keyexpr: Union[str, KeyExpr]):
		raise NotImplemented()
	
	def get(self, keyexpr: Union[str, KeyExpr]):
		raise NotImplemented()
	
	def declare_keyexpr(self, keyexpr: Union[str, KeyExpr]):
		raise NotImplemented()
	
	def declare_queryable(self, keyexpr: Union[str, KeyExpr]):
		raise NotImplemented()
	
	def declare_publisher(self, keyexpr: Union[str, KeyExpr]):
		raise NotImplemented()
	
	def declare_subscriber(self, keyexpr: Union[str, KeyExpr]):
		raise NotImplemented()
	
	def declare_pull_subscriber(self, keyexpr: Union[str, KeyExpr]):
		raise NotImplemented()