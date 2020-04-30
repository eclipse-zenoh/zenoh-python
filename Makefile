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

# -*-Makefile-*-

# zenoh-python/ directory
ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))


all:
	python3 setup.py sdist bdist_wheel

install:
	python3 setup.py install --record zenoh_files.txt

DOCKCROSS_x86_IMAGE=dockcross/manylinux2010-x86
DOCKCROSS_x64_IMAGE=dockcross/manylinux2010-x64

DOCKER_OK := $(shell docker version 2> /dev/null)
DOCKCROSS_x86_INFO := $(shell docker image inspect $(DOCKCROSS_x86_IMAGE) 2> /dev/null)
DOCKCROSS_x64_INFO := $(shell docker image inspect $(DOCKCROSS_x64_IMAGE) 2> /dev/null)

check-docker:
ifndef DOCKER_OK
	$(error "Docker is not available. Please install Docker")
endif
ifeq ($(DOCKCROSS_x86_INFO),[])
	docker pull $(DOCKCROSS_x86_IMAGE)
endif
ifeq ($(DOCKCROSS_x64_INFO),[])
	docker pull $(DOCKCROSS_x64_IMAGE)
endif

all-cross:
	docker run --rm -v $(ROOT_DIR):/workdir -w /workdir $(DOCKCROSS_x86_IMAGE) bash -c " \
		/opt/python/cp35-cp35m/bin/python setup.py bdist_wheel --dist-dir dist/dockcross-x86 && \
		/opt/python/cp36-cp36m/bin/python setup.py bdist_wheel --dist-dir dist/dockcross-x86 && \
		/opt/python/cp37-cp37m/bin/python setup.py bdist_wheel --dist-dir dist/dockcross-x86 && \
		/opt/python/cp38-cp38/bin/python setup.py bdist_wheel --dist-dir dist/dockcross-x86 && \
		for i in dist/dockcross-x86/*; do auditwheel repair \$$i -w dist; done "
	docker run --rm -v $(ROOT_DIR):/workdir -w /workdir $(DOCKCROSS_x64_IMAGE) bash -c " \
		/opt/python/cp35-cp35m/bin/python setup.py bdist_wheel --dist-dir dist/dockcross-x64 && \
		/opt/python/cp36-cp36m/bin/python setup.py bdist_wheel --dist-dir dist/dockcross-x64 && \
		/opt/python/cp37-cp37m/bin/python setup.py bdist_wheel --dist-dir dist/dockcross-x64 && \
		/opt/python/cp38-cp38/bin/python setup.py bdist_wheel --dist-dir dist/dockcross-x64 && \
		for i in dist/dockcross-x64/*; do auditwheel repair \$$i -w dist; done "

clean:
	rm -rf ./build ./_skbuild ./dist ./zenoh.egg-info .coverage zenoh/include zenoh/libzenohc.* zenoh/zenohc.*;
	rm -rf zenoh_api.log .tox ./zenoh/__pycache__/ ./zenoh/*/__pycache__/ ./zenoh/*/*/__pycache__/;

test: clean
	tox

doc:
	cd docs && $(MAKE) clean
	cd docs && $(MAKE) latexpdf
	cd docs && $(MAKE) dirhtml
