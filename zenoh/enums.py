from .zenoh import _Encoding, _SampleKind, _CongestionControl, _Priority

class Priority(_Priority):
	def __new__(cls, inner: _SampleKind):
		return super().__new__(cls, inner)
	@staticmethod
	def REAL_TIME():
		return Priority(_Priority.REAL_TIME)
	@staticmethod
	def REAL_TIME():
		return Priority(_Priority.REAL_TIME)
	@staticmethod
	def INTERACTIVE_HIGH():
		return Priority(_Priority.INTERACTIVE_HIGH)
	@staticmethod
	def INTERACTIVE_LOW():
		return Priority(_Priority.INTERACTIVE_LOW)
	@staticmethod
	def DATA_HIGH():
		return Priority(_Priority.DATA_HIGH)
	@staticmethod
	def DATA():
		return Priority(_Priority.DATA)
	@staticmethod
	def DATA_LOW():
		return Priority(_Priority.DATA_LOW)
	@staticmethod
	def BACKGROUND():
		return Priority(_Priority.BACKGROUND)

class SampleKind(_SampleKind):
	def __new__(cls, inner: _SampleKind):
		return super().__new__(cls, inner)
	@staticmethod
	def PUT() -> 'SampleKind':
		return SampleKind(_SampleKind.PUT)
	@staticmethod
	def DELETE() -> 'SampleKind':
		return SampleKind(_SampleKind.DELETE)

class CongestionControl(_CongestionControl):
	def __new__(cls, inner: _CongestionControl):
		return super().__new__(cls, inner)
	@staticmethod
	def BLOCK() -> 'CongestionControl':
		return CongestionControl(_CongestionControl.BLOCK)
	@staticmethod
	def DROP() -> 'CongestionControl':
		return CongestionControl(_CongestionControl.DROP)

class Encoding(_Encoding):
	def __new__(cls, inner: _Encoding):
		return super().__new__(cls, inner)
	@staticmethod
	def from_str(s: str) -> 'Encoding':
		return super(Encoding, Encoding).from_str(s)
	def append(self, s: str):
		super().append(s)
	def __eq__(self, other: 'Encoding'):
		return super().equals(other)
	@staticmethod
	def EMPTY():
		return Encoding(_Encoding.EMPTY )
	@staticmethod
	def APP_OCTET_STREAM():
		return Encoding(_Encoding.APP_OCTET_STREAM)
	@staticmethod
	def APP_CUSTOM():
		return Encoding(_Encoding.APP_CUSTOM)
	@staticmethod
	def TEXT_PLAIN():
		return Encoding(_Encoding.TEXT_PLAIN)
	@staticmethod
	def APP_PROPERTIES():
		return Encoding(_Encoding.APP_PROPERTIES)
	@staticmethod
	def APP_JSON():
		return Encoding(_Encoding.APP_JSON)
	@staticmethod
	def APP_SQL():
		return Encoding(_Encoding.APP_SQL)
	@staticmethod
	def APP_INTEGER():
		return Encoding(_Encoding.APP_INTEGER)
	@staticmethod
	def APP_FLOAT():
		return Encoding(_Encoding.APP_FLOAT)
	@staticmethod
	def APP_XML():
		return Encoding(_Encoding.APP_XML)
	@staticmethod
	def APP_XHTML_XML():
		return Encoding(_Encoding.APP_XHTML_XML)
	@staticmethod
	def APP_X_WWW_FORM_URLENCODED():
		return Encoding(_Encoding.APP_X_WWW_FORM_URLENCODED)
	@staticmethod
	def TEXT_JSON():
		return Encoding(_Encoding.TEXT_JSON)
	@staticmethod
	def TEXT_HTML():
		return Encoding(_Encoding.TEXT_HTML)
	@staticmethod
	def TEXT_XML():
		return Encoding(_Encoding.TEXT_XML)
	@staticmethod
	def TEXT_CSS():
		return Encoding(_Encoding.TEXT_CSS)
	@staticmethod
	def TEXT_CSV():
		return Encoding(_Encoding.TEXT_CSV)
	@staticmethod
	def TEXT_JAVASCRIPT():
		return Encoding(_Encoding.TEXT_JAVASCRIPT)
	@staticmethod
	def IMAGE_JPEG():
		return Encoding(_Encoding.IMAGE_JPEG)
	@staticmethod
	def IMAGE_PNG():
		return Encoding(_Encoding.IMAGE_PNG)
	@staticmethod
	def IMAGE_GIF():
		return Encoding(_Encoding.IMAGE_GIF)

