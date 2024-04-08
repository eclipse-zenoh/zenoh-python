from zenoh.config import ZenohId

class SessionInfo:
    def zid(self) -> ZenohId:
        """Return the ZenohId of the current zenoh Session."""

    def routers_zid(self) -> list[ZenohId]:
        """Return the ZenohId of the zenoh routers this process is currently connected to or the ZenohId of the current router if this code is run from a router (plugin)."""

    def peers_zid(self) -> list[ZenohId]:
        """Return the ZenohId of the zenoh peers this process is currently connected to."""
