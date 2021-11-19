..
.. Copyright (c) 2017, 2020 ADLINK Technology Inc.
..
.. This program and the accompanying materials are made available under the
.. terms of the Eclipse Public License 2.0 which is available at
.. http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
.. which is available at https://www.apache.org/licenses/LICENSE-2.0.
..
.. SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
..
.. Contributors:
..   ADLINK zenoh team, <zenoh@adlink-labs.tech>
..

Zenoh-net API Reference
=======================

.. automodule:: zenoh
    :members: open, scout

Hello
-----
.. autoclass:: zenoh.Hello
    :members:

Session
-------
.. autoclass:: zenoh.Session
    :members:

Subscriber
----------
.. autoclass:: zenoh.Subscriber
    :members:

KeyExpr
------
.. autoclass:: zenoh.KeyExpr
    :members:

PeerId
------
.. autoclass:: zenoh.PeerId
    :members:

Timestamp
---------
.. autoclass:: zenoh.Timestamp
    :members:

DataInfo
--------
.. autoclass:: zenoh.DataInfo
    :members:

Sample
------
.. autoclass:: zenoh.Sample
    :members:

Reliability
-----------
.. autoclass:: zenoh.Reliability
    :members:

    .. autoattribute:: BestEffort
            :annotation:
    .. autoattribute:: Reliable
            :annotation:

SubMode
-------
.. autoclass:: zenoh.SubMode
    :members:

    .. autoattribute:: Push
            :annotation:
    .. autoattribute:: Pull
            :annotation:

Period
------
.. autoclass:: zenoh.Period
    :members:

SubInfo
-------
.. autoclass:: zenoh.SubInfo
    :members:

Publisher
---------
.. autoclass:: zenoh.Publisher
    :members:

CongestionControl
-----------------
.. autoclass:: zenoh.CongestionControl
    :members:

    .. autoattribute:: Drop
            :annotation:
    .. autoattribute:: Block
            :annotation:

Query
-----
.. autoclass:: zenoh.Query
    :members:

Queryable
---------
.. autoclass:: zenoh.Queryable
    :members:

Target
------
.. autoclass:: zenoh.Target
    :members:

    .. autoattribute:: BestMatching
            :annotation:
    .. autoattribute:: Complete
            :annotation:
    .. autoattribute:: All
            :annotation:
    .. autoattribute:: No
            :annotation:

QueryTarget
-----------
.. autoclass:: zenoh.QueryTarget
    :members:

ConsolidationMode
-----------------
.. autoclass:: zenoh.ConsolidationMode
    :members:

    .. autoattribute:: No
            :annotation:
    .. autoattribute:: Lazy
            :annotation:
    .. autoattribute:: Full
            :annotation:

QueryConsolidation
------------------
.. autoclass:: zenoh.QueryConsolidation
    :members:

Reply
-----
.. autoclass:: zenoh.Reply
    :members:

module zenoh.config
-----------------------
.. automodule:: zenoh.config
    :members:
    :undoc-members:

module zenoh.info
---------------------
.. automodule:: zenoh.info
    :members:
    :undoc-members:

module zenoh.whatami
------------------------
.. automodule:: zenoh.whatami
    :members:
    :undoc-members:

module zenoh.queryable
--------------------------
.. automodule:: zenoh.queryable
    :members:
    :undoc-members:

module zenoh.resource_name
------------------------------
.. automodule:: zenoh.resource_name
    :members:

module zenoh.encoding
-------------------------
.. automodule:: zenoh.encoding
    :members:
    :undoc-members:

module zenoh.data_kind
--------------------------
.. automodule:: zenoh.data_kind
    :members:
    :undoc-members:

