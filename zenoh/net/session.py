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

from .binding import *
import zenoh.core
from zenoh.core import *
import socket
import time

ZN_USER_KEY = 0x50
ZN_PASSWD_KEY = 0x51

ZN_INFO_PID_KEY = 0
ZN_INFO_PEER_KEY = 1
ZN_INFO_PEER_PID_KEY = 2


class SubscriberMode(object):
    """
    An object representing a subscription mode
    (see :func:`Session.declare_subscriber`).

    kind
        One of the following:

        | ``SubscriberMode.ZN_PUSH_MODE``
        | ``SubscriberMode.ZN_PULL_MODE``
        | ``SubscriberMode.ZN_PERIODIC_PUSH_MODE``
        | ``SubscriberMode.ZN_PERIODIC_PULL_MODE``

    tprop
        A temporal property representing the period. `Unsupported`

    """

    ZN_PUSH_MODE = 1
    ZN_PULL_MODE = 2
    ZN_PERIODIC_PUSH_MODE = 3
    ZN_PERIODIC_PULL_MODE = 4

    def __init__(self, kind, tprop):
        self.zn_sm = zn_sub_mode_t()
        self.zn_sm.kind = c_uint8(kind)
        self.zn_sm.tprop.origin = 0
        self.zn_sm.tprop.period = 0
        self.zn_sm.tprop.duration = 0

        if tprop is not None:
            self.zn_sm.tprop.origin = tprop.origin
            self.zn_sm.tprop.period = tprop.period
            self.zn_sm.tprop.duration = tprop.duration

    @staticmethod
    def push():
        """
        Return a SubscriberMode instance with `kind` =
        ``SubscriberMode.ZN_PUSH_MODE``

        :returns: the equivalent of
            ``SubscriberMode(SubscriberMode.ZN_PUSH_MODE, None)``
        """
        return SubscriberMode(SubscriberMode.ZN_PUSH_MODE, None)

    @staticmethod
    def pull():
        """
        Return a SubscriberMode instance with `kind` =
        ``SubscriberMode.ZN_PULL_MODE``

        :returns: the equivalent of
            ``SubscriberMode(SubscriberMode.ZN_PULL_MODE, None)``
        """
        return SubscriberMode(SubscriberMode.ZN_PULL_MODE, None)


class QueryDest(object):
    """
    An object defining which storages or evals should be destination of a
    query (see :func:`Session.query`).

    kind
        One of the following:

        | ``SubscriberMode.ZN_BEST_MATCH``
        | ``SubscriberMode.ZN_COMPLETE``
        | ``SubscriberMode.ZN_ALL``
        | ``SubscriberMode.ZN_NONE``

    nb
        The number of storages or evals that should be destination of
        the query when `kind` equals ``SubscriberMode.ZN_COMPLETE``.

    """
    ZN_BEST_MATCH = 0
    ZN_COMPLETE = 1
    ZN_ALL = 2
    ZN_NONE = 3

    def __init__(self, kind, nb=1):
        self.zn_qd = zn_query_dest_t()
        self.zn_qd.kind = kind
        self.zn_qd.nb = nb


def zn_to_canonical_locator(locator):
    if locator is None:
        return None
    locator = locator.strip()
    a, b, c = locator.partition('/')
    if a == 'tcp' and b == '/':
        h, s, p = c.partition(':')
        if s == ':' and p != '':
            return ('tcp/' + socket.gethostbyname(h) + ':' + p).encode()
        else:
            raise ZException('Invalid locator format {}, it should be '
                             'tcp/<ip-addr|host-name>:port'.format(locator))
    elif b == '':
        h, s, p = locator.partition(':')
        if s == ':':
            return ('tcp/' + socket.gethostbyname(h) + ':' + p).encode()
        else:
            return ('tcp/' + socket.gethostbyname(h) + ':7447').encode()


def zn_rname_intersect(a, b):
    if Session.zenohc_native_lib.zn_rname_intersect(
        a.encode(), b.encode()) == 1:
        return True
    else:
        return False


