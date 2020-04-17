# Copyright (c) 2014, 2020 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
# Angelo Corsaro <angelo.corsaro@adlinktech.com>
# Olivier Hecart <olivier.hecart@adlinktech.com>
# Julien Enoch   <julien.enoch@adlinktech.com>

# Zenoh Python examples

## Prerequisites

   The [Zenoh](https://zenoh.io) C client library and Python API must be installed on your host.
   See installation instructions on https://zenoh.io or clone, build and install it yourself from https://github.com/eclipse-zenoh/zenoh-c and https://github.com/eclipse-zenoh/zenoh-python.

## Start instructions
   
   ```bash
   python3 <example.py>
   ```

   Each example accepts the -h or --help option that provides a description of its arguments and their default values.

## Examples description

### z_add_storage

   Add a storage in the Zenoh router it's connected to.

   Usage:
   ```bash
   python3 z_add_storage.py [--selector SELECTOR] [--id ID] [--locator LOCATOR]
   ```

   Note that his example doesn't specify the Backend that Zenoh has to use for storage creation.  
   Therefore, Zenoh will automatically select the memory backend, meaning the storage will be in memory
   (i.e. not persistent).

### z_put

   Put a key/value into Zenoh.  
   The key/value will be stored by all the storages with a selector that matches the key.
   It will also be received by all the matching subscribers (see [z_sub](#z_sub) below).  
   Note that if no storage and no subscriber are matching the key, the key/value will be dropped.
   Therefore, you probably should run [z_add_storage](#z_add_storage) and/or [z_sub](#z_sub) before YPut.

   Usage:
   ```bash
   python3 z_put.py [--path PATH] [--locator LOCATOR] [--msg MSG]
   ```

### z_get

   Get a list of keys/values from Zenoh.  
   The values will be retrieved from the Storages containing paths that match the specified selector.  
   The Eval functions (see [z_eval](#z_eval) below) registered with a path matching the selector
   will also be triggered.

   Usage:
   ```bash
   python3 z_get.py [--selector SELECTOR] [--locator LOCATOR]
   ```

### z_remove

   Remove a key and its associated value from Zenoh.  
   Any storage that store the key/value will drop it.  
   The subscribers with a selector matching the key will also receive a notification of this removal.

   Usage:
   ```bash
   python3 z_remove [--path PATH] [--locator LOCATOR]
   ```

### z_sub

   Register a subscriber with a selector.  
   The subscriber will be notified of each put/remove made on any path matching the selector,
   and will print this notification.

   Usage:
   ```bash
   python3 z_sub.py [--selector SELECTOR] [--locator LOCATOR]
   ```
   where the optional arguments are:

### z_eval

   Register an evaluation function with a path.  
   This evaluation function will be triggered by each call to a get operation on Zenoh 
   with a selector that matches the path. In this example, the function returns a string value.
   See the code for more details.

   Usage:
   ```bash
   python3 z_eval.py [--path PATH] [--locator LOCATOR]
   ```
   where the optional arguments are:

### z_pub_thr & z_sub_thr

   Pub/Sub throughput test.
   This example allows to perform throughput measurements between a pubisher performing
   put operations and a subscriber receiving notifications of those put.
   Note that you can run this example with or without any storage.

   Publisher usage:
   ```bash
   python3 z_pub_thr.py [--size SIZE] [--locator LOCATOR] [--path PATH]
   ```

   Subscriber usage:
   ```bash
   python3 z_sub_thr.py [--path PATH] [--locator LOCATOR]
   ```
