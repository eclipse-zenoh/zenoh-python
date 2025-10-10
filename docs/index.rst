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

*******************
Zenoh API Reference
*******************


`Zenoh <https://zenoh.io>`_ /zeno/ is a stack that unifies data in motion, data at
rest and computations. It elegantly blends traditional pub/sub with geo distributed
storage, queries and computations, while retaining a level of time and space efficiency
that is well beyond any of the mainstream stacks.

The Zenoh protocol allows nodes to form a graph with an arbitrary topology, such as a mesh, 
a star, or a clique. The zenoh routers keeps the network connected and routes the messages
between the nodes.

Quick start examples
====================

Below are some examples that highlight these key concepts and show how easy it is to get
started with. The more detailed documentation is available in the sections below.

Publish a key/value pair onto Zenoh
-----------------------------------

>>> import zenoh
>>> with zenoh.open() as session:
>>>     session.put('demo/example/hello', 'Hello World!')

Subscribe to a set of keys with Zenoh
-------------------------------------

>>> import zenoh, time
>>> def listener(sample):
>>>     print(f"{sample.key_expr} => {sample.payload.to_string()}")
>>>
>>> with zenoh.open() as session:
>>>     with session.declare_subscriber('demo/example/**', listener) as subscriber:
>>>         time.sleep(60)

Get keys/values from zenoh
--------------------------

>>> import zenoh
>>> with zenoh.open() as session:
>>>     for response in session.get('demo/example/**'):
>>>         response = response.ok
>>>         print(f"{response.key_expr} => {response.payload.to_string()}")


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

In the publish/subscribe paradigm, data is produced by :class:`zenoh.Publisher` and consumed by :class:`zenoh.Subscriber`.

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


API Reference
=============

module zenoh
------------

.. automodule:: zenoh
    :members:
    :undoc-members:

module zenoh.handlers
---------------------

.. automodule:: zenoh.handlers
    :members:
    :undoc-members:

module zenoh.ext
----------------

.. automodule:: zenoh.ext
    :members:
    :undoc-members:

module zenoh.shm
----------------

.. automodule:: zenoh.shm
    :members:
    :undoc-members:
