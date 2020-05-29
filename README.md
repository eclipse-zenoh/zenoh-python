![zenoh banner](./zenoh-dragon.png)

![Build](https://github.com/eclipse-zenoh/zenoh-python/workflows/Python%20package/badge.svg)
[![Documentation Status](https://readthedocs.org/projects/zenoh-python/badge/?version=latest)](https://zenoh-python.readthedocs.io/en/latest/?badge=latest)
[![Gitter](https://badges.gitter.im/atolab/zenoh.svg)](https://gitter.im/atolab/zenoh?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)
[![License](https://img.shields.io/badge/License-EPL%202.0-blue)](https://choosealicense.com/licenses/epl-2.0/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

# Eclipse zenoh Python API

[Eclipse zenoh](http://zenoh.io) is an extremely efficient and fault-tolerant [Named Data Networking](http://named-data.net) (NDN) protocol 
that is able to scale down to extremely constrainded devices and networks. 

The Python API is for pure clients, in other terms does not support peer-to-peer communication, can be easily tested against a zenoh router running in a Docker container (see https://github.com/eclipse-zenoh/zenoh#how-to-test-it). 

-------------------------------
## How to install it

The Eclipse zenoh-python library is available on [Pypi.org](https://pypi.org/project/eclipse-zenoh/).  
Install the latest available version using `pip`:
```
pip install eclipse-zenoh
```

### Supported Python versions and platforms

zenoh-python has been tested with Python 3.5, 3.6, 3.7 and 3.8. 

It relies on the [zenoh-c](https://github.com/eclipse-zenoh/zenoh-c) API which is currently supported on the following platforms:
 * Linux
 * MacOS X

Notice that the Python wheels available on [Pypi.org](https://pypi.org/project/zenoh/) are pre-compiled for:
 * All MacOS X >= 10.9
 * Most of the 32-bits Linux distriutions thanks to manylinux2010_i686
 * Most of the 64-bits Linux distriutions thanks to manylinux2010_x86_64

On other Linux architectures such as Raspberry, the `pip` tool will be able to download the sources distribution and to compile it.

-------------------------------
## How to build it 

The zenoh-python repository uses the [zenoh-c](https://github.com/eclipse-zenoh/zenoh-c) repository as a sub-module. Thus, be sure to also clone this submodule.  
Also make sure to have [cmake >= 3.0](https://cmake.org) available on your host.

For convenience, a top-level Makefile is available. Just do the following to build and install:

  ```bash
  $ cd /path/to/zenoh-python
  $ make
  $ make install # on linux use **sudo**
  ```


-------------------------------
## Running the Examples

The simplest way to run some of the example is to get a Docker image of the **zenoh** network router (see https://github.com/eclipse-zenoh/zenoh#how-to-test-it) and then to run the examples on your machine.

Then, run the zenoh-python examples following the instructions in [examples/zenoh/README.md](https://github.com/eclipse-zenoh/zenoh-python/blob/master/examples/zenoh/README.md)

