from os import path
from subprocess import Popen, PIPE
import time

examples = path.dirname(path.realpath(__file__))
tab = "\t"
ret = "\r\n"
KILL = -9

class Pyrun:
	def __init__(self, p, args=None) -> None:
		if args is None:
			args = []
		self.name = p
		print(f"starting {self.name}")
		self.process: Popen = Popen(["python3", path.join(examples, p), *args], stdin=PIPE, stdout=PIPE, stderr=PIPE)
		self.start = time.time()
		self.end = None
		self._stdouts = []
		self._stderrs = []
	def dbg(self):
		self.wait()
		print("stdout:")
		print(f"{tab}{tab.join(self.stdout)}")
		print("stderr:")
		print(f"{tab}{tab.join(self.stderr)}")
	def status(self, expecting=0, do_print=True):
		status = self.wait()
		formatted = f"{self.name}: returned {status} (expected {expecting}) - {self.time:.2}s"
		if do_print:
			print(formatted)
		return formatted if status != expecting else None
	def wait(self):
		code = self.process.wait()
		if self.end is None:
			self.end = time.time()
		return code
	@property
	def stdout(self):
		self._stdouts.extend(line.decode('utf8') for line in self.process.stdout.readlines())
		return self._stdouts
	@property
	def stderr(self):
		self._stderrs.extend(line.decode('utf8') for line in self.process.stderr.readlines())
		return self._stderrs
	@property
	def time(self):
		return None if self.end is None else (self.end - self.start)

errors = []

info = Pyrun("z_info.py")
if info.status():
	info.dbg()
	errors.append(info.status())
scout = Pyrun("z_scout.py")
if scout.status():
	scout.dbg()
	errors.append(scout.status())

storage = Pyrun("z_storage.py")
sub = Pyrun("z_sub.py")
pull = Pyrun("z_pull.py")
time.sleep(2)
write = Pyrun("z_write.py")
if write.status():
	write.dbg()
	errors.append(write.status())
pub = Pyrun("z_pub.py", ["--iter=2"])
try:
	pull.process.stdin.write(b"\n")
	pull.process.stdin.flush()
	pull.process.stdin.write(b"q\n")
	pull.process.stdin.flush()
	pull.process.stdin.close()
except Exception as e:
	pull.dbg()
	errors.append(f"pull stdin sequence failed: {e}")
if pub.status():
	pub.dbg()
	errors.append(pub.status())
if pull.process.poll() is not None:
	pull.status()
	pull.dbg()
	if pull.wait():
		errors.append(pull.status())
else:
	pull.process.kill()
	pull.dbg()
	if status:=pull.status(KILL):
		errors.append(status)
	else:
		errors.append("z_pull: didn't return (bad), but was killed properly (not terrible)")
try:
	sub.process.stdin.write(b"q\n")
	sub.process.stdin.flush()
	sub.process.stdin.close()
except Exception as e:
	errors.append(f"pub stdin sequence failed: {e}")
if sub.status():
	sub.dbg()
	errors.append(sub.status())
subout = "".join(sub.stdout)
if not ("Received ('/demo/example/zenoh-python-write': 'Write from Python!')" in subout):
	errors.append("z_sub didn't catch z_write")
if not ("Received ('/demo/example/zenoh-python-pub': '[   1] Pub from Python!')" in subout):
	errors.append("z_sub didn't catch second z_pub")
if any(("z_sub" in error) for error in errors):
	sub.dbg()

eval = Pyrun("z_eval.py", ["-p=/demo/eval/zenoh-python-eval"])
time.sleep(3)
query = Pyrun("z_query.py", ["-s=/demo/eval/zenoh-python-eval"])
if query.status():
	query.dbg()
	errors.append(query.status())
try:
	eval.process.stdin.write(b"q\n")
	eval.process.stdin.flush()
	eval.process.stdin.close()
except Exception as e:
	errors.append(f"eval stdin sequence failed: {e}")
if eval.status():
	eval.dbg()
	errors.append(eval.status())
if not ("received (/demo/eval/zenoh-python-eval:Eval from Python!)" in "".join(query.stdout)):
	query.dbg()
	eval.dbg()
	errors.append("z_query didn't get a response from z_eval")

query = Pyrun("z_query.py", ["-s=/demo/example/zenoh-python-write"])
if query.status():
	query.dbg()
	errors.append(query.status())
try:
	storage.process.stdin.write(b"q\n")
	storage.process.stdin.flush()
	storage.process.stdin.close()
except Exception as e:
	errors.append(f"storage stdin sequence failed: {e}")
if storage.status():
	storage.dbg()
	errors.append(storage.status())
if not ("received (/demo/example/zenoh-python-write:Write from Python!)" in "".join(query.stdout)):
	storage.dbg()
	errors.append("z_query didn't get a response from z_storage about z_write")
if any(("z_query" in error) for error in errors):
	query.dbg()

sub_thr = Pyrun("z_sub_thr.py")
pub_thr = Pyrun("z_pub_thr.py", ["128"])
time.sleep(5)
sub_thr.process.kill()
pub_thr.process.kill()
if sub_thr.status(KILL):
	sub_thr.dbg()
	errors.append(sub_thr.status())
if pub_thr.status(KILL):
	pub_thr.dbg()
	errors.append(pub_thr.status())


if len(errors):
	message = f"Found {len(errors)} errors: {(ret+tab) + (ret+tab).join(errors)}"
	raise Exception(message)