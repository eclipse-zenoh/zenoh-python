from zenoh import *

init_logger()
s: Session = Session({"mode": "peer"})
print(s)

print(KeyExpr("a/b"))
print(KeyExpr.autocanonize("a/b"))
print(KeyExpr.autocanonize("a/**/*"))