from typing import Union
from .zenoh import _Config
import json

class Config:
	def __init__(self):
		self.inner = _Config()
	@staticmethod
	def from_file(obj):
		c = Config()
		c.inner = _Config.from_file(json.dumps(obj))
		return c
	@staticmethod
	def from_obj(obj):
		c = Config()
		c.inner = _Config.from_json5(json.dumps(obj))
		return c
	@staticmethod
	def from_json5(json: str):
		c = Config()
		c.inner = _Config.from_json5(json)
		return c
	
	def get_json(self, path: str) -> str:
		return self.inner.get_json(path)
	
	def insert_json5(self, path: str, value: str) -> str:
		return self.inner.insert_json5(path, value)