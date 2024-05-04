import zenoh
import json
from zenoh import Session, Query, Sample, Priority, CongestionControl
from typing import List, Tuple, Optional
import time

SLEEP = 1
MSG_COUNT = 1_000;
MSG_SIZE = [1_024, 131_072];


def open_session(endpoints: List[str]) -> Tuple[Session, Session]:
    # listen peer
    conf = zenoh.Config()
    conf.insert_json5("listen/endpoints", json.dumps(endpoints))
    conf.insert_json5("scouting/multicast/enabled", "false")
    print("[  ][01a] Opening peer01 session");
    peer01 = zenoh.open(conf)

    # connect peer
    conf = zenoh.Config()
    conf.insert_json5("connect/endpoints", json.dumps(endpoints))
    conf.insert_json5("scouting/multicast/enabled", "false")
    print("[  ][02a] Opening peer02 session");
    peer02 = zenoh.open(conf)

    return (peer01, peer02)


def close_session(peer01: Session, peer02: Session):
    print("[  ][01e] Closing peer01 session");
    peer01.close()
    print("[  ][02e] Closing peer02 session");
    peer02.close()


def run_session_qryrep(peer01: Session, peer02: Session):
    keyexpr = "test/session"

    for size in MSG_SIZE:
        num_requests = 0
        num_replies = 0
        num_errors = 0

        def queryable_callback(query: Query):
            nonlocal num_requests
            query.reply(Sample(keyexpr, bytes(size)))
            num_requests += 1

        print("[QR][01c] Queryable on peer01 session");
        queryable = peer01.declare_queryable(
            keyexpr,
            queryable_callback,
            complete=False
        )

        time.sleep(SLEEP)

        print(f"[QR][02c] Getting on peer02 session. {MSG_COUNT} msgs.");
        for _ in range(MSG_COUNT):
            replies = peer02.get(keyexpr, zenoh.Queue())
            for reply in replies.receiver:
                try:
                    unwraped_reply = reply.ok
                except:
                    unwraped_reply = None

                if unwraped_reply:
                    assert len(unwraped_reply.payload) == size
                    num_replies += 1
                else:
                    num_errors += 1

        time.sleep(SLEEP)
        print(f"[QR][02c] Got on peer02 session. {num_replies}/{MSG_COUNT} msgs.");
        assert num_replies == MSG_COUNT
        assert num_requests == MSG_COUNT
        assert num_errors == 0

        print("[QR][03c] Unqueryable on peer01 session");
        queryable.undeclare()


def run_session_pubsub(peer01: Session, peer02: Session):
    keyexpr = "test_pub/session"
    msg = 'Pub Message'.encode()

    num_received = 0
    num_errors = 0

    def sub_callback(sample: Sample):
        nonlocal num_received
        nonlocal num_errors
        if sample.key_expr != keyexpr \
                or sample.qos.priority != Priority.DATA_HIGH() \
                or sample.qos.congestion_control != CongestionControl.BLOCK() \
                or sample.payload != msg:
            num_errors += 1
        num_received += 1

    print("[PS][01d] Publisher on peer01 session");
    publisher = peer01.declare_publisher(
        keyexpr,
        Priority.DATA_HIGH(),
        CongestionControl.BLOCK()
    )
    time.sleep(SLEEP)

    print(f"[PS][02d] Subscriber on peer02 session. {MSG_COUNT} msgs.")
    subscriber = peer02.declare_subscriber(keyexpr, sub_callback)
    time.sleep(SLEEP)

    for _ in range(0, MSG_COUNT):
        publisher.put('Pub Message')

    time.sleep(SLEEP)
    print(f"[PS][02d] Received on peer02 session. {num_received}/{MSG_COUNT} msgs.");
    assert num_received == MSG_COUNT
    assert num_errors == 0

    print("[PS][03d] Undeclare publisher on peer01 session");
    publisher.undeclare()
    print("[PS][04d] Undeclare subscriber on peer02 session");
    subscriber.undeclare()


def run_session_attachment(peer01, peer02):
    keyexpr = "test_attachment/session"

    last_sample: Optional[Sample] = None

    def callback(sample: Sample):
        nonlocal last_sample
        last_sample = sample

    print("[A][01d] Publisher on peer01 session");
    publisher = peer01.declare_publisher(keyexpr)
    time.sleep(SLEEP)

    print("[A][02d] Publisher on peer01 session");
    subscriber = peer02.declare_subscriber(keyexpr, callback)
    time.sleep(SLEEP)

    publisher.put("no attachment")
    time.sleep(SLEEP)
    assert last_sample is not None
    assert last_sample.attachment is None

    publisher.put("attachment", attachment={"key1": "value1", b"key2": b"value2"})
    time.sleep(SLEEP)
    attachment = last_sample.attachment
    assert attachment is not None
    assert attachment
    assert len(attachment) == 2
    assert attachment.keys() == list(attachment) == [b"key1", b"key2"]
    assert attachment.values() == [b"value1", b"value2"]
    attachment.update([("key1", b"value1b")], key3="value3")
    assert attachment.items() == [(b"key1", b"value1"), (b"key2", b"value2"), (b"key1", b"value1b"), (b"key3", b"value3")]
    assert attachment.get("foo") is None

    publisher.put("attachment", attachment=[("key", "value")])
    time.sleep(SLEEP)
    assert dict(last_sample.attachment) == {b"key": b"value"}

    print("[A][03d] Undeclare publisher on peer01 session");
    publisher.undeclare()
    print("[A][04d] Undeclare subscriber on peer02 session");
    subscriber.undeclare()


def test_session():
    zenoh.init_logger()
    (peer01, peer02) = open_session(["tcp/127.0.0.1:17447"])
    run_session_qryrep(peer01, peer02)
    run_session_pubsub(peer01, peer02)
    run_session_attachment(peer01, peer02)
    close_session(peer01, peer02)
