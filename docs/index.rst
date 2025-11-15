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

**************************
Zenoh Python API Reference
**************************

`Zenoh <https://zenoh.io>`_ /zeno/ is a stack that unifies data in motion, data at
rest and computations. It elegantly blends traditional pub/sub with geo distributed
storage, queries and computations, while retaining a level of time and space efficiency
that is well beyond any of the mainstream stacks.

The Zenoh protocol allows nodes to form a graph with an arbitrary topology, such as a mesh, 
a star, or a clique. The zenoh routers keeps the network connected and routes the messages
between the nodes.

This documentation provides an overview of the Zenoh concepts and components and a
reference of the Zenoh python API. For more information about Zenoh, please visit the
documentation section on the `Zenoh website <https://zenoh.io/docs/getting-started/first-app/>`_.
It's useful to consult also the `Zenoh Rust API <https://docs.rs/crate/zenoh/latest/>`_ 
reference since the Python API is a binding over the Rust implementation.

All examples presented in this documentation can be found in the examples/ directory of the
`Zenoh Python GitHub repository <https://github.com/eclipse-zenoh/zenoh-python/tree/main/docs/examples>`_.

Documentation Contents
======================

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   quickstart
   concepts
   api_reference

Indices and Tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
