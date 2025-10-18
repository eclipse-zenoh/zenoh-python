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
that perform communication (for example, publishers, subscribers, queriers, and queryables) are
declared through a :class:`zenoh.Session`. A session is created by the :func:`zenoh.open` function,
which takes a :class:`zenoh.Config` as an argument.

The configuration is stored in a json file. The file format is documented in the Zenoh Rust API
`Config <https://docs.rs/zenoh/latest/zenoh/config/struct.Config.html>`_ reference.

.. _publish-subscribe:

Publish/Subscribe
-----------------

Data is published via a :class:`zenoh.Publisher`, which is declared using
:meth:`zenoh.Session.declare_publisher`. The publisher exposes two primary operations:
:meth:`zenoh.Publisher.put` and :meth:`zenoh.Publisher.delete`. Publishing can also be performed
directly from the session via :meth:`zenoh.Session.put` and :meth:`zenoh.Session.delete`.

Published data is received as :class:`zenoh.Sample` instances by a :class:`zenoh.Subscriber`,
which is declared using :meth:`zenoh.Session.declare_subscriber`. The samples are delivered to the
callback or channel (:ref:`channels-and-callbacks`).

Publishing can express two different semantics:

- producing a sequence of values
- updating a single value associated with a key expression

In the second case, it is necessary to indicate that a key is no longer associated
with any value; the :meth:`zenoh.Publisher.delete` operation is used for this.

On the receiving side, the subscriber distinguishes between
:attr:`zenoh.SampleKind.PUT` and :attr:`zenoh.SampleKind.DELETE` using the
:attr:`zenoh.Sample.kind` field in the :class:`zenoh.Sample` structure.

The delete operation allows a subscriber to work with a :class:`zenoh.Queryable`
that caches the values associated with key expressions.

Examples
^^^^^^^^

**Declaring a publisher and publishing data**

.. literalinclude:: examples/pubsub_publisher.py
   :language: python
   :lines: 7-9

**Declaring a subscriber and receiving data**

.. literalinclude:: examples/pubsub_subscriber.py
   :language: python
   :lines: 20-23

**Using session methods directly**

.. literalinclude:: examples/pubsub_session_direct.py
   :language: python
   :lines: 7-11

Query/Reply
-----------

In the query/reply paradigm, data is made available by a :class:`zenoh.Queryable` and
requested by a :class:`zenoh.Querier` or directly via :meth:`zenoh.Session.get`.

A :class:`zenoh.Queryable` is declared using :meth:`zenoh.Session.declare_queryable`. 
It serves :class:`zenoh.Query` requests via a callback or channel
(:ref:`channels-and-callbacks`).

The :class:`zenoh.Query` provides
:meth:`zenoh.Query.reply` to reply with a data sample of the :attr:`zenoh.SampleKind.PUT` kind, and
:meth:`zenoh.Query.reply_del` to send a :attr:`zenoh.SampleKind.DELETE` reply.
See :ref:`publish-subscribe` for more details on the difference between the
two sample kinds. There is also :meth:`zenoh.Query.reply_err` method 
which can be used to send a reply containing error information.

Data is requested from queryables via :meth:`zenoh.Session.get` or via a
:class:`zenoh.Querier` object. Each request returns zero or more
:class:`zenoh.Reply` structures — one per queryable that matches the request.
Each reply contains either a :class:`zenoh.Sample` from `reply` and `reply_del`
or a :class:`zenoh.ReplyError` from `reply_err`.

Query Parameters
^^^^^^^^^^^^^^^^

The query/reply API allows specifying additional parameters for the request.
A :class:`zenoh.Selector` is passed to the :meth:`zenoh.Session.get` operation
and combines a key expression with optional parameters. The selector string has
a syntax similar to a URL: it is a key expression followed by a question mark
and a list of parameters in the format "name=value", separated by ``;``.
For example: ``key/expression?param1=value1;param2=value2``.

Alternatively, parameters can be constructed programmatically using the
:class:`zenoh.Parameters` class, which accepts a dictionary, and then combined
with a key expression to create a :class:`zenoh.Selector`.

On the receiving side, queryables can access these parameters via
:attr:`zenoh.Query.parameters`.

Examples
^^^^^^^^

**Declaring a queryable**

.. literalinclude:: examples/query_queryable.py
   :language: python
   :lines: 33-44

**Requesting data using Session.get**

.. literalinclude:: examples/query_session_get.py
   :language: python
   :lines: 37-43

**Creating a Selector from Parameters**

.. literalinclude:: examples/query_parameters.py
   :language: python
   :lines: 28-38

**Using a Querier**

.. literalinclude:: examples/query_querier.py
   :language: python
   :lines: 37-46 

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

The :class:`zenoh.KeyExpr` class provides validation and operations on key
expressions. The key expressions constructor validates the syntax of the provided string
and raises a :class:`zenoh.ZError` exception if the syntax is invalid (e.g. contains spaces, other illegal characters, have empty chunks `foo//bar` or `/foo`).

The `KeyExpr` constructor fires exception on key expressions which are valid, but not in
`canon form <https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Key%20Expressions.md#canon-forms>`_.
For example, ``robot/sensor/**/*`` is valid but it's canon form is ``robot/sensor/*/**``.
The :meth:`zenoh.KeyExpr.autocanonize` method can accept such key expressions and
convert them to their canon form.

.. literalinclude:: examples/keyexpr_validation.py
   :language: python
   :lines: 12-23

