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

.. _session-and-config:

Session and Config
------------------

Zenoh supports two paradigms of communication: :ref:`publish-subscribe` and :ref:`query-reply`. The entities
that perform communication (for example, publishers, subscribers, queriers, and queryables) are
declared through a :class:`zenoh.Session`. A session is created by the :func:`zenoh.open` function,
which takes a :class:`zenoh.Config` as an argument.

The configuration is stored in a JSON file and can be read with :func:`zenoh.Config.from_file`.
The file format is documented in the Zenoh Rust API
`Config <https://docs.rs/zenoh/latest/zenoh/config/struct.Config.html>`_ reference.

.. important::

   The recommended way to create a session is using a context manager (``with`` statement).
   If a session is not explicitly closed or managed with a context manager, on exit object 
   finalizers may be called when the library thread has already been killed, which can
   cause the script to hang.

   Either use a context manager (recommended) or explicitly call :meth:`zenoh.Session.close`
   before your script exits. See examples in the :doc:`quickstart` section.

Example: Creating a session with context manager
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/session_config.py
   :language: python
   :start-after: [session_context_manager]
   :end-before: # [session_context_manager]

Example: Creating a session with explicit close
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/session_config.py
   :language: python
   :start-after: [session_explicit_close]
   :end-before: # [session_explicit_close]

.. _key-expressions:

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
expressions. The :class:`zenoh.KeyExpr` constructor validates the syntax of the provided string
and raises a :class:`zenoh.ZError` exception if the syntax is invalid (e.g., it contains spaces, other illegal characters, or has empty chunks like ``foo//bar`` or ``/foo``).

The :class:`zenoh.KeyExpr` constructor raises an exception for key expressions that are valid but not in
`canonical form <https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Key%20Expressions.md#canon-forms>`_.
For example, ``robot/sensor/**/*`` is valid, but its canonical form is ``robot/sensor/*/**``.
The :meth:`zenoh.KeyExpr.autocanonize` method can accept such key expressions and
convert them to their canonical form.

Example: Validating key expressions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/keyexpr_validation.py
   :language: python
   :start-after: [keyexpr_validation]
   :end-before: # [keyexpr_validation]

Key expressions support operations such as intersection and inclusion (see
:meth:`zenoh.KeyExpr.intersects` and :meth:`zenoh.KeyExpr.includes`), which
help determine how different expressions relate to each other.

Example: Performing operations on key expressions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/keyexpr_operations.py
   :language: python
   :start-after: [keyexpr_operations]
   :end-before: # [keyexpr_operations]

Key expressions can also be declared with the session to optimize routing and
network usage:

Example: Declaring key expressions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/keyexpr_declare.py
   :language: python
   :start-after: [keyexpr_declare]
   :end-before: # [keyexpr_declare]

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

Example: Declaring a publisher and publishing data
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/pubsub_publisher.py
   :language: python
   :start-after: [pubsub_publisher]
   :end-before: # [pubsub_publisher]

Example: Declaring a subscriber and receiving data
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/pubsub_subscriber.py
   :language: python
   :start-after: [pubsub_subscriber]
   :end-before: # [pubsub_subscriber]

Example: Using session methods directly
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/pubsub_session_direct.py
   :language: python
   :start-after: [pubsub_session_direct]
   :end-before: # [pubsub_session_direct]

.. _query-reply:

Query/Reply
-----------

In the query/reply paradigm, data is made available by a :class:`zenoh.Queryable` and
requested by a :class:`zenoh.Querier` or directly via :meth:`zenoh.Session.get`.

A :class:`zenoh.Queryable` is declared using :meth:`zenoh.Session.declare_queryable`. 
It serves :class:`zenoh.Query` requests via a callback or channel
(:ref:`channels-and-callbacks`).

The :class:`zenoh.Query` provides the :meth:`zenoh.Query.reply` method to reply with a
data sample of the :attr:`zenoh.SampleKind.PUT` kind, and
:meth:`zenoh.Query.reply_del` to send a :attr:`zenoh.SampleKind.DELETE` reply.
See :ref:`publish-subscribe` for more details on the difference between the
two sample kinds. There is also the :meth:`zenoh.Query.reply_err` method 
which can be used to send a reply containing error information.

Data is requested from queryables via :meth:`zenoh.Session.get` or via a
:class:`zenoh.Querier` object. Each request returns zero or more
:class:`zenoh.Reply` structures — one per queryable that matches the request.
Each reply contains either a :class:`zenoh.Sample` from `reply` and `reply_del`
or a :class:`zenoh.ReplyError` from `reply_err`.

Example: Declaring a queryable
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/query_queryable.py
   :language: python
   :start-after: [query_queryable]
   :end-before: # [query_queryable]

Example: Requesting data using Session.get
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/query_session_get.py
   :language: python
   :start-after: [query_session_get]
   :end-before: # [query_session_get]

Example: Using a Querier
~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/query_querier.py
   :language: python
   :start-after: [query_querier]
   :end-before: # [query_querier] 

.. _query-parameters:

Query Parameters
----------------

The query/reply API allows specifying additional parameters for the request.
A :class:`zenoh.Selector` object is passed to the :meth:`zenoh.Session.get` operation.
It combines a key expression with optional parameters and can be constructed from these
elements or by parsing a selector string. The selector string has
a syntax similar to a URL: it is a key expression followed by a question mark
and a list of parameters in the format "name=value", separated by ``;``.
For example: ``key/expression?param1=value1;param2=value2``.

Alternatively, parameters can be constructed programmatically using the
:class:`zenoh.Parameters` class, which accepts a dictionary, and then combined
with a key expression to create a :class:`zenoh.Selector`.

On the receiving side, queryables can access these parameters via
:attr:`zenoh.Query.parameters`.

Example: Constructing a Selector from dictionary
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/query_parameters.py
   :language: python
   :start-after: [query_parameters]
   :end-before: # [query_parameters]

.. _data-representation:

Data representation
-------------------

Data is received as :class:`zenoh.Sample` objects, which contain the
:attr:`zenoh.Sample.payload` and associated metadata like :attr:`zenoh.Sample.timestamp`,
:attr:`zenoh.Sample.encoding`, and :attr:`zenoh.Sample.kind`. Additionally, optional
user-defined metadata can be attached via :attr:`zenoh.Sample.attachment`.


Both :attr:`zenoh.Sample.payload` and :attr:`zenoh.Sample.attachment` are of type
:class:`zenoh.ZBytes`, which represents raw byte data. 

Example: Using :class:`zenoh.ZBytes`
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/data_representation.py
   :language: python
   :start-after: [raw_data]
   :end-before: # [raw_data]

Serialization and deserialization of basic types and structures is provided in the :mod:`zenoh.ext`
module via :func:`zenoh.ext.z_serialize` and :func:`zenoh.ext.z_deserialize`.

Example: Data serialization
~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/data_representation.py
   :language: python
   :start-after: [serialized_data]
   :end-before: # [serialized_data]

.. _encoding:

Encoding
--------

Zenoh uses :class:`zenoh.Encoding` to indicate how data should be interpreted by the application. An encoding has a similar role to Content-Type in HTTP and is represented as a string in MIME-like format: ``type/subtype[;schema]``.

To optimize network usage, Zenoh internally maps some predefined encoding strings to integer identifiers. These encodings are provided as class attributes of the :class:`zenoh.Encoding` class, such as :attr:`zenoh.Encoding.ZENOH_BYTES`, :attr:`zenoh.Encoding.APPLICATION_JSON`, etc. This internal mapping is not exposed to the application layer, but using these predefined encodings is more efficient than custom strings.

The Zenoh protocol does not impose any encoding value and does not operate on it. It can be seen as optional metadata that is carried over by Zenoh, allowing applications to perform different operations depending on the encoding value.

Additionally, a schema can be associated with the encoding. The convention is to use the ``;`` separator if an encoding is created from a string. Alternatively, :meth:`zenoh.Encoding.with_schema` can be used to add a schema to one of the predefined class attributes.

Example: Creating an :class:`zenoh.Encoding` from a string and vice versa
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/encoding.py
   :language: python
   :start-after: [string_operations]
   :end-before: # [string_operations]

Example: Using the schema
~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/encoding.py
   :language: python
   :start-after: [schema]
   :end-before: # [schema]

.. _scouting:

Scouting
--------

Scouting is the process of discovering Zenoh nodes on the network. The scouting
process depends on the transport layer and the Zenoh configuration. Note that
it is not necessary to explicitly discover other nodes to publish, subscribe, or
query data.

Scouting is performed using the :func:`zenoh.scout` function, which returns a
:class:`zenoh.Scout` object that yields :class:`zenoh.Hello` messages for each
discovered Zenoh node.

Scouting is different from :ref:`liveliness <liveliness>` requesting and monitoring. Liveliness
works on the Zenoh protocol logical level and allows getting information about resources in terms of
:ref:`key expressions <key-expressions>`. On the other hand, :ref:`scouting <scouting>` is about discovering Zenoh nodes visible
to the local node on the network. The result of scouting is a list of :class:`zenoh.Hello` messages,
each containing information about a discovered Zenoh node:

- unique node identifier (:attr:`zenoh.Hello.zid`)
- node type (:attr:`zenoh.Hello.whatami`)
- list of node's network addresses (:attr:`zenoh.Hello.locators`)

See more details at `scouting documentation <https://zenoh.io/docs/getting-started/deployment/#scouting>`_.

Example: Scouting for Zenoh nodes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/scouting.py
   :language: python
   :start-after: [scouting]
   :end-before: # [scouting]

.. _liveliness:

Liveliness
----------

Zenoh supports liveliness monitoring to notify when a specified resource appears
or disappears on the network.

Sometimes it is necessary to know whether a Zenoh node is available. This
can be achieved by declaring special publishers and queryables, but the 
dedicated liveliness API is more convenient and efficient.

The :class:`zenoh.Liveliness` object is created by calling :meth:`zenoh.Session.liveliness`.
It allows a node to declare a :class:`zenoh.LivelinessToken` associated with a key expression.
To declare the token, use :meth:`zenoh.Liveliness.declare_token`.

Other nodes can query this key expression using :meth:`zenoh.Liveliness.get`.
They can also subscribe using :meth:`zenoh.Liveliness.declare_subscriber` to be notified when the token appears or disappears.

The `history` parameter of
:meth:`zenoh.Liveliness.declare_subscriber` allows immediate receipt of tokens
that are already present on the network.


Example: Declaring a liveliness token
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/liveliness_token.py
   :language: python
   :start-after: [liveliness_token]
   :end-before: # [liveliness_token]

Example: Getting currently present liveliness tokens
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/liveliness_get.py
   :language: python
   :start-after: [liveliness_get]
   :end-before: # [liveliness_get]

Example: Checking if a liveliness token is present and subscribing to changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/liveliness_subscriber.py
   :language: python
   :start-after: [liveliness_subscriber]
   :end-before: # [liveliness_subscriber]

.. _matching:

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


Example: Declaring a matching listener for a publisher
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/matching_publisher.py
   :language: python
   :start-after: [matching_publisher]
   :end-before: # [matching_publisher]

Example: Declaring a matching listener for a querier
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. literalinclude:: examples/matching_querier.py
   :language: python
   :start-after: [matching_querier]
   :end-before: # [matching_querier]

.. _channels-and-callbacks:

Channels and callbacks
----------------------

Overview
^^^^^^^^

There are two ways to receive sequential data from Zenoh primitives (for
example, a series of :class:`zenoh.Sample` objects from a
:class:`zenoh.Subscriber` or :class:`zenoh.Reply` objects from a
:class:`zenoh.Query`): by channel or by callback.

This behavior is controlled by the ``handler`` parameter of the declare
methods (for example, :meth:`zenoh.Session.declare_subscriber` and
:meth:`zenoh.Session.declare_querier`). The parameter can be either a callable
(a function or a method) or a channel type (blocking
:class:`zenoh.handlers.FifoChannel` or non-blocking :class:`zenoh.handlers.RingChannel`).
By default, the ``handler`` parameter is ``None``, which uses
:class:`zenoh.handlers.DefaultHandler` (a FIFO channel with default capacity).

Channels
~~~~~~~~

When constructed with a :class:`zenoh.handlers.FifoChannel` or :class:`zenoh.handlers.RingChannel`
as ``handler`` (or using the default one), the returned object is iterable
and can be used in a ``for`` loop to receive data sequentially. It also provides explicit
methods such as :meth:`zenoh.Subscriber.recv` to wait for data and
:meth:`zenoh.Subscriber.try_recv` to attempt a non-blocking receive. The
subscriber (or queryable) is automatically undeclared when the object goes out of scope
or when :meth:`zenoh.Subscriber.undeclare` is explicitly called.

.. literalinclude:: examples/channels.py
   :language: python
   :start-after: [channels]
   :end-before: # [channels]

Callbacks
~~~~~~~~~

.. caution::

   Calling Zenoh API functions, as well as performing any blocking operations from within a callback is disallowed.
   Even if this works in some particular cases, it's unsafe and may lead to deadlocks or crashes at any moment or with
   the future updates of the library.

It is possible to pass a callable object as ``handler``. This callable is invoked for each received
:class:`zenoh.Sample` or :class:`zenoh.Reply`. This also means the subscriber or queryable runs in
**background mode**, i.e., it remains active even if the returned object
goes out of scope. This allows declaring a subscriber without managing the
returned object's lifetime.

.. literalinclude:: examples/callback_simple.py
   :language: python
   :start-after: [callback_simple]
   :end-before: # [callback_simple]


For more advanced callback handling, you can use :class:`zenoh.handlers.Callback`
to create a callback handler with cleanup functionality.

.. literalinclude:: examples/callback_advanced.py
   :language: python
   :start-after: [callback_advanced]
   :end-before: # [callback_advanced]


Custom channel implementation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

For advanced use cases, you can implement your own custom channel in Python and pass
it in the tuple form ``(callback, handler)`` where ``callback`` is a callable and ``handler``
is your custom Python object. This solution has the same performance penalties as the
callback API, but it can be useful in some scenarios.

The callback is invoked for each received item and stores the data in the custom channel,
which is accessible via the :meth:`zenoh.Subscriber.handler` property, in the same way
as with built-in channels.

**Custom channel with priority queue**

.. literalinclude:: examples/custom_channel.py
   :language: python
   :start-after: [custom_channel]
   :end-before: # [custom_channel]

**Usage of the custom channel**

.. literalinclude:: examples/custom_channel.py
   :language: python
   :start-after: [custom_channel_usage]
   :end-before: # [custom_channel_usage]
