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

.. automodule:: zenoh.net
    :members: open, scout

Config
------
.. autoclass:: zenoh.net.Config
    :members:

Hello
-----
.. autoclass:: zenoh.net.Hello
    :members:

Session
-------
.. autoclass:: zenoh.net.Session
    :members:

Subscriber
----------
.. autoclass:: zenoh.net.Subscriber
    :members:

ResKey
------
.. autoclass:: zenoh.net.ResKey
    :members:

PeerId
------
.. autoclass:: zenoh.net.PeerId
    :members:

Timestamp
---------
.. autoclass:: zenoh.net.Timestamp
    :members:

DataInfo
--------
.. autoclass:: zenoh.net.DataInfo
    :members:

Sample
------
.. autoclass:: zenoh.net.Sample
    :members:

Reliability
-----------
.. autoclass:: zenoh.net.Reliability
    :members:

    .. autoattribute:: BestEffort
            :annotation:
    .. autoattribute:: Reliable
            :annotation:

SubMode
-------
.. autoclass:: zenoh.net.SubMode
    :members:

    .. autoattribute:: Push
            :annotation:
    .. autoattribute:: Pull
            :annotation:

Period
------
.. autoclass:: zenoh.net.Period
    :members:

SubInfo
-------
.. autoclass:: zenoh.net.SubInfo
    :members:

Publisher
---------
.. autoclass:: zenoh.net.Publisher
    :members:

Query
-----
.. autoclass:: zenoh.net.Query
    :members:

Queryable
---------
.. autoclass:: zenoh.net.Queryable
    :members:

Target
------
.. autoclass:: zenoh.net.Target
    :members:

QueryTarget
-----------
.. autoclass:: zenoh.net.QueryTarget
    :members:

QueryConsolidation
------------------
.. autoclass:: zenoh.net.QueryConsolidation
    :members:

    .. autoattribute:: None
            :annotation:
    .. autoattribute:: LastHop
            :annotation:
    .. autoattribute:: Incremental
            :annotation:

Reply
-----
.. autoclass:: zenoh.net.Reply
    :members:

module zenoh.net.properties
---------------------------
.. autoclass:: zenoh.net.properties
    :members:

    .. autoattribute:: ZN_USER_KEY
    .. autoattribute:: ZN_PASSWD_KEY
    .. autoattribute:: ZN_INFO_PID_KEY
    .. autoattribute:: ZN_INFO_PEER_PID_KEY
    .. autoattribute:: ZN_INFO_ROUTER_PID_KEY

module zenoh.net.whatami
------------------------
.. autoclass:: zenoh.net.whatami
    :members:

    .. autoattribute:: ROUTER
    .. autoattribute:: PEER
    .. autoattribute:: CLIENT

module zenoh.net.queryable
--------------------------
.. autoclass:: zenoh.net.queryable
    :members:

    .. autoattribute:: ALL_KINDS
    .. autoattribute:: EVAL
    .. autoattribute:: STORAGE

module zenoh.net.resource_name
------------------------------
.. automodule:: zenoh.net.resource_name
    :members:

module zenoh.net.encoding
-------------------------
.. autoclass:: zenoh.net.encoding
    :members: to_str, from_str

    .. autoattribute:: APP_OCTET_STREAM
    .. autoattribute:: NONE
    .. autoattribute:: APP_CUSTOM
    .. autoattribute:: TEXT_PLAIN
    .. autoattribute:: STRING
    .. autoattribute:: APP_PROPERTIES
    .. autoattribute:: APP_JSON
    .. autoattribute:: APP_SQL
    .. autoattribute:: APP_INTEGER
    .. autoattribute:: APP_FLOAT
    .. autoattribute:: APP_XML
    .. autoattribute:: APP_XHTML_XML
    .. autoattribute:: APP_X_WWW_FORM_URLENCODED
    .. autoattribute:: TEXT_JSON
    .. autoattribute:: TEXT_HTML
    .. autoattribute:: TEXT_XML
    .. autoattribute:: TEXT_CSS
    .. autoattribute:: TEXT_CSV
    .. autoattribute:: TEXT_JAVASCRIPT
    .. autoattribute:: IMG_JPG
    .. autoattribute:: IMG_PNG
    .. autoattribute:: IMG_GIF
    .. autoattribute:: DEFAULT

