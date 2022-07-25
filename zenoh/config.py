from typing import Union
from .zenoh import _Config
import json

class Config(_Config):
	def __init__(self):
		super().__init__()
	@staticmethod
	def from_file(filename: str):
		c = super(Config, Config).from_file(filename)
		return c
	@staticmethod
	def from_obj(obj):
		c = Config.from_json5(json.dumps(obj))
		return c
	@staticmethod
	def from_json5(json: str):
		c =  super(Config, Config).from_json5(json)
		return c
	
	def get_json(self, path: str) -> str:
		return super().get_json(path)
	
	def insert_json5(self, path: str, value: str) -> str:
		return super().insert_json5(path, value)