Key expressions support operations such as intersection and inclusion (see
:meth:`zenoh.KeyExpr.intersects` and :meth:`zenoh.KeyExpr.includes`), which
help determine how different expressions relate to each other.

.. literalinclude:: examples/keyexpr_operations.py
   :language: python
   :lines: 5-24

Key expressions can also be declared with the session to optimize routing and
network usage:

.. literalinclude:: examples/keyexpr_declare.py
   :language: python
   :lines: 7-11

Data representation
-------------------

Data is received as :class:`zenoh.Sample` objects, which contain the
:attr:`zenoh.Sample.payload` and associated metadata like :attr:`zenoh.Sample.timestamp`,
:attr:`zenoh.Sample.encoding`, and :attr:`zenoh.Sample.kind`. Additionally, optional
user-defined metadata can be attached via :attr:`zenoh.Sample.attachment`.

Both :attr:`zenoh.Sample.payload` and :attr:`zenoh.Sample.attachment` are of type
:class:`zenoh.ZBytes`, which represents raw byte data. Serialization and
deserialization of basic types and structures is provided in the :mod:`zenoh.ext`
module via :func:`zenoh.ext.z_serialize` and :func:`zenoh.ext.z_deserialize`.

Examples
^^^^^^^^

.. literalinclude:: examples/data_representation.py
   :language: python
   :lines: 17-33

Scouting
--------

Scouting is the process of discovering Zenoh nodes on the network. The scouting
process depends on the transport layer and the Zenoh configuration. Note that
it is not necessary to explicitly discover other nodes to publish, subscribe, or
query data.
See more details at `scouting documentation <https://zenoh.io/docs/getting-started/deployment/#scouting>`_.

Examples
^^^^^^^^

.. literalinclude:: examples/scouting.py
   :language: python
   :lines: 6-9

Liveliness
----------

Zenoh supports liveliness monitoring to notify when a specified resource appears
or disappears on the network.

Sometimes it is necessary to know whether a Zenoh node is available. This
can be achieved by declaring special publishers and queryables, but the 
dedicated liveliness API is more convenient and efficient.

The :meth:`zenoh.Session.liveliness` API allows a node to declare a
:class:`zenoh.LivelinessToken` using :meth:`zenoh.Liveliness.declare_token`,
associated with a key expression. Other nodes can query this key expression via
:meth:`zenoh.Liveliness.get` or subscribe using
:meth:`zenoh.Liveliness.declare_subscriber` to be notified when the token
appears or disappears. The ``history`` parameter of
:meth:`zenoh.Liveliness.declare_subscriber` allows immediate receipt of tokens
that are already present on the network.

Examples
^^^^^^^^

**Declare a liveliness token**

.. literalinclude:: examples/liveliness_token.py
   :language: python
   :lines: 7-8

**Get currently present liveliness tokens**

.. literalinclude:: examples/liveliness_get.py
   :language: python
   :lines: 22-28


**Check if a liveliness token is present and subscribe to changes**

.. literalinclude:: examples/liveliness_subscriber.py
   :language: python
   :lines: 26-32


Matching
--------

The matching API lets the active side of communication (publisher or querier)
learn whether there are interested parties on the other side (subscriber or
queryable). This information can save bandwidth and CPU resources.

Declare a :class:`zenoh.MatchingListener` via
:meth:`zenoh.Publisher.declare_matching_listener` or
:meth:`zenoh.Querier.declare_matching_listener`.

The matching listener behaves like a subscriber, but instead of producing data
samples it yields :class:`zenoh.MatchingStatus` instances whenever the matching
status changes — for example, when the first matching subscriber or queryable
appears or when the last one disappears.

Examples
^^^^^^^^

**Declare a matching listener for a publisher**

.. literalinclude:: examples/matching_publisher.py
   :language: python
   :lines: 26-33

**Declare a matching listener for a querier**

.. literalinclude:: examples/matching_querier.py
   :language: python
   :lines: 26-33

.. _channels-and-callbacks:

Channels and callbacks
----------------------

There are two ways to receive sequential data from Zenoh primitives (for
example, a series of :class:`zenoh.Sample` objects from a
:class:`zenoh.Subscriber` or :class:`zenoh.Reply` objects from a
:class:`zenoh.Query`): by channel or by callback.

This behavior is controlled by the ``handler`` parameter of the declare
methods (for example, :meth:`zenoh.Session.declare_subscriber` and
:meth:`zenoh.Session.declare_querier`). The parameter can be either a callable
(a function or a method) or a channel type (blocking
:class:`zenoh.FifoChannel` or non-blocking :class:`zenoh.RingChannel`). By
default, the ``handler`` parameter is set to :class:`zenoh.FifoChannel`.

When constructed with a channel, the returned object is iterable and can be
used in a ``for`` loop to receive data sequentially. It also provides explicit
methods such as :meth:`zenoh.Subscriber.recv` to wait for data and
:meth:`zenoh.Subscriber.try_recv` to attempt a non-blocking receive. The
subscriber (or queryable) is stopped when the object goes out of scope or when
:meth:`zenoh.Subscriber.undeclare` is called.

When constructed with a callback, the returned object is not iterable. The
callable is invoked for each received :class:`zenoh.Sample` or
:class:`zenoh.Reply`. With a callback the object is started in "background"
mode, which means the subscriber or queryable remains active even if the
returned object goes out of scope. This allows declaring a subscriber or
queryable without managing the returned object's lifetime.
