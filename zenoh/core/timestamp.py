
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


from ctypes import *
import datetime
import binascii


# Timestamp
class Timestamp(Structure):
    '''
    Data structure representing a unique timestamp.

    time
        The time as a 64-bit long, where:
        - The higher 32-bit represent the number of seconds
            since midnight, January 1, 1970 UTC
        - The lower 32-bit represent a fraction of 1 second.

    clock_id
        The unique identifier of the clock that generated this timestamp.
    '''
    _fields_ = [('clock_id', c_uint8 * 16),
                ('time', c_size_t)]

    def __hash__(self):
        return hash((self.time, self.clock_id[0], self.clock_id[1],
                     self.clock_id[2], self.clock_id[3], self.clock_id[4],
                     self.clock_id[5], self.clock_id[6], self.clock_id[7],
                     self.clock_id[8], self.clock_id[9], self.clock_id[10],
                     self.clock_id[11], self.clock_id[12], self.clock_id[13],
                     self.clock_id[14], self.clock_id[15]))

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            if self.time != other.time:
                return False
            for i in range(0, 15):
                if self.clock_id[i] != other.clock_id[i]:
                    return False
            return True

    def __lt__(self, other):
        if self.time < other.time:
            return True
        if self.time > other.time:
            return False
        for i in range(0, 15):
            if self.clock_id[i] < other.clock_id[i]:
                return True
            if self.clock_id[i] > other.clock_id[i]:
                return False
        return False

    def __str__(self):
        sec = self.time >> 32
        time = datetime.datetime.utcfromtimestamp(float(sec))
        frac = self.time & 0xffffffff
        ns = int((frac * 1000000000) / 0x100000000)
        id = binascii.hexlify(self.clock_id).decode("ascii")
        return "{}.{:09d}Z/{}".format(time.isoformat(), ns, id)

    def floattime(self):
        """
        Return the time as a float
        (i.e. number of seconds since Epoch:  January 1, 1970, 00:00:00 (UTC))

        Warning: the time might be rounded, depending the float precision on
        your host.

        :returns: the time as a float
        """
        sec = self.time >> 32
        frac = self.time & 0xffffffff
        ns = float(frac) / 0x100000000
        return sec + ns

    def datetime(self, tzinfo=None):
        """
        Return the time as a :py:class:`datetime.datetime`

        Warning: the time is rounded to milliseconds as datetime precision
        is millisecond.

        :param tzinfo: optional argument. If ``None`` or not specified,
            the timestamp is converted to the platform’s local date and time,
            and the returned datetime object is naive.
            If not ``None``, it must be an instance of a
            :py:class:`datetime.tzinfo` subclass, and the timestamp is
            converted to tz’s time zone.
        :returns: the time as a :py:class:`datetime.datetime`
        """
        return datetime.datetime.fromtimestamp(self.floattime(), tzinfo)

    def clockid(self):
        """
        Return the unique identifier of the clock that created this Timestamp.

        :returns: the clock_id as an array of 16 c_uint8.
        """
        return self.clock_id
