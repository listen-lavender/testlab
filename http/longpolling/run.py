#!/usr/bin/env python
# coding=utf8

import time, random
import os
from tornado import web, httpserver, ioloop, gen
from tornado.options import options, define

define(name='port', default=8888, type=int)
static_path = os.path.dirname(os.path.abspath(__file__))

class LongPollingHandler(web.RequestHandler):

    abuffer = []

    def get(self):
        time.sleep(1)
        data = LongPollingHandler.abuffer
        LongPollingHandler.abuffer = []
        for one in data:
            self.write(" Server says: %d " % one)
        self.write("\n")
        self.finish()

def add_data():
    for k in range(random.randint(1, 3)):
        LongPollingHandler.abuffer.append(random.randint(1, 100))
    ioloop.IOLoop.instance().add_timeout(
        time.time() + 2,
        lambda: add_data()
    )

add_data()

if __name__ == '__main__':

    options.parse_command_line()
    app = web.Application([
        (r"/longpolling", LongPollingHandler), 
        (r'/static/(.*)', web.StaticFileHandler, {'path': static_path})])

    http_server = httpserver.HTTPServer(app)
    http_server.listen(options.port)
    print "[start server on port:%s]..." % options.port
    ioloop.IOLoop.instance().start()
