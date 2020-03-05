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


class ZException(Exception):
    def __get_code_name(self, code):
        codes = {
            0x01: 'Z_VLE_PARSE_ERROR',
            0x02: 'Z_ARRAY_PARSE_ERROR',
            0x03: 'Z_STRING_PARSE_ERROR',
            0x81: 'ZN_PROPERTY_PARSE_ERROR',
            0x82: 'ZN_PROPERTIES_PARSE_ERROR',
            0x83: 'ZN_MESSAGE_PARSE_ERROR',
            0x84: 'ZN_INSUFFICIENT_IOBUF_SIZE',
            0x85: 'ZN_IO_ERROR',
            0x86: 'ZN_RESOURCE_DECL_ERROR',
            0x87: 'ZN_PAYLOAD_HEADER_PARSE_ERROR',
            0x89: 'ZN_TX_CONNECTION_ERROR',
            0x8a: 'ZN_INVALID_ADDRESS_ERROR',
            0x8b: 'ZN_FAILED_TO_OPEN_SESSION',
            0x8c: 'ZN_UNEXPECTED_MESSAGE'
        }
        return codes.get(code, 'UNKOWN_ERROR_CODE(' + str(code) + ')')

    def __init__(self, message, code=0, cause=None):
        if code != 0:
            message += ' (error code: ' + self.__get_code_name(code) + ')'
        if cause is not None:
            message = message + '. Caused by: ' + cause.str()
        super().__init__(message)
        self.code = code
        self.cause = cause
