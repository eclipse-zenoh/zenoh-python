# Zenoh Python examples

## Start instructions

   ```bash
   python3 <example.py>
   ```

   Each example accepts the `-h` or `--help` option that provides a description of its arguments and their default values.

   If you run the tests against the zenoh router running in a Docker container, you need to add the
   `-e tcp/localhost:7447` option to your examples. That's because Docker doesn't support UDP multicast
   transport, and therefore the zenoh scouting and discrovery mechanism cannot work with.

## Examples description

### z_scout

   Scouts for zenoh peers and routers available on the network.

   Typical usage:
   ```bash
      python3 z_scout.py
   ```

### z_info

   Gets information about the Zenoh session.

   Typical usage:
   ```bash
      python3 z_info.py
   ```


### z_put

   Puts a path/value into Zenoh.
   The path/value will be received by all matching subscribers, for instance the [z_sub](#z_sub)
   and [z_storage](#z_storage) examples.

   Typical usage:
   ```bash
      python3 z_put.py
   ```
   or
   ```bash
      python3 z_put.py -k /demo/example/test -v 'Hello World'
   ```

### z_pub

   Declares a resource with a path and a publisher on this resource. Then puts a value using the numerical resource id.
   The path/value will be received by all matching subscribers, for instance the [z_sub](#z_sub)
   and [z_storage](#z_storage) examples.

   Typical usage:
   ```bash
      python3 z_pub.py
   ```
   or
   ```bash
      python3 z_pub.py -k /demo/example/test -v 'Hello World'
   ```

### z_sub

   Creates a subscriber with a key expression.
   The subscriber will be notified of each put made on any key expression matching
   the subscriber's key expression, and will print this notification.

   Typical usage:
   ```bash
      python3 z_sub.py
   ```
   or
   ```bash
      python3 z_sub.py -k '/demo/**'
   ```

### z_pull

   Creates a pull subscriber with a selector.
   The pull subscriber will receive each put made on any key expression matching
   the subscriber's key expression and will pull on demand and print the received
   key/value.

   Typical usage:
   ```bash
      python3 z_pull.py
   ```
   or
   ```bash
      python3 z_pull.py -k '/demo/**'
   ```

### z_get

   Sends a query message for a selector.
   The queryables with a matching path or selector (for instance [z_queryable](#z_queryable) and [z_storage](#z_storage))
   will receive this query and reply with paths/values that will be received by the query callback.

   Typical usage:
   ```bash
      python3 z_get.py
   ```
   or
   ```bash
      python3 z_get.py -s '/demo/**'
   ```

### z_queryable

   Creates a queryable function with a key expression.
   This queryable function will be triggered by each call to a get operation on zenoh
   with a selector that matches the key expression, and will return a value to the querier.

   Typical usage:
   ```bash
      python3 z_queryable.py
   ```
   or
   ```bash
      python3 z_queryable.py -k /demo/example/queryable -v 'This is the result'
   ```

### z_storage

   Trivial implementation of a storage in memory.
   This examples creates a subscriber and a queryable on the same key expression.
   The subscriber callback will store the received key/values in an hashmap.
   The queryable callback will answer to queries with the key/values stored in the hashmap
   and that match the queried selector.

   Typical usage:
   ```bash
      python3 z_storage.py
   ```
   or
   ```bash
      python3 z_storage.py -k '/demo/**'
   ```

### z_pub_thr & z_sub_thr

   Pub/Sub throughput test.
   This example allows to perform throughput measurements between a pubisher performing
   put operations and a subscriber receiving notifications of those puts.

   Typical Subscriber usage:
   ```bash
      python3 z_sub_thr.py
   ```

   Typical Publisher usage:
   ```bash
      python3 z_pub_thr.py 1024
   ```

### asyncio

In `asyncio` directory there are similar examples than thse described above, but leveraging Python's [asyncio](https://docs.python.org/fr/3/library/asyncio.html).

Especially, the `asyncio/z_get_parallel.py` and `asyncio/z_queryable.py` examples show how a zenoh application can issue concurent queries and how another zenoh application can concurrently compute replies to those queries.