#!/usr/bin/env python3

# Copyright (c) 2017, 2020 ADLINK Technology Inc.
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ADLINK zenoh team, <zenoh@adlink-labs.tech>

from skbuild import setup
import os


def read(fname):
    return open(os.path.join(os.path.dirname(__file__), fname)).read()


setup(
    name='zenoh',
    version='0.4.2.dev3',
    packages=['zenoh', 'zenoh.net', 'zenoh.core'],
    author='ADLINK zenoh team',
    author_email='zenoh@adlink-labs.tech',
    description="Python client API for Eclipse zenoh",
    long_description=read('README.md'),
    long_description_content_type="text/markdown",
    url='https://github.com/eclipse-zenoh/zenoh-python',
    install_requires=['hexdump', 'mvar', 'papero==0.2.7'],
    classifiers=[
        'Programming Language :: Python :: 3',
        'Intended Audience :: Developers',
        'Development Status :: 4 - Beta',
        'Topic :: System :: Networking',
        'License :: OSI Approved :: Apache Software License',
        'License :: OSI Approved :: Eclipse Public License 2.0 (EPL-2.0)',
        'Operating System :: POSIX :: Linux',
        'Operating System :: MacOS :: MacOS X'],
    cmake_source_dir='zenoh-c',
    cmake_args=['-DCMAKE_MAKE_PROGRAM=make']
    )
