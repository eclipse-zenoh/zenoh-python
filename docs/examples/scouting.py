import zenoh
import threading

scout = zenoh.scout(what="peer|router")
threading.Timer(1.0, lambda: scout.stop()).start()
for hello in scout:
    print(hello)
