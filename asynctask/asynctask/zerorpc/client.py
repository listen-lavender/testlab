#!/usr/bin/env python
# coding=utf-8

import zmq
import logging
from . import Error, serializer, deserializer
log = logging.getLogger("zerorpc.client")


class _Method(object):
    def __init__(self, send, name):
        self.__send = send
        self.__name = name
    def __getattr__(self, name):
        return _Method(self.__send, "%s.%s" % (self.__name, name))
    def __call__(self, *args, **kwargs):
        result = self.__send(self.__name, *args, **kwargs)[0]
        if result["success"]:
            return result["result"]
        else:
            raise Error(result["error_type"], result["message"], result["traceback"])


class Client(object):
    def __init__(self, host='localhost', port=5570):
        self.context = zmq.Context()
        self.socket = self.context.socket(zmq.REQ)
        # self.socket = self.context.socket(zmq.REQ)
        self.socket.connect('tcp://%s:%s' % (host, str(port)))

    def __getattr__(self, name):
        return _Method(self._call, name)

    def _call(self, mark, *args, **kwargs):
        packet = serializer([[mark, args, kwargs],])
        return self._communicate(packet)

    def _communicate(self, packet):
        self.socket.send(packet)
        data = self.socket.recv()
        result = deserializer(data)
        return result

    def close(self):
        self.socket.close()
        self.context.term()

if __name__ == "__main__":
    pass
