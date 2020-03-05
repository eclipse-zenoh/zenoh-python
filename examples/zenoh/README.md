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

## Examples description

### z_add_storage

   Add a storage in the Zenoh router it's connected to.

   Usage:
   ```bash
   python3 z_add_storage.py [selector] [storage-id] [locator]
   ```
   where the optional arguments are:
   - **selector** :  the selector matching the keys (path) that have to be stored.  
                     Default value: `/demo/example/**`
   - **storage-id** : the storage identifier.  
                      Default value: `Demo` 
   - **locator** : the locator of the Zenoh router to connect.  
                   Default value: none, meaning the Zenoh router is found via multicast.

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
   python3 z_put.py [path] [value] [locator]
   ```
   where the optional arguments are:
   - **path** : the path used as a key for the value.  
                Default value: `/demo/example/zenoh-python-put` 
   - **value** : the value (as a string).  
                Default value: `"Put from Zenoh Python!"` 
   - **locator** : the locator of the Zenoh router to connect.  
                   Default value: none, meaning the Zenoh router is found via multicast.

### z_get

   Get a list of keys/values from Zenoh.  
   The values will be retrieved from the Storages containing paths that match the specified selector.  
   The Eval functions (see [z_eval](#z_eval) below) registered with a path matching the selector
   will also be triggered.

   Usage:
   ```bash
   python3 z_get.py [selector] [locator]
   ```
   where the optional arguments are:
   - **selector** : the selector that all replies shall match.  
                    Default value: `/demo/example/**` 
   - **locator** : the locator of the Zenoh router to connect.  
                   Default value: none, meaning the Zenoh router is found via multicast.

### z_remove

   Remove a key and its associated value from Zenoh.  
   Any storage that store the key/value will drop it.  
   The subscribers with a selector matching the key will also receive a notification of this removal.

   Usage:
   ```bash
   python3 z_remove [path] [locator]
   ```
   where the optional arguments are:
   - **path** : the key to be removed.  
                Default value: `/demo/example/zenoh-python-put` 
   - **locator** : the locator of the Zenoh router to connect.  
                   Default value: none, meaning the Zenoh router is found via multicast.

### z_sub

   Register a subscriber with a selector.  
   The subscriber will be notified of each put/remove made on any path matching the selector,
   and will print this notification.

   Usage:
   ```bash
   python3 z_sub.py [selector] [locator]
   ```
   where the optional arguments are:
   - **selector** : the subscription selector.  
                    Default value: `/demo/example/**` 
   - **locator** : the locator of the Zenoh router to connect.  
                   Default value: none, meaning the Zenoh router is found via multicast.

### z_eval

   Register an evaluation function with a path.  
   This evaluation function will be triggered by each call to a get operation on Zenoh 
   with a selector that matches the path. In this example, the function returns a string value.
   See the code for more details.

   Usage:
   ```bash
   python3 z_eval.py [selector] [locator]
   ```
   where the optional arguments are:
   - **path** : the eval path.  
                Default value: `/demo/example/zenoh-python-eval` 
   - **locator** : the locator of the Zenoh router to connect.  
                   Default value: none, meaning the Zenoh router is found via multicast.

### z_pub_thr & z_sub_thr

   Pub/Sub throughput test.
   This example allows to perform throughput measurements between a pubisher performing
   put operations and a subscriber receiving notifications of those put.
   Note that you can run this example with or without any storage.

   Publisher usage:
   ```bash
   python3 z_pub_thr.py <payload-size> [locator]
   ```
   where the arguments are:
   - **payload-size** : the size of the payload in bytes.  
   - **locator** : the locator of the Zenoh router to connect.  
                   Default value: none, meaning the Zenoh router is found via multicast.

   Subscriber usage:
   ```bash
   python3 z_sub_thr.py [locator]
   ```
   where the optional arguments are:
   - **locator** : the locator of the Zenoh router to connect.  
                   Default value: none, meaning the Zenoh router is found via multicast.
