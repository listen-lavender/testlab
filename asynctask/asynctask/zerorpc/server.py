#!/usr/bin/env python
# coding=utf-8

import sys
import traceback
import threading
import zmq
import logging
from . import Error, serializer, deserializer

log = logging.getLogger('zerorpc.server')


class Worker(threading.Thread):
    """ServerWorker"""
    def __init__(self, context, executor={}, host='inproc://workers'):
        threading.Thread.__init__(self)
        self.context = context
        self.executor = executor
        self.host = host

    def _execute(self, request):
        mark, args, kwargs = request
        proj, func_name = mark.split('.')
        try:
            func = self.executor[proj][func_name]
        except KeyError:
            return {"success" : False,
                    "error_type" : sys.exc_info()[0].__name__,
                    "message" : "Function '%s' not found" % mark,
                    "traceback" : ""}
        else:
            try:
                result = func(*args, **kwargs)
            except Exception, e:
                log.warning("call to %s failed:" % mark)
                return {"success" : False,
                        "error_type" : sys.exc_info()[0].__name__,
                        "message" : str(e),
                        "traceback" : traceback.format_exc()}
            else:
                return {"success" : True,
                        "result" : result}

    def run(self):
        worker = self.context.socket(zmq.REP)
        worker.connect(self.host)
        # socket = self.context.socket(zmq.REP)
        # socket.connect(self.url_worker)
        while True:
            try:
                request_packet = worker.recv()
                print 'request_packet', request_packet
            except zmq.core.error.ZMQError, e:
                if e.errno == zmq.ETERM:
                    break
            else:
                request_list = deserializer(request_packet)
                response_list = [self._execute(request) for request in request_list]
                response_packet = serializer(response_list)
                worker.send(response_packet)
        log.info("worker thread %s exiting" % threading.currentThread().ident)
        worker.close()


class Server(object):

    def __init__(self, host='*', port=5570, backend_domain='inproc://workers', worknum=1):
        self.backend_domain = backend_domain
        self.context = zmq.Context()
        self.frontend = self.context.socket(zmq.ROUTER)
        self.frontend.bind('tcp://%s:%s' % (host, str(port)))
        self.backend = self.context.socket(zmq.DEALER)
        self.backend.bind(self.backend_domain)
        self.executor = {}
        self.worknum = worknum

    def register(self, proj, func, func_name=None):
        func_name = func_name or func.__name__
        if proj in self.executor:
            self.executor[proj][func_name] = func
        else:
            self.executor[proj] = {func_name:func}

    def start(self):
        workers = []
        for k in range(self.worknum):
            worker = Worker(self.context, self.executor, self.backend_domain)
            worker.start()
            workers.append(worker)

        zmq.proxy(self.frontend, self.backend)
        self.frontend.close()
        self.backend.close()
        self.context.term()

    def shutdown(self):
        pass


if __name__ == "__main__":
    pass
