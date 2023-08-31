import zenoh
import json
from zenoh import Session, Query, Sample
from typing import List, Tuple
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
    print("[  ][01d] Closing peer01 session");
    peer01.close()
    print("[  ][02d] Closing peer02 session");
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


def test_session():
    zenoh.init_logger()
    (peer01, peer02) = open_session(["tcp/127.0.0.1:17447"])
    run_session_qryrep(peer01, peer02)
    close_session(peer01, peer02)
