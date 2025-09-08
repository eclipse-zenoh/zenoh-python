try:
    from zenoh._shm import *
except ImportError:
    import warnings

    raise ModuleNotFoundError(
        "No module named 'zenoh.shm'.\nzenoh must be built wit shared-memory feature to enable it."
    ) from None