class Session(object):
    """
    An object that represents a zenoh session.
    """

    zenohc_native_lib = None

    def __init__(self, locator, properties={}):
        if Session.zenohc_native_lib is None:
            Session.zenohc_native_lib = CDLL(zenohc_lib_path)

        self.zlib = Session.zenohc_native_lib

        self.zlib.zn_open.restype = zn_session_p_result_t
        self.zlib.zn_open.argtypes = [c_char_p, c_void_p, POINTER(z_vec_t)]

        self.zlib.zn_info.restype = z_vec_t
        self.zlib.zn_info.argtypes = [c_void_p]

        self.zlib.zn_running.restype = c_int
        self.zlib.zn_running.argtypes = [c_void_p]

        self.zlib.zn_declare_subscriber.restype = zn_sub_p_result_t
        self.zlib.zn_declare_subscriber.argtypes = [
            c_void_p, c_char_p, POINTER(zn_sub_mode_t),
            ZENOH_SUBSCRIBER_CALLBACK_PROTO, POINTER(c_int64)]

        self.zlib.zn_declare_storage.restype = zn_sto_p_result_t
        self.zlib.zn_declare_storage.argtypes = [
            c_void_p, c_char_p, ZENOH_SUBSCRIBER_CALLBACK_PROTO,
            ZENOH_QUERY_HANDLER_PROTO, POINTER(c_int64)]

        self.zlib.zn_declare_eval.restype = zn_eval_p_result_t
        self.zlib.zn_declare_eval.argtypes = [
            c_void_p, c_char_p, ZENOH_QUERY_HANDLER_PROTO,
            POINTER(c_int64)]

        self.zlib.zn_declare_publisher.restype = zn_pub_p_result_t
        self.zlib.zn_declare_publisher.argtypes = [c_void_p, c_char_p]

        self.zlib.zn_start_recv_loop.restype = c_int
        self.zlib.zn_start_recv_loop.argtypes = [c_void_p]

        self.zlib.zn_stop_recv_loop.restype = c_int
        self.zlib.zn_stop_recv_loop.argtypes = [c_void_p]

        self.zlib.zn_stream_compact_data.restype = c_int
        self.zlib.zn_stream_compact_data.argtypes = [
            c_void_p, POINTER(c_char), c_int]

        self.zlib.zn_stream_data_wo.restype = c_int
        self.zlib.zn_stream_data_wo.argtypes = [
            c_void_p, c_char_p, c_int, c_uint8, c_uint8]

        self.zlib.zn_write_data_wo.restype = c_int
        self.zlib.zn_write_data_wo.argtypes = [
            c_void_p, c_char_p, c_char_p, c_int, c_uint8, c_uint8]

        self.zlib.zn_pull.restype = c_int
        self.zlib.zn_pull.argtypes = [c_void_p]

        self.zlib.zn_query_wo.restype = c_int
        self.zlib.zn_query_wo.argtypes = [
            c_void_p, c_char_p, c_char_p,
            ZENOH_REPLY_CALLBACK_PROTO, POINTER(c_int64),
            zn_query_dest_t, zn_query_dest_t]

        self.zlib.zn_undeclare_subscriber.restype = c_int
        self.zlib.zn_undeclare_subscriber.argtypes = [c_void_p]

        self.zlib.zn_undeclare_storage.restype = c_int
        self.zlib.zn_undeclare_storage.argtypes = [c_void_p]

        self.zlib.zn_undeclare_eval.restype = c_int
        self.zlib.zn_undeclare_eval.argtypes = [c_void_p]

        self.zlib.zn_undeclare_publisher.restype = c_int
        self.zlib.zn_undeclare_publisher.argtypes = [c_void_p]

        self.zlib.zn_close.restype = c_int
        self.zlib.zn_close.argtypes = [c_void_p]

        self.zlib.zn_rname_intersect.restype = c_int
        self.zlib.zn_rname_intersect.argtypes = [c_char_p, c_char_p]

        loc = zn_to_canonical_locator(locator)

        r = self.zlib.zn_open(loc, 0, dict_to_propsvec(properties))
        if r.tag == Z_OK_TAG:
            self.session = r.value.session
            self.connected = True
        else:
            raise ZException('Unable to open zenoh-net session', r.value.error)

        self.zlib.zn_start_recv_loop(self.session)
        while not self.running:
            time.sleep(0.01)

    @staticmethod
    def open(locator=None, properties={}):
        """
        Open a zenoh-net session.

        :param locator: a string representing the network endpoint to which
            establish the session. A typical locator looks like this :
            ``tcp/127.0.0.1:7447``. If ``None``, :func:`open` will scout and
            try to establish the session automatically.
        :param properties: a {int: bytes} dictionary of properties that will
            be used to establish and configure the zenoh session.
            **properties** will typically contain the username and
            password informations needed to establish the zenoh session
            with a secured infrastructure. It can be set to ``NULL``.
        :returns: a handle to the zenoh-net session.

        """
        return Session(locator, properties)

    def info(self):
        """
        Return a {int: bytes} dictionary of properties containing various
        informations about the established zenoh-net session.

        :returns: a {int: bytes} dictionary of properties.

        """
        return propsvec_to_dict(self.zlib.zn_info(self.session))

    @property
    def running(self):
        return (self.zlib.zn_running(self.session) != 0)

    def declare_publisher(self, resource):
        """
        Declare a publication for resource name **resource**.

        :param resource: is the resource name to publish.
        :returns: a zenoh-net publisher.

        """
        r = self.zlib.zn_declare_publisher(self.session, resource.encode())
        if r.tag == 0:
            return r.value.pub
        else:
            raise ZException('Unable to create publisher', r.value.error)

    def declare_subscriber(self, resource, sub_mode, callback):
        """
        Declare a subscription for all published data matching the provided
        resource name **resource**.

        :param resource: the resource name to subscribe to.
        :param sub_mode: the subscription mode.
        :param callback: the callback function that will be called each time a
            data matching the subscribed resource namle **resource** is
            received.
        :returns: a zenoh-net subscriber.

        """
        global subscriberCallbackMap
        h = hash(callback)
        k = POINTER(c_int64)(c_int64(h))
        r = self.zlib.zn_declare_subscriber(self.session,
                                            resource.encode(),
                                            byref(sub_mode.zn_sm),
                                            zn_subscriber_trampoline_callback,
                                            k)
        subscriberCallbackMap[h] = (k, callback)
        if r.tag == 0:
            return r.value.sub
        else:
            del subscriberCallbackMap[h]
            raise ZException('Unable to create subscriber', r.value.error)

    def declare_storage(self, resource, subscriber_callback, query_handler):
        """
        Declare a storage for all data matching the provided resource name
        **resource**.

        :param resource: the the resource selection to store.
        :param subscriber_callback: the callback function that will be called
            each time a data matching the stored resource name **resource**
            is received.
        :param query_handler: the callback function that will be called each
            time a query for data matching the stored resource name
            **resource** is received.
            The **query_handler** function MUST call the provided
            **send_replies** function with the resulting data.
            **send_replies** can be called with an empty array.
        :returns: a zenoh-net storage.

        """
        global replyCallbackMap
        h = hash(query_handler)
        k = POINTER(c_int64)(c_int64(h))
        subscriberCallbackMap[h] = (k, subscriber_callback)
        queryHandlerMap[h] = (k, query_handler)
        r = self.zlib.zn_declare_storage(
                self.session,
                resource.encode(),
                zn_subscriber_trampoline_callback,
                zn_query_handler_trampoline,
                k)
        if r.tag == 0:
            return r.value.sto
        else:
            del subscriberCallbackMap[h]
            del replyCallbackMap[h]
            raise ZException('Unable to create storage', r.value.error)

    def declare_eval(self, resource, query_handler):
        """
        Declare an eval able to provide data matching the provided resource
        name **resource**.

        :param resource: the resource to evaluate.
        :param query_handler: is the callback function that will be called
            each time a query for data matching the evaluated resource name
             **resource** is received. The **query_handler** function MUST
             call the provided **send_replies** function with the resulting
             data. **send_replies** can be called with an empty array.
        :returns: a zenoh eval.

        """
        global replyCallbackMap
        h = hash(query_handler)
        k = POINTER(c_int64)(c_int64(h))
        queryHandlerMap[h] = (k, query_handler)
        r = self.zlib.zn_declare_eval(
                self.session,
                resource.encode(),
                zn_query_handler_trampoline,
                k)
        if r.tag == 0:
            return r.value.eval
        else:
            del replyCallbackMap[h]
            raise ZException('Unable to create eval', r.value.error)

    def stream_compact_data(self, pub, payload):
        """
        Send data in a *compact_data* message for the resource published by
        publisher **pub**.

        :param pub: the publisher to use to send data.
        :param payload: a the data to be sent as bytes.
        :returns: 0 if the publication is successful.

        """
        self.zlib.zn_stream_compact_data(pub, data, len(data))

    def stream_data(self, pub, data, encoding=0, kind=ZN_PUT):
        """
        Send data in a *stream_data* message for the resource published by
        publisher **pub**.

        :param pub: the publisher to use to send data.
        :param data: the data to be sent as bytes.
        :param encoding: a metadata information associated with the published
            data that represents the encoding of the published data.
        :param kind: a metadata information associated with the published
            data that represents the kind of publication.
        :returns: 0 if the publication is successful.

        """
        self.zlib.zn_stream_data_wo(pub, data, len(data), encoding, kind)

    def write_data(self, resource, data, encoding=0, kind=ZN_PUT):
        """
        Send data in a *write_data* message for the resource **resource**.

        :param resource: the resource name of the data to be sent.
        :param data: the data to be sent as bytes.
        :param encoding: a metadata information associated with the published
            data that represents the encoding of the published data.
        :param kind: a metadata information associated with the published
            data that represents the kind of publication.
        :returns: 0 if the publication is successful.
        """
        self.zlib.zn_write_data_wo(self.session,
                                   resource.encode(),
                                   data,
                                   len(data),
                                   encoding,
                                   kind)

    def pull(self, sub):
        """
        Pull data for the `ZN_PULL_MODE` or `ZN_PERIODIC_PULL_MODE`
        subscription **sub**.
        The pulled data will be provided by calling the **data_handler**
        function provided to the **declare_subscriber** function.

        :param sub: the subscribtion to pull from.
        :returns: 0 if pull is successful.
        """
        self.zlib.zn_pull(sub)

    def query(self, resource, predicate, callback,
              dest_storages=QueryDest(QueryDest.ZN_BEST_MATCH),
              dest_evals=QueryDest(QueryDest.ZN_BEST_MATCH)):
        """
        Query data matching resource name **resource**.

        :param resource: the resource to query.
        :param predicate: a string that will be  propagated to the storages
            and evals that should provide the queried data. It may allow them
            to filter, transform and/or compute the queried data.
        :param reply_handler: the callback function that will be called on
            reception of the replies of the query.
        :param dest_storages: indicates which matching storages should be
            destination of the query (see :class:`QueryDest`).
        :param dest_evals: indicates which matching evals should be
            destination of the query (see :class:`QueryDest`).
        :returns: 0 if the query is sent successfully.
        """
        global replyCallbackMap
        h = hash(callback)
        k = POINTER(c_int64)(c_int64(h))
        replyCallbackMap[h] = (k, callback)
        r = self.zlib.zn_query_wo(self.session,
                                  resource.encode(),
                                  predicate.encode(),
                                  zn_reply_trampoline_callback,
                                  k,
                                  dest_storages.zn_qd,
                                  dest_evals.zn_qd)
        if r != 0:
            del replyCallbackMap[h]
            raise ZException('Unable to create query', r)

    def undeclare_publisher(self, pub):
        """
        Undeclare the publication **pub**.

        :param pub: the publication to undeclare.
        :returns: 0 when successful.

        """
        self.zlib.zn_undeclare_publisher(pub)

    def undeclare_subscriber(self, sub):
        """
        Undeclare the subscrbtion **sub**.

        :param sub: the subscription to undeclare.
        :returns: 0 when successful.

        """
        self.zlib.zn_undeclare_subscriber(sub)

    def undeclare_storage(self, sto):
        """
        Undeclare the storage **sto**.

        :param sto: the storage to undeclare.
        :returns: 0 when successful.

        """
        self.zlib.zn_undeclare_storage(sto)

    def undeclare_eval(self, eval):
        """
        Undeclare the eval **eval**.

        :param eval: the eval to undeclare.
        :returns: 0 when successful.

        """
        self.zlib.zn_undeclare_eval(eval)

    def close(self):
        """
            Close the zenoh session.
        """
        self.zlib.zn_close(self.session)
        self.zlib.zn_stop_recv_loop(self.session)
