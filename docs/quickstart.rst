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

Quick Start Examples
====================

Below are some examples that highlight these key concepts and show how easy it is to get
started with. The more detailed documentation is available in the other sections.

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