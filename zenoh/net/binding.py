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

import platform
import os
import ctypes
import traceback
from ctypes import *
from functools import partial
from zenoh.core import Timestamp

ZN_INT_RES_KEY = 0
ZN_STR_RES_KEY = 1


ZN_PUT = 0x00
ZN_UPDATE = 0x01
ZN_REMOVE = 0x02

ZN_STORAGE_DATA = 0x00
ZN_STORAGE_FINAL = 0x01
ZN_EVAL_DATA = 0x02
ZN_EVAL_FINAL = 0x03
ZN_REPLY_FINAL = 0x04

Z_OK_TAG = 0
Z_ERROR_TAG = 1

ZN_SRC_ID = 0x01
ZN_SRC_SN = 0x02
ZN_BRK_ID = 0x04
ZN_BRK_SN = 0x08
ZN_T_STAMP = 0x10
ZN_KIND = 0x20
ZN_ENCODING = 0x40

subscriberCallbackMap = {}
replyCallbackMap = {}
queryHandlerMap = {}
replyMap = {}


def get_lib_ext():
    system = platform.system()
    if system == 'Linux':
        return '.so'
    elif system == 'Darwin':
        return '.dylib'
    else:
        return '.dll'


def get_user_lib_path():
    system = platform.system()
    if system == 'Linux':
        return '/usr/local/lib'
    elif system == 'Darwin':
        return '/usr/local/lib'
    elif system in ['windows', 'Windows', 'win32']:
        return os.environ['ZENOH_HOME']
    else:
        return '/usr/local/lib'


system = platform.system()


if system in ['windows', 'Windows', 'win32']:
    zenohc_lib = 'zenohc' + get_lib_ext()
else:
    zenohc_lib = 'libzenohc' + get_lib_ext()


zenohc_lib_path = os.path.join(os.path.dirname(__file__), '..', zenohc_lib)
if not os.path.exists(zenohc_lib_path):
    print('* WARNING : {} not found along with zenoh python installation '
          '(not present in the wheel?). Try to load it from {}'
          .format(zenohc_lib, get_user_lib_path()))
    zenohc_lib_path = os.path.join(get_user_lib_path(), zenohc_lib)


# zenoh-c result types
class zn_session_p_result_union_t(Union):
    _fields_ = [('session', c_void_p), ('error', c_int)]


class zn_session_p_result_t(Structure):
    _fields_ = [('tag', c_int), ('value', zn_session_p_result_union_t)]


class zn_pub_p_result_union_t(Union):
    _fields_ = [('pub', c_void_p), ('error', c_int)]


class zn_pub_p_result_t(Structure):
    _fields_ = [('tag', c_int), ('value', zn_pub_p_result_union_t)]


class zn_sub_p_result_union_t(Union):
    _fields_ = [('sub', c_void_p), ('error', c_int)]


class zn_sub_p_result_t(Structure):
    _fields_ = [('tag', c_int), ('value', zn_sub_p_result_union_t)]


class zn_sto_p_result_union_t(Union):
    _fields_ = [('sto', c_void_p), ('error', c_int)]


class zn_sto_p_result_t(Structure):
    _fields_ = [('tag', c_int), ('value', zn_sto_p_result_union_t)]


class zn_eval_p_result_union_t(Union):
    _fields_ = [('eval', c_void_p), ('error', c_int)]


class zn_eval_p_result_t(Structure):
    _fields_ = [('tag', c_int), ('value', zn_eval_p_result_union_t)]


class z_vec_t(Structure):
    _fields_ = [('capacity', c_int), ('length', c_int), ('elem', c_void_p)]


# Resource Id
class zn_res_key_t(Union):
    _fields_ = [('rkey', c_int), ('rname', c_char_p)]


class zn_resource_key_t(Structure):
    _fields_ = [('kind', c_int), ('key', zn_res_key_t)]


# Data Info
class zn_data_info_t(Structure):
    _fields_ = [('flags', c_uint),
                ('tstamp', Timestamp),
                ('encoding', c_uint8),
                ('kind', c_ushort)]


class DataInfo():
    """
    Data structure containing meta informations about the associated data.

    kind
        The kind of the data.
    encoding
        The encoding of the data.
    tstamp
        The unique Timestamp at which the data was produced.

    """

    def __init__(self, kind=None, encoding=None, tstamp=None):
        self.kind = kind
        self.tstamp = tstamp
        self.encoding = encoding

    @staticmethod
    def from_zn_data_info(zn_info):
        return DataInfo(
            kind=zn_info.kind if zn_info.flags & ZN_KIND else None,
            encoding=zn_info.encoding if zn_info.flags & ZN_ENCODING else None,
            tstamp=zn_info.tstamp if zn_info.flags & ZN_T_STAMP else None
        )


# Query destination
class zn_query_dest_t(Structure):
    """
    Data structure defining which storages or evals should be destination of a
    query (see Session.query())

    kind
        One of the following destination kinds:
            ZN_BEST_MATCH the nearest complete storage/eval if there is one,
                all storages/evals if not.
            ZN_COMPLETE only complete storages/evals.
            ZN_ALL all storages/evals.
            ZN_NONE no storages/evals.
    nb
        The number of storages or evals that should be destination of the query
        when zn_query_dest_t.kind equals ZN_COMPLETE
    """
    _fields_ = [('kind', c_uint8), ('nb', c_uint8)]


# Temporal properties
class zn_temporal_property_t(Structure):
    _fields_ = [('origin', c_int), ('period', c_int), ('duration', c_int)]


class zn_sub_mode_t(Structure):
    _fields_ = [('kind', c_uint8), ('tprop', zn_temporal_property_t)]


# properties
class z_uint8_array_t(Structure):
    _fields_ = [
        ('length', c_uint),
        ('elem', POINTER(c_char))]


class zn_property_t(Structure):
    _fields_ = [('id', c_size_t), ('value', z_uint8_array_t)]


def dict_to_propsvec(props):
    length = len(props)
    elems = [POINTER(zn_property_t)(zn_property_t(
                key, z_uint8_array_t(
                        len(val),
                        ctypes.create_string_buffer(val, len(val)))))
             for key, val in props.items()]
    return POINTER(z_vec_t)(z_vec_t(
        length, length,
        cast(((POINTER(zn_property_t) * length)(*elems)), c_void_p)))


def propsvec_to_dict(vec):
    vectype = POINTER((POINTER(zn_property_t) * vec.length))
    props = [prop[0] for prop in cast(vec.elem, vectype)[0]]
    return {prop.id: prop.value.elem[:prop.value.length] for prop in props}


CHAR_PTR = POINTER(c_char)


# zenoh-c callbacks
class zn_reply_value_t(Structure):
    _fields_ = [
        ('kind', c_uint8),
        ('srcid', CHAR_PTR),
        ('srcid_length', c_size_t),
        ('rsn', c_uint32),
        ('rname', c_char_p),
        ('data', CHAR_PTR),
        ('data_length', c_size_t),
        ('info', zn_data_info_t)]


class QueryReply(object):
    """
    An object containing one of the replies to a :func:`Session.query`.

    kind
        One of the following:

        | ``ZN_STORAGE_DATA`` the reply contains some data from a storage.
        | ``ZN_STORAGE_FINAL`` the reply indicates that no more data is
            expected from the specified storage.
        | ``ZN_EVAL_DATA`` the reply contains some data from an eval.
        | ``ZN_EVAL_FINAL`` the reply indicates that no more data is expected
            from the specified eval.
        | ``ZN_REPLY_FINAL`` the reply indicates that no more replies are
            expected for the query.

    source_id
        The unique identifier of the storage or eval that sent the reply
        when `kind` equals ``ZN_STORAGE_DATA``, ``ZN_STORAGE_FINAL``,
        ``ZN_EVAL_DATA`` or ``ZN_EVAL_FINAL``.
    seq_num
        The sequence number of the reply from the identified storage or
        eval when `kind` equals ``ZN_STORAGE_DATA``, ``ZN_STORAGE_FINAL``,
        ``ZN_EVAL_DATA`` or ``ZN_EVAL_FINAL``.
    rname
        The resource name of the received data when `kind` equals
        ``ZN_STORAGE_DATA`` or ``ZN_EVAL_DATA``.
    data
        The received data when `kind` equals ``ZN_STORAGE_DATA`` or
        ``ZN_EVAL_DATA``.
    info
        A :class:`DataInfo` object holding meta information about the received
        data when `kind` equals ``ZN_STORAGE_DATA`` or ``ZN_EVAL_DATA``.

    """

    def __init__(self, zrv):
        self.kind = zrv.kind
        self.source_id = None
        self.seq_num = None
        self.rname = None
        self.data = None
        self.info = None

        if(self.kind == ZN_STORAGE_DATA
           or self.kind == ZN_EVAL_DATA):
            self.source_id = zrv.srcid[:zrv.srcid_length]
            self.seq_num = zrv.rsn
            self.rname = zrv.rname.decode()
            self.data = zrv.data[:zrv.data_length]
            self.info = zn_data_info_t()
            self.info.flags = zrv.info.flags
            self.info.tstamp.clock_id = zrv.info.tstamp.clock_id
            self.info.tstamp.time = zrv.info.tstamp.time
            self.info.encoding = zrv.info.encoding
            self.info.kind = zrv.info.kind

        elif(self.kind == ZN_STORAGE_FINAL
             or self.kind == ZN_EVAL_FINAL):
            self.source_id = zrv.srcid[:zrv.srcid_length]


class zn_resource_t(Structure):
    _fields_ = [
        ('rname', c_char_p),
        ('data', c_char_p),
        ('length', c_size_t),
        ('encoding', c_ushort),
        ('kind', c_ushort)
    ]


class zn_resource_p_array_t(Structure):
    _fields_ = [
        ('length', c_uint),
        ('elem', POINTER(POINTER(zn_resource_t)))
    ]


ZENOH_ON_DISCONNECT_CALLBACK_PROTO = CFUNCTYPE(None, c_void_p)
ZENOH_SUBSCRIBER_CALLBACK_PROTO = CFUNCTYPE(None,
                                            POINTER(zn_resource_key_t),
                                            CHAR_PTR, c_uint,
                                            POINTER(zn_data_info_t),
                                            POINTER(c_int64))
ZENOH_REPLY_CALLBACK_PROTO = CFUNCTYPE(None,
                                       POINTER(zn_reply_value_t),
                                       POINTER(c_int64))
ZENOH_SEND_REPLIES_PROTO = CFUNCTYPE(None,
                                     POINTER(c_int64),
                                     zn_resource_p_array_t)
ZENOH_QUERY_HANDLER_PROTO = CFUNCTYPE(None,
                                      c_char_p,
                                      c_char_p,
                                      ZENOH_SEND_REPLIES_PROTO,
                                      POINTER(c_int64),
                                      POINTER(c_int64))


@ZENOH_SUBSCRIBER_CALLBACK_PROTO
def zn_subscriber_trampoline_callback(rkey, data, length, info, arg):
    global subscriberCallbackMap
    key = arg.contents.value
    _, callback = subscriberCallbackMap[key]
    if rkey.contents.kind == ZN_STR_RES_KEY:
        py_info = DataInfo.from_zn_data_info(info.contents)
        callback(rkey.contents.key.rname.decode(), data[:length], py_info)
    else:
        print('WARNING: Received data for unknown  resource name, rkey = {}'
              .format(rkey.key.rid))


@ZENOH_REPLY_CALLBACK_PROTO
def zn_reply_trampoline_callback(reply_value, arg):
    global replyCallbackMap
    key = arg.contents.value
    _, callback = replyCallbackMap[key]
    qr = QueryReply(reply_value.contents)
    callback(qr)


def send_replies_fun(send_replies, query_handle, replies):
    replies_array = zn_resource_p_array_t()
    replies_array.length = len(replies)
    rs = (POINTER(zn_resource_t) * len(replies))()
    replies_array.elem = cast(rs, POINTER(POINTER(zn_resource_t)))
    i = 0
    for k, v in replies:
        d, info = v
        replies_array.elem[i].contents = zn_resource_t()
        replies_array.elem[i].contents.rname = k.encode()
        replies_array.elem[i].contents.data = d
        replies_array.elem[i].contents.length = len(d)

        encoding_not_none = info is not None and info.encoding is not None
        encoding = info.encoding if encoding_not_none else 0
        replies_array.elem[i].contents.encoding = encoding

        kind_not_none = info is not None and info.kind is not None
        kind = info.kind if kind_not_none else 0
        replies_array.elem[i].contents.kind = kind

        tstamp_not_none = info is not None and info.tstamp is not None
        tstamp = info.tstamp if tstamp_not_none else 0
        replies_array.elem[i].contents.tstamp = tstamp

        i += 1
    send_replies(query_handle, replies_array)


@ZENOH_QUERY_HANDLER_PROTO
def zn_query_handler_trampoline(rname,
                                predicate,
                                send_replies,
                                query_handle,
                                arg):
    global queryHandlerMap
    key = arg.contents.value
    _, handler = queryHandlerMap[key]
    try:
        handler(rname.decode(), predicate.decode(),
                partial(send_replies_fun, send_replies, query_handle))
    except Exception:
        print('WARNING: error in query handle for {} :\n{}'
              .format(rname.decode(), traceback.format_exc()))
        send_replies_fun(send_replies, query_handle, [])
