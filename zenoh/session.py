from typing import Union, Any

from .zenoh import _Session, _Config

from .keyexpr import KeyExpr
from .config import Config
from .enums import Priority, SampleKind, CongestionControl

class Session(_Session):
	def __new__(cls, config: Union[Config, Any] = None):
		if config is None:
			return super().__new__(cls)
		elif isinstance(config, _Config):
			return super().__new__(cls, config)
		else:
			return super().__new__(cls, Config.from_obj(config))
	
	def put(self, keyexpr: Union[str, KeyExpr], value, priority: Priority = None, congestion_control: CongestionControl = None, local_routing: bool = None, sample_kind: SampleKind = None):
		super().put(KeyExpr(keyexpr).inner, value, priority=priority, congestion_control=congestion_control, local_routing=local_routing, sample_kind=sample_kind)
	
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