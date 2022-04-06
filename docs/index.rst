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

module zenoh
============
.. automodule:: zenoh
    :members: init_logger, config_from_file, open, async_open, scout, async_scout


AsyncQueryable
--------------
.. autoclass:: zenoh.AsyncQueryable
    :members:

AsyncSession
------------
.. autoclass:: zenoh.AsyncSession
    :members:

AsyncSubscriber
---------------
.. autoclass:: zenoh.AsyncSubscriber
    :members:

Config
------
.. autoclass:: zenoh.Config
    :members:

CongestionControl
-----------------
.. autoclass:: zenoh.CongestionControl
    :members:
    :undoc-members:

ConsolidationMode
-----------------
.. autoclass:: zenoh.ConsolidationMode
    :members:
    :undoc-members:

Encoding
--------
.. autoclass:: zenoh.Encoding
    :members:
    :undoc-members:

Hello
-----
.. autoclass:: zenoh.Hello
    :members:

KeyExpr
-------
.. autoclass:: zenoh.KeyExpr
    :members:

KnownEncoding
-------------
.. autoclass:: zenoh.KnownEncoding
    :members:
    :undoc-members:

PeerId
------
.. autoclass:: zenoh.PeerId
    :members:

Period
------
.. autoclass:: zenoh.Period
    :members:
    :special-members: __init__

Priority
--------
.. autoclass:: zenoh.Priority
    :members:
    :undoc-members:

Query
-----
.. autoclass:: zenoh.Query
    :members:

Queryable
---------
.. autoclass:: zenoh.Queryable
    :members:

QueryConsolidation
------------------
.. autoclass:: zenoh.QueryConsolidation
    :members:

QueryTarget
-----------
.. autoclass:: zenoh.QueryTarget
    :members:

Reliability
-----------
.. autoclass:: zenoh.Reliability
    :members:
    :undoc-members:

Reply
-----
.. autoclass:: zenoh.Reply
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

Selector
--------
.. autoclass:: zenoh.Selector
    :members:

Session
-------
.. autoclass:: zenoh.Session
    :members:

SourceInfo
----------
.. autoclass:: zenoh.SourceInfo
    :members:

SubMode
-------
.. autoclass:: zenoh.SubMode
    :members:
    :undoc-members:

Subscriber
----------
.. autoclass:: zenoh.Subscriber
    :members:

Target
------
.. autoclass:: zenoh.Target
    :members:
    :undoc-members:

Timestamp
---------
.. autoclass:: zenoh.Timestamp
    :members:

Value
-----
.. autoclass:: zenoh.Value
    :members:

ValueSelector
-------------
.. autoclass:: zenoh.ValueSelector
    :members:

WhatAmI
-------
.. autoclass:: zenoh.WhatAmI
    :members:
    :undoc-members:

ZError
------
.. autoexception:: zenoh.ZError
    :members:


***********
submodules
***********

module zenoh.config
===================
.. autoclass:: zenoh.config
    :members:
    :undoc-members:

module zenoh.info
=================
.. autoclass:: zenoh.info
    :members:
    :undoc-members:

module zenoh.queryable
======================
.. autoclass:: zenoh.queryable
    :members:
    :undoc-members:
