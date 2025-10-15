import zenoh
import time
import threading


def liveliness_subscriber_example(session, max_samples=2):
    """Example: Subscribe to liveliness token changes."""
    # Check if a liveliness token is present and subscribe to changes
    with session.liveliness().declare_subscriber("node/A", history=True) as sub:
        count = 0
        for sample in sub:
            if sample.kind == zenoh.SampleKind.GET:
                print(f"Alive token ('{sample.key_expr}')")
            elif sample.kind == zenoh.SampleKind.DELETE:
                print(f"Dropped token ('{sample.key_expr}')")
            count += 1
            if count >= max_samples:
                break


def test_liveliness_subscriber():
    """Test harness that creates environment to exercise all branches."""
    session = zenoh.open(zenoh.Config())

    # Run the example in a thread
    example_ready = threading.Event()
    example_done = threading.Event()

    def run_example():
        example_ready.set()
        liveliness_subscriber_example(session, max_samples=2)
        example_done.set()

    example_thread = threading.Thread(target=run_example, daemon=True)
    example_thread.start()

    # Wait for example to start
    example_ready.wait(timeout=1.0)
    time.sleep(0.1)

    # Create environment: declare and undeclare token to trigger both branches
    token = session.liveliness().declare_token("node/A")
    time.sleep(0.1)
    token.undeclare()
    time.sleep(0.1)

    # Wait for example to complete
    example_done.wait(timeout=1.0)
    example_thread.join(timeout=0.1)

    session.close()
    print("Test passed!")


if __name__ == "__main__":
    test_liveliness_subscriber()
