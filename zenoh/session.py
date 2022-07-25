from typing import Union, Any

from .zenoh import _Session, _Config

from .keyexpr import KeyExpr
from .config import Config
from .enums import Priority, SampleKind, CongestionControl
from .value import IntoValue, Value

class Session(_Session):
	def __new__(cls, config: Union[Config, Any] = None):
		if config is None:
			return super().__new__(cls)
		elif isinstance(config, _Config):
			return super().__new__(cls, config)
		else:
			return super().__new__(cls, Config.from_obj(config))
	
	def put(self, keyexpr: Union[str, KeyExpr], value: IntoValue, priority: Priority = None, congestion_control: CongestionControl = None, local_routing: bool = None, sample_kind: SampleKind = None):
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
	
	def delete(self, keyexpr: Union[str, KeyExpr]):
		return super().delete(KeyExpr(keyexpr))
	
	def get(self, keyexpr: Union[str, KeyExpr]):
		raise NotImplemented()
	
	def declare_keyexpr(self, keyexpr: Union[str, KeyExpr]) -> 'KeyExpr':
		return super().declare_keyexpr(KeyExpr(keyexpr))
	
	def declare_queryable(self, keyexpr: Union[str, KeyExpr]):
		raise NotImplemented()
	
	def declare_publisher(self, keyexpr: Union[str, KeyExpr]):
		raise NotImplemented()
	
	def declare_subscriber(self, keyexpr: Union[str, KeyExpr], ):
		raise NotImplemented()
	
	def declare_pull_subscriber(self, keyexpr: Union[str, KeyExpr]):
		raise NotImplemented()
	
	def close(self):
		pass