from typing import Union, Any

from .zenoh import _Session
from .config import Config

class Session:
	def __init__(self, config: Union[Config, Any] = None):
		if config is None:
			self.inner = _Session()
		elif isinstance(config, Config):
			self.inner = _Session(config)
		else:
			self.inner = _Session(Config.from_obj(config).inner)