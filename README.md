![zenoh banner](./zenoh-dragon.png)

[![CI](https://github.com/eclipse-zenoh/zenoh-python/workflows/CI/badge.svg)](https://github.com/eclipse-zenoh/zenoh-python/actions?query=workflow%3A%22CI%22)
[![Documentation Status](https://readthedocs.org/projects/zenoh-python/badge/?version=latest)](https://zenoh-python.readthedocs.io/en/latest/?badge=latest)
[![Gitter](https://badges.gitter.im/atolab/zenoh.svg)](https://gitter.im/atolab/zenoh?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)
[![License](https://img.shields.io/badge/License-EPL%202.0-blue)](https://choosealicense.com/licenses/epl-2.0/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

# Eclipse zenoh Python API

[Eclipse zenoh](http://zenoh.io) is an extremely efficient and fault-tolerant [Named Data Networking](http://named-data.net) (NDN) protocol 
that is able to scale down to extremely constrainded devices and networks.

-------------------------------
## How to install it

The Eclipse zenoh-python library is available on [Pypi.org](https://pypi.org/project/eclipse-zenoh/).  
Install the latest available version using `pip`:
```
pip install eclipse-zenoh
```

:warning:WARNING:warning: zenoh-python is developped in Rust.
On Pypi.org we provide binary wheels for the most common platforms (MacOS, Linux x86). But also a source distribution package for other platforms.  
However, for `pip` to be able to build this source distribution, there some prerequisites:
 - `pip` version 19.3.1 minimum (for full support of PEP 517).  
   (if necessary upgrade it with command: `'sudo pip install --upgrade pip'` )
 - Have a Rust toolchain installed (instructions at https://rustup.rs/)

### Supported Python versions and platforms

zenoh-python has been tested with Python 3.6, 3.7, 3.8 and 3.9.

It relies on the [zenoh](https://github.com/eclipse-zenoh/zenoh/tree/master/zenoh) Rust API which require the full `std` library. See the list Rust supported platforms here: https://doc.rust-lang.org/nightly/rustc/platform-support.html .


-------------------------------
## How to build it

Requirements:
 * Python >= 3.6
 * pip >= 19.3.1
 * [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Steps:
 * Install developments requirements:
   ```bash
   pip install -r requirements-dev.txt
   ```
 * Build zenoh-python
   ```bash
   python setup.py develop
   ```

This will automatically build the zenoh Rust API, as well as the zenoh-python API and install it in your Python environement.

-------------------------------
## Running the Examples

The simplest way to run some of the example is to get a Docker image of the **zenoh** network router (see https://github.com/eclipse-zenoh/zenoh#how-to-test-it) and then to run the examples on your machine.

Then, run the zenoh-python examples following the instructions in [examples/zenoh/README.md](https://github.com/eclipse-zenoh/zenoh-python/blob/master/examples/zenoh/README.md)
