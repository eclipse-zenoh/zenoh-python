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



[Zenoh](https://zenoh.io) /zeno/ is a stack that unifies data in motion, data at
rest and computations. It elegantly blends traditional pub/sub with geo distributed
storage, queries and computations, while retaining a level of time and space efficiency
that is well beyond any of the mainstream stacks.

Before delving into the examples, we need to introduce few **Zenoh** concepts.
First off, in Zenoh you will deal with **Resources**, where a resource is made up of a
key and a value.  The other concept you'll have to familiarize yourself with are
**key expressions**, such as ``robot/sensor/temp``, ``robot/sensor/*``, ``robot/**``, etc.
As you can gather, the above key expression denotes set of keys, while the ``*`` and ``**``
are wildcards representing respectively (1) a single chunk (non-empty sequence of characters that doesn't contain ``/``), and (2) any amount of chunks (including 0).

Below are some examples that highlight these key concepts and show how easy it is to get
started with.

Quick start examples:
^^^^^^^^^^^^^^^^^^^^^

Publish a key/value pair onto Zenoh
"""""""""""""""""""""""""""""""""""

>>> import zenoh
>>> z = zenoh.open()
>>> z.put('/demo/example/hello', 'Hello World!')

Subscribe to a set of keys with Zenoh
"""""""""""""""""""""""""""""""""""""

>>> import zenoh, time
>>> def listener(sample):
>>>     print(f"{sample.key_expr} => {sample.payload.decode('utf-8')}")
>>>
>>> z = zenoh.open()
>>> subscriber = z.subscribe('/demo/example/**', listener)
>>> time.sleep(60)
>>> subscriber.undeclare()

Get keys/values from zenoh
""""""""""""""""""""""""""

>>> import zenoh
>>> z = zenoh.open()
>>> for response in z.get('/demo/example/**', zenoh.Queue()):
>>>     response = response.ok
>>>     print(f"{response.key_expr} => {response.payload.decode('utf-8')}")

module zenoh
============

.. automodule:: zenoh
    :members: init_logger, open, scout

Hello
-----
.. autoclass:: zenoh.Hello
    :members:

Config
------
.. autoclass:: zenoh.Config
    :members:

Session
-------
.. autoclass:: zenoh.Session
    :members:

Info
-----
.. autoclass:: zenoh.Info
    :members:

KeyExpr
-------
.. autoclass:: zenoh.KeyExpr
    :members:

Sample
------
.. autoclass:: zenoh.Sample
    :members:

SampleKind
----------
.. autoclass:: zenoh.SampleKind
    :members:
    :undoc-members:

Value
-----
.. autoclass:: zenoh.Value
    :members:

Encoding
--------
.. autoclass:: zenoh.Encoding
    :members:
    :undoc-members:

Publisher
----------
.. autoclass:: zenoh.Publisher
    :members:

CongestionControl
-----------------
.. autoclass:: zenoh.CongestionControl
    :members:
    :undoc-members:

Priority
--------
.. autoclass:: zenoh.Priority
    :members:
    :undoc-members:

Subscriber
----------
.. autoclass:: zenoh.Subscriber
    :members:

PullSubscriber
--------------
.. autoclass:: zenoh.PullSubscriber
    :members:

Reliability
-----------
.. autoclass:: zenoh.Reliability
    :members:
    :undoc-members:

Query
-----
.. autoclass:: zenoh.Query
    :members:

Selector
--------
.. autoclass:: zenoh.Selector
    :members:

QueryTarget
-----------
.. autoclass:: zenoh.QueryTarget
    :members:

QueryConsolidation
------------------
.. autoclass:: zenoh.QueryConsolidation
    :members:

.. ConsolidationMode
.. -----------------
.. .. autoclass:: zenoh.ConsolidationMode
..     :members:
..     :undoc-members:

Reply
-----
.. autoclass:: zenoh.Reply
    :members:

Queryable
---------
.. autoclass:: zenoh.Queryable
    :members:

ZenohId
-------
.. autoclass:: zenoh.ZenohId
    :members:

Timestamp
---------
.. autoclass:: zenoh.Timestamp
    :members:

.. automodule:: zenoh
    :members: Queue, ListCollector, Closure, Handler, IClosure, IHandler, IValue
