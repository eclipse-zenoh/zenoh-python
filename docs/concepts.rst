..
.. Copyright (c) 2017, 2022 ZettaScale Technology
..
.. This program and the accompanying materials are made available under the
.. terms of the Eclipse Public License 2.0 which is available at
.. http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
.. which is available at https://www.apache.org/licenses/LICENSE-2.0.
..
.. SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
..
.. Contributors:
..   ZettaScale Zenoh team, <zenoh@zettascale.tech>
..

Components and Concepts
=======================

Session and Config
------------------

Zenoh supports two paradigms of communication: publish/subscribe and query/reply. The entities 
that perform the communication (e.g., publishers, subscribers, queriers, and queryables) are declared 
by the :class:`zenoh.Session` object. A session is created by the :func:`zenoh.open` function, which 
takes a :class:`zenoh.Config` as an argument. 

Publish/Subscribe
-----------------

Data is published via the :class:`zenoh.Publisher` which is declared by the :meth:`zenoh.Session.declare_publisher` method. 
There are two operations in the publisher: :meth:`zenoh.Publisher.put` and :meth:`zenoh.Publisher.delete`.
Publishing can also be done directly from the session via the :meth:`zenoh.Session.put` and :meth:`zenoh.Session.delete` methods.

The published data is received as :class:`zenoh.Sample` by :class:`zenoh.Subscriber` which is declared with :meth:`zenoh.Session.declare_subscriber`.

Publishing may express two different semantics:

• producing a sequence of values
• updating a single value associated with a key expression

In the second case, it's necessary to be able to declare that some key is no longer associated with any value. The :meth:`zenoh.Publisher.delete` operation is used for this.

On the receiving side, the subscriber distinguishes between the :attr:`zenoh.SampleKind.PUT` and :attr:`zenoh.SampleKind.DELETE` operations by the :attr:`zenoh.Sample.kind` field of the :class:`zenoh.Sample` structure.

The delete operation allows the subscriber to work with a :class:`zenoh.Queryable` which caches the values associated with key expressions.

Examples
^^^^^^^^

**Declaring a publisher and publishing data**

.. code-block:: python

    publisher = session.declare_publisher("key/expression")
    publisher.put("value")

**Declaring a subscriber and receiving data**

.. code-block:: python

    subscriber = session.declare_subscriber("key/expression")
    for sample in subscriber:
        print(f">> Received {sample.payload.to_string()}")

**Using session methods directly**

.. code-block:: python

    # Direct put operation
    session.put("key/expression", "value")
    
    # Direct delete operation  
    session.delete("key/expression")

Query/Reply
-----------

In the query/reply paradigm, data is made available by :class:`zenoh.Queryable` and requested by 
:class:`zenoh.Querier` or directly via :meth:`zenoh.Session.get` operations.

A :class:`zenoh.Queryable` is declared by the :meth:`zenoh.Session.declare_queryable` method and serves queries 
(:class:`zenoh.Query`). 
The :class:`zenoh.Query` has the methods :meth:`zenoh.Query.reply` to reply with a data sample
with a :attr:`zenoh.SampleKind.PUT` :attr:`zenoh.Sample.kind`, and
:meth:`zenoh.Query.reply_del` to send a reply with a :attr:`zenoh.SampleKind.DELETE` kind. See the `Publish/Subscribe` section for more details on the difference between the two kinds of samples.
The :meth:`zenoh.Query.reply_err` method is used to send a reply with error information.

Data is requested from queryables via the :meth:`zenoh.Session.get` function or by a :class:`zenoh.Querier` object. 
Each request returns zero or more :class:`zenoh.Reply` structures, each one from each queryable that matches 
the request. The reply contains either a :class:`zenoh.Sample` or a :class:`zenoh.ReplyError`.

Query Parameters
^^^^^^^^^^^^^^^^

The query/reply API allows specifying additional parameters for the request. These parameters are passed to 
the get operation using the :class:`zenoh.Selector` syntax. The selector string has a syntax similar to a URL: 
it's a key expression followed by a question mark and the list of parameters in the format "name=value" 
separated by ';'. For example ``key/expression?param1=value1;param2=value2``.

Examples
^^^^^^^^

**Declaring a queryable**

.. code-block:: python

    # Queryable that replies with temperature data for a given day
    queryable = session.declare_queryable("room/temperature/history")
    for query in queryable:
        if "day" in query.selector.parameters:
            day = query.selector.parameters["day"]
            if day in temperature_data:
                query.reply("room/temperature/history", temperature_data[day])
            else:
                query.reply_del("no data for this day")
        else:
            query.reply_err("missing day parameter")

**Requesting data using Session.get**

.. code-block:: python

    # Request temperature for a specific day
    replies = session.get("room/temperature/history?day=2023-03-15")
    for reply in replies:
        if reply.ok:
            print(f">> Temperature is {reply.ok.payload.to_string()}")
        else:
            print(f">> Error: {reply.err.payload.to_string()}")

**Using a Querier**

.. code-block:: python

    # Declare a querier for multiple queries
    querier = session.declare_querier("room/temperature/history")
    
    # Send a query with parameters
    replies = querier.get(parameters="?day=2023-03-15")
    for reply in replies:
        if reply.ok:
            print(f">> Temperature is {reply.ok.payload.to_string()}")
        else:
            print(f">> Error: {reply.err.payload.to_string()}") 

Key Expressions
---------------

`Key expressions <https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Key%20Expressions.md>`_ are Zenoh's address space.

In Zenoh, data is associated with keys in the form of a slash-separated path, e.g., ``robot/sensor/temp``. The 
requesting side uses key expressions to address the data of interest. Key expressions can contain 
wildcards:

- ``*`` matches any chunk (a chunk is a sequence of characters between ``/`` separators)
- ``**`` matches any number of chunks (including zero chunks)

For example:
- ``robot/sensor/*`` matches ``robot/sensor/temp``, ``robot/sensor/humidity``, etc.
- ``robot/**`` matches ``robot/sensor/temp``, ``robot/actuator/motor``, ``robot/status``, etc.

The :class:`zenoh.KeyExpr` class provides validation and operations on key expressions. Key expressions 
can be created using the constructor, which validates the syntax of the provided string:

.. code-block:: python

    from zenoh import KeyExpr
    
    # Create a key expression with validation
    sensor_ke = KeyExpr("robot/sensor")
    
    # Join with another segment
    temp_ke = sensor_ke.join("temp")
    
    # Create a wildcard expression
    all_sensors = sensor_ke.join("**")

Key expressions support various operations to check relationships between them,
like intersection and inclusion, which are useful for determining how different key expressions relate to each other, like :meth:`zenoh.KeyExpr.intersects` and :meth:`zenoh.KeyExpr.includes`.

The key expressions can also be declared with the session to optimize routing and network usage:

.. code-block:: python

    # Declare a key expression for optimized routing
    declared_ke = session.declare_keyexpr("robot/sensor/temperature")
    
    # Use the declared key expression
    publisher = session.declare_publisher(declared_ke)

Data representation
-------------------

Data is received as :class:`zenoh.Sample`\s, which contain the payload and all metadata associated with 
the data.
The raw byte payload object is :class:`zenoh.ZBytes`. The serialization and deserialization 
of basic types and structures is provided in the :mod:`zenoh.ext` module with functions :func:`zenoh.ext.z_serialize` and :func:`zenoh.ext.z_deserialize`.

Scouting
--------

Scouting is the process of discovering Zenoh nodes in the network. The scouting process depends on the transport 
layer and the Zenoh configuration. Note that it's not necessary to explicitly discover other nodes just to publish,
subscribe, or query data.
See more details at `scouting documentation <https://zenoh.io/docs/getting-started/deployment/#scouting>`_.

Examples
^^^^^^^^

.. code-block:: python

    scout = zenoh.scout(what="peer|router")
    threading.Timer(1.0, lambda: scout.stop()).start()
    for hello in scout:
        print(hello)

Liveliness
----------

Zenoh allows monitoring of liveliness to be notified when a specified resource appears or disappears in the network.

Sometimes it's necessary to know whether a Zenoh node is available on the network. It's possible to achieve this 
by declaring special publishers and queryables, but this task is not straightforward, so a dedicated API is provided.

The :meth:`zenoh.Session.liveliness` API allows a node to declare a :class:`zenoh.LivelinessToken` 
by :meth:`zenoh.Liveliness.declare_token` with a key expression assigned to it. Other nodes can use the 
liveliness API to query this key expression with :meth:`zenoh.Liveliness.get` or subscribe to it 
with :meth:`zenoh.Liveliness.declare_subscriber` to be notified when the token appears or disappears on the network.
The `history` parameter of the :meth:`zenoh.Liveliness.declare_subscriber` allows to immediately receive the 
alive tokens that are already present on the network. 

Examples
^^^^^^^^

Declare a liveliness token

.. code-block:: python

    token = session.liveliness.declare_token("node/A")

Get currently present liveliness tokens

.. code-block:: python

        replies = session.liveliness().get("node/A", timeout=5)
        for reply in replies:
            if reply.ok:
                print(f"Alive token ('{reply.ok.key_expr}')")
            else:
                print(f"Received (ERROR: '{reply.err.payload.to_string()}')")


Check if a liveliness token is present and subscribe to changes

.. code-block:: python

    with session.liveliness().declare_subscriber("node/A", history=True) as sub:
        for sample in sub:
            if sample.kind == zenoh.SampleKind.PUT:
                print(f"Alive token ('{sample.key_expr}')")
            elif sample.kind == zenoh.SampleKind.DELETE:
                print(f"Dropped token ('{sample.key_expr}')")


Matching
--------

The matching API allows the active side of communication (publisher, querier) to know whether there are any interested parties on the other side (subscriber, queryable),
which can save bandwidth and CPU resources.

A MatchingListener can be declared via the :meth:`zenoh.Publisher.matching_listener` or :meth:`zenoh.Querier.matching_listener` methods.

The matching listener behaves like a subscriber, but instead of producing data samples it yields :class:`zenoh.MatchingStatus` instances whenever the matching status changes,
i.e., when the first matching subscriber or queryable appears, or when the last one disappears.

Examples
^^^^^^^^

**Declare a matching listener for a publisher**

.. code-block:: python

    publisher = session.declare_publisher("key/expression")
    listener = publisher.matching_listener()
    for status in listener:
        if status.matching:
            print(">> Publisher has at least one matching subscriber")
        else:
            print(">> Publisher has no matching subscribers")

**Declare a matching listener for a querier**

.. code-block:: python

    querier = session.declare_querier("service/endpoint")
    listener = querier.matching_listener()
    for status in listener:
        if status.matching:
            print(">> Querier has at least one matching queryable")
        else:
            print(">> Querier has no matching queryables")

Channels and callbacks
----------------------

There are two ways to get sequential data from Zenoh primitives (e.g., a series of :class:`zenoh.Sample`\s from a :class:`zenoh.Subscriber` or :class:`zenoh.Reply`\s from a :class:`zenoh.Query`): by channel or by callback.

In channel mode, methods like ``recv()`` become available on the subscriber or query object. By default, the ``FifoChannel`` is used.

The builders provide methods ``with_handler()`` to assign an arbitrary channel instead of the default one, and ``callback()`` to assign a callback function.