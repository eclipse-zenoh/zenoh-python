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
>>> with zenoh.open() as session:
>>>     session.put('demo/example/hello', 'Hello World!')

Subscribe to a set of keys with Zenoh
"""""""""""""""""""""""""""""""""""""

>>> import zenoh, time
>>> def listener(sample):
>>>     print(f"{sample.key_expr} => {sample.payload.decode('utf-8')}")
>>>
>>> with zenoh.open() as session:
>>>     with session.declare_subscriber('demo/example/**', listener) as subscriber:
>>>         time.sleep(60)

Get keys/values from zenoh
""""""""""""""""""""""""""

>>> import zenoh
>>> with zenoh.open() as session:
>>>     for response in session.get('demo/example/**'):
>>>         response = response.ok
>>>         print(f"{response.key_expr} => {response.payload.deserialize(str)}")

module zenoh
============

.. automodule:: zenoh
    :members:

module zenoh.handlers
============

.. automodule:: zenoh.handlers
    :members:
