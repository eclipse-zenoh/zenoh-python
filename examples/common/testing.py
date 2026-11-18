#
# Copyright (c) 2026 ZettaScale Technology
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
#
"""Private loopback networks for executable examples and tests."""

import json
import random

import zenoh


def isolated_config(config=None):
    config = config if config is not None else zenoh.Config()
    for key, value in {
        "scouting/multicast/enabled": False,
        "scouting/gossip/enabled": False,
        "listen/endpoints": [],
        "connect/endpoints": [],
    }.items():
        config.insert_json5(key, json.dumps(value))
    return config


def open_listener(config=None, **options):
    # Keep the successful listener open: probing and releasing a port first
    # would allow another test process to bind it before Zenoh does.
    config = isolated_config(config)
    for _ in range(16):
        endpoint = f"tcp/127.0.0.1:{random.randrange(20000, 32768)}"
        config.insert_json5("listen/endpoints", json.dumps([endpoint]))
        config.insert_json5("listen/timeout_ms", "0")
        config.insert_json5("listen/exit_on_failure", "true")
        try:
            return zenoh.open(config, **options), endpoint
        except zenoh.ZError as error:
            message = str(error)
            # Windows uses WSAEADDRINUSE (10048), with localized error text.
            if (
                "Address already in use" not in message
                and "os error 10048" not in message
            ):
                raise
    raise RuntimeError("Could not bind a test listener after 16 attempts")


def connecting_config(endpoint):
    config = isolated_config()
    config.insert_json5("connect/endpoints", json.dumps([endpoint]))
    config.insert_json5("connect/timeout_ms", "5000")
    config.insert_json5("connect/exit_on_failure", "true")
    return config


def open_session_pair(**listener_options):
    listener, endpoint = open_listener(**listener_options)
    try:
        connector = zenoh.open(connecting_config(endpoint))
    except BaseException:
        listener.close()
        raise
    return listener, connector


def discovery_config():
    # Discovery tests need multicast, but must not see other test networks.
    # Use loopback and a private group/port, matching the C++ locator tests.
    config = isolated_config()
    address = (
        f"239.255.{random.randrange(256)}.{random.randrange(1, 255)}:"
        f"{random.randrange(20000, 32768)}"
    )
    config.insert_json5("mode", '"router"')
    config.insert_json5("listen/endpoints", '["tcp/127.0.0.1:0"]')
    config.insert_json5("scouting/multicast/enabled", "true")
    config.insert_json5("scouting/multicast/interface", '"127.0.0.1"')
    config.insert_json5("scouting/multicast/address", json.dumps(address))
    return config
