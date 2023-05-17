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

Examples:
=========

Publish
-------

>>> import zenoh
>>> s = zenoh.open({})
>>> s.put('key/expression', 'value')

Subscribe
---------

>>> import zenoh
>>> def listener(sample):
...     print(f"Received '{sample.key_expr}': '{sample.payload.decode('utf-8')}'")
... 
>>> s = zenoh.open({})
>>> sub = s.declare_subscriber('key/expression', listener)

Query
-----

>>> import zenoh
>>> s = zenoh.open({})
>>> for reply in s.get('key/expression', zenoh.Queue()):
...     try:
...         print(f"Received '{reply.ok.key_expr}': '{reply.ok.payload.decode('utf-8')}'")
...     except:
...         print(f"Received ERROR: '{reply.err.payload.decode('utf-8')}'")
... 

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

Subscriber
----------
.. autoclass:: zenoh.Subscriber
    :members:

PullSubscriber
--------------
.. autoclass:: zenoh.PullSubscriber
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

Reliability
-----------
.. autoclass:: zenoh.Reliability
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
