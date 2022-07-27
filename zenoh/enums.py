from .zenoh import _Encoding, _SampleKind, _CongestionControl, _Priority, _Reliability, _QueryTarget, _QueryConsolidation

class Priority(_Priority):
	def __new__(cls, inner: _SampleKind):
		return super().__new__(cls, inner)
	@staticmethod
	def REAL_TIME() -> 'Priority':
		return Priority(_Priority.REAL_TIME)
	@staticmethod
	def REAL_TIME() -> 'Priority':
		return Priority(_Priority.REAL_TIME)
	@staticmethod
	def INTERACTIVE_HIGH() -> 'Priority':
		return Priority(_Priority.INTERACTIVE_HIGH)
	@staticmethod
	def INTERACTIVE_LOW() -> 'Priority':
		return Priority(_Priority.INTERACTIVE_LOW)
	@staticmethod
	def DATA_HIGH() -> 'Priority':
		return Priority(_Priority.DATA_HIGH)
	@staticmethod
	def DATA() -> 'Priority':
		return Priority(_Priority.DATA)
	@staticmethod
	def DATA_LOW() -> 'Priority':
		return Priority(_Priority.DATA_LOW)
	@staticmethod
	def BACKGROUND() -> 'Priority':
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
	def EMPTY() -> 'Encoding':
		return Encoding(_Encoding.EMPTY )
	@staticmethod
	def APP_OCTET_STREAM() -> 'Encoding':
		return Encoding(_Encoding.APP_OCTET_STREAM)
	@staticmethod
	def APP_CUSTOM() -> 'Encoding':
		return Encoding(_Encoding.APP_CUSTOM)
	@staticmethod
	def TEXT_PLAIN() -> 'Encoding':
		return Encoding(_Encoding.TEXT_PLAIN)
	@staticmethod
	def APP_PROPERTIES() -> 'Encoding':
		return Encoding(_Encoding.APP_PROPERTIES)
	@staticmethod
	def APP_JSON() -> 'Encoding':
		return Encoding(_Encoding.APP_JSON)
	@staticmethod
	def APP_SQL() -> 'Encoding':
		return Encoding(_Encoding.APP_SQL)
	@staticmethod
	def APP_INTEGER() -> 'Encoding':
		return Encoding(_Encoding.APP_INTEGER)
	@staticmethod
	def APP_FLOAT() -> 'Encoding':
		return Encoding(_Encoding.APP_FLOAT)
	@staticmethod
	def APP_XML() -> 'Encoding':
		return Encoding(_Encoding.APP_XML)
	@staticmethod
	def APP_XHTML_XML() -> 'Encoding':
		return Encoding(_Encoding.APP_XHTML_XML)
	@staticmethod
	def APP_X_WWW_FORM_URLENCODED() -> 'Encoding':
		return Encoding(_Encoding.APP_X_WWW_FORM_URLENCODED)
	@staticmethod
	def TEXT_JSON() -> 'Encoding':
		return Encoding(_Encoding.TEXT_JSON)
	@staticmethod
	def TEXT_HTML() -> 'Encoding':
		return Encoding(_Encoding.TEXT_HTML)
	@staticmethod
	def TEXT_XML() -> 'Encoding':
		return Encoding(_Encoding.TEXT_XML)
	@staticmethod
	def TEXT_CSS() -> 'Encoding':
		return Encoding(_Encoding.TEXT_CSS)
	@staticmethod
	def TEXT_CSV() -> 'Encoding':
		return Encoding(_Encoding.TEXT_CSV)
	@staticmethod
	def TEXT_JAVASCRIPT() -> 'Encoding':
		return Encoding(_Encoding.TEXT_JAVASCRIPT)
	@staticmethod
	def IMAGE_JPEG() -> 'Encoding':
		return Encoding(_Encoding.IMAGE_JPEG)
	@staticmethod
	def IMAGE_PNG() -> 'Encoding':
		return Encoding(_Encoding.IMAGE_PNG)
	@staticmethod
	def IMAGE_GIF() -> 'Encoding':
		return Encoding(_Encoding.IMAGE_GIF)

class Reliability(_Reliability):
	def __new__(cls, inner: _Reliability):
		return super().__new__(cls, inner)
	@staticmethod
	def BEST_EFFORT() -> 'CongestionControl':
		return Reliability(_Reliability.BEST_EFFORT)
	@staticmethod
	def RELIABLE() -> 'CongestionControl':
		return Reliability(_Reliability.RELIABLE)

class QueryTarget(_QueryTarget):
	def __new__(cls, inner: _QueryTarget):
		return super().__new__(cls, inner)
	@staticmethod
	def BEST_MATCHING() -> 'QueryTarget':
		return QueryTarget(_QueryTarget.BEST_MATCHING)
	@staticmethod
	def ALL() -> 'QueryTarget':
		return QueryTarget(_QueryTarget.ALL)
	@staticmethod
	def ALL_COMPLETE() -> 'QueryTarget':
		return QueryTarget(_QueryTarget.ALL_COMPLETE)

class QueryConsolidation(_QueryConsolidation):
	def __new__(cls, inner: _QueryConsolidation):
		return super().__new__(cls, inner)
	@staticmethod
	def AUTO() -> 'QueryConsolidation':
		return QueryConsolidation(_QueryConsolidation.AUTO)
	@staticmethod
	def NONE() -> 'QueryConsolidation':
		return QueryConsolidation(_QueryConsolidation.NONE)
	@staticmethod
	def LAZY() -> 'QueryConsolidation':
		return QueryConsolidation(_QueryConsolidation.LAZY)
	@staticmethod
	def RECEPTION() -> 'QueryConsolidation':
		return QueryConsolidation(_QueryConsolidation.RECEPTION)
	@staticmethod
	def LAST_ROUTER() -> 'QueryConsolidation':
		return QueryConsolidation(_QueryConsolidation.LAST_ROUTER)
	@staticmethod
	def FULL() -> 'QueryConsolidation':
		return QueryConsolidation(_QueryConsolidation.FULL)