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

import logging
import logging.handlers
import sys
import os


class APILogger:
    class __SingletonLogger:
        def __init__(self, level, debug_flag):

            log_format = '[%(asctime)s] - [%(levelname)s] > %(message)s'

            self.logger = logging.getLogger('zenoh.python.api')

            self.logger.setLevel(level)
            formatter = logging.Formatter(log_format)
            if not debug_flag:
                platform = sys.platform
                if platform == 'linux':
                    handler = logging.handlers.SysLogHandler('/dev/log')
                elif platform == 'darwin':
                    handler = logging.handlers.SysLogHandler('/var/run/syslog')
                elif platform in ['windows', 'Windows', 'win32']:
                    handler = logging.handlers.SysLogHandler()
            else:
                handler = logging.StreamHandler(sys.stdout)
            handler.setFormatter(formatter)
            self.logger.addHandler(handler)

        def info(self, caller, message):
            self.logger.info('< {} > {}'.format(caller, message))

        def warning(self, caller, message):
            self.logger.warning('< {} > {}'.format(caller, message))

        def error(self, caller, message):
            self.logger.error('< {} > {}'.format(caller, message))

        def debug(self, caller, message):
            self.logger.debug('< {} > {}'.format(caller, message))

    instance = None
    enabled = True

    def __init__(self, level, debug_flag):

        if not APILogger.instance:
            APILogger.instance = \
                APILogger.__SingletonLogger(level, debug_flag)

    def enable(self):
        self.enabled = True

    def disable(self):
        self.enabled = False

    def info(self, caller, message):
        if self.enabled:
            self.instance.info(caller, message)

    def warning(self, caller, message):
        if self.enabled:
            self.instance.warning(caller, message)

    def error(self, caller, message):
        if self.enabled:
            self.instance.error(caller, message)

    def debug(self, caller, message):
        if self.enabled:
            self.instance.debug(caller, message)
