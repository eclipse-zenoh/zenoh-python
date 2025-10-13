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

Key Expressions
---------------

Data is associated with keys in the form of a slash-separated path, e.g., ``robot/sensor/temp``. The 
requesting side uses key expressions to address the data of interest. Key expressions can contain 
wildcards, e.g., ``robot/sensor/*`` or ``robot/**``.

Data representation
-------------------

Data is received as :class:`zenoh.Sample`\s, which contain the payload and all metadata associated with 
the data. The raw byte payload object is :class:`zenoh.ZBytes`. The serialization and deserialization 
of basic types and structures is provided in the :mod:`zenoh.ext` module.

Scouting
--------

Scouting is the process of discovering Zenoh nodes in the network. Note that it's not necessary to 
explicitly discover other nodes just to publish, subscribe, or query data.

Liveliness
----------

Zenoh allows monitoring of liveliness to be notified when a specified resource appears or disappears in the network.

Matching
--------

The matching API allows the active side of communication (publisher, querier) to know whether there are any interested parties on the other side (subscriber, queryable), which allows saving bandwidth and CPU resources.

Channels and callbacks
----------------------

There are two ways to get sequential data from Zenoh primitives (e.g., a series of :class:`zenoh.Sample`\s from a :class:`zenoh.Subscriber` or :class:`zenoh.Reply`\s from a :class:`zenoh.Query`): by channel or by callback.

In channel mode, methods like ``recv()`` become available on the subscriber or query object. By default, the ``FifoChannel`` is used.

The builders provide methods ``with_handler()`` to assign an arbitrary channel instead of the default one, and ``callback()`` to assign a callback function.