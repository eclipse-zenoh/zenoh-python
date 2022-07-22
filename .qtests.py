from zenoh import Config, Session, init_logger

init_logger()
s = Session({"mode": "peer"})
print(s)