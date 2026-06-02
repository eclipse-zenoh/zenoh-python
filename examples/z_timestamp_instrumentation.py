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
# Demonstrates opt-in end-to-end latency instrumentation.
#
# Run this example to see Send/Route/Receive timestamps on each message.
# The custom_callback variant shows how to inject your own clock bytes.
#
import time

import zenoh
from zenoh import InterceptionPoint, TimestampInstrumentation


def print_stack(stack):
    if stack is None:
        print("  (no timestamp stack)")
        return
    for rec in stack.records:
        ts = rec.as_timestamp()
        if ts is not None:
            print(f"  {rec.point.name:8s}  hlc={ts}  custom={rec.is_custom}")
        else:
            print(f"  {rec.point.name:8s}  raw={rec.timestamp().hex()}  custom={rec.is_custom}")


def example_put_subscribe(session):
    print("\n── put/subscribe with send+receive instrumentation ─────────────────")
    instr = TimestampInstrumentation(send=True, receive=True)
    received = []
    with session.declare_subscriber("demo/ts/**", lambda s: received.append(s)):
        time.sleep(0.05)
        session.put("demo/ts/hello", b"world", timestamp_instrumentation=instr)
        time.sleep(0.2)
    if received:
        print(f"Received sample on '{received[0].key_expr}':")
        print_stack(received[0].timestamp_stack)


def example_publisher_default(session):
    print("\n── publisher with default instrumentation ───────────────────────────")
    instr = TimestampInstrumentation(send=True, receive=True)
    received = []
    with session.declare_publisher("demo/ts/pub", timestamp_instrumentation=instr) as pub:
        with session.declare_subscriber("demo/ts/pub", lambda s: received.append(s)):
            time.sleep(0.05)
            pub.put(b"message-1")
            pub.put(b"message-2", timestamp_instrumentation=TimestampInstrumentation(send=True))
            time.sleep(0.2)
    for s in received:
        print(f"Received '{s.payload.to_string()}':")
        print_stack(s.timestamp_stack)


def example_custom_callback():
    print("\n── session with custom timestamp callback ───────────────────────────")
    import struct
    import time as _t

    def my_clock(ctx):
        # Return a simple 8-byte little-endian nanosecond timestamp.
        ns = int(_t.time_ns())
        return struct.pack("<Q", ns)

    instr = TimestampInstrumentation(send=True, receive=True)
    received = []
    with zenoh.open(zenoh.Config(), timestamp_callback=my_clock) as session:
        with session.declare_subscriber("demo/ts/custom", lambda s: received.append(s)):
            time.sleep(0.05)
            session.put("demo/ts/custom", b"data", timestamp_instrumentation=instr)
            time.sleep(0.2)
    if received:
        print("Received sample with custom timestamps:")
        print_stack(received[0].timestamp_stack)


def main(conf: zenoh.Config):
    zenoh.init_log_from_env_or("error")
    print("Opening session...")
    with zenoh.open(conf) as session:
        example_put_subscribe(session)
        example_publisher_default(session)
    example_custom_callback()
    print("\nDone.")


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="Zenoh timestamp instrumentation example")
    parser.add_argument("--config", "-c", type=str, help="Path to zenoh config file")
    args = parser.parse_args()

    conf = zenoh.Config.from_file(args.config) if args.config else zenoh.Config()
    main(conf)
