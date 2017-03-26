#!/usr/bin/env python
# coding=utf8

import time, random
import os
from tornado import web, httpserver, ioloop, gen
from tornado.options import options, define

define(name='port', default=9999, type=int)
static_path = os.path.dirname(os.path.abspath(__file__))

class PollingHandler(web.RequestHandler):

    def get(self):
        pid = self.get_cookie("pid")
        print 'pid=', pid, "from: ", self.request.headers['referer']
        self.write("")
        self.finish()

if __name__ == "__main__":
    options.parse_command_line()
    app = web.Application([
        (r"/polling", PollingHandler), 
        (r'/static/(.*)', web.StaticFileHandler, {'path': static_path})])

    http_server = httpserver.HTTPServer(app)
    http_server.listen(options.port)
    print "[start server on port:%s]..." % options.port
    ioloop.IOLoop.instance().start()
