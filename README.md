![zenoh banner](./zenoh-dragon.png)

![Build](https://github.com/eclipse-zenoh/zenoh-python/workflows/Python%20package/badge.svg)
[![Documentation Status](https://readthedocs.org/projects/zenoh-python/badge/?version=latest)](https://zenoh-python.readthedocs.io/en/latest/?badge=latest)
[![Gitter](https://badges.gitter.im/atolab/zenoh.svg)](https://gitter.im/atolab/zenoh?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)
[![License](https://img.shields.io/badge/License-EPL%202.0-blue)](https://choosealicense.com/licenses/epl-2.0/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

# Eclipse zenoh Python API

[Eclipse zenoh](http://zenoh.io) is an extremely efficient and fault-tolerant [Named Data Networking](http://named-data.net) (NDN) protocol 
that is able to scale down to extremely constrainded devices and networks. 

The Python API is for pure clients, in other terms does not support peer-to-peer communication, can be easily
tested with our demo instace available at **demo.zenoh.io**.

-------------------------------
## How to install it

Eclipse zenoh-python library is available on [Pypi.org](https://pypi.org/project/eclipse-zenoh/).  
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
## How to test it

Run a zenoh router on your local host, following the instructions in [eclipse-zenoh/zenoh](https://github.com/eclipse-zenoh/zenoh#how-to-test-it)

Then, run the zenoh-python examples following the instructions in [examples/zenoh/README.md](https://github.com/eclipse-zenoh/zenoh-python/blob/master/examples/zenoh/README.md)

