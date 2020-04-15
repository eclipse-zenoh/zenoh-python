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

## Dependencies
The zenoh-python API depends on the [zenoh-c](https://github.com/eclipse-zenoh/zenoh-c) API. Thus the first thing to do is to ensure that 
**zenoh-c** in installed on your machine. To do so, please follow the instructions provided [here](https://github.com/eclipse-zenoh/zenoh-c/blob/master/README.md).

## Installing the Python API from Sources
To install the API you can do:

    $ python3 setup.py install

Notice that on some platforms, such as Linux, you will need to do this as *sudo*.

<!-- ## Installing the API from PyPi
You can also install the [zenoh](http://zenoh.io)'s python API from PyPi by  simply doing:

    pip3 install zenoh -->
    
## Running the Examples
To run the bundled examples without installing any additional software you can the **zenoh** demo instance 
available at **demo.zenoh.io**. To do so, simply run as follows:

    $ cd zenoh-python/example
    $ python3 sub.py -z demo.zenoh.io

From another terminal:

    $ cd zenoh-python/example
    $ python3 sub.py -z demo.zenoh.io


