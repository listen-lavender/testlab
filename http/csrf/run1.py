#!/usr/bin/env python
# coding=utf8

import time, random
import os
from tornado import web, httpserver, ioloop, gen
from tornado.options import options, define

define(name='port', default=8888, type=int)
static_path = os.path.dirname(os.path.abspath(__file__))

class PollingHandler(web.RequestHandler):
    def set_default_headers(self):
        self.set_header('Access-Control-Allow-Origin', 'http://127.0.0.1:9999')
        self.set_header('Access-Control-Allow-Methods', 'POST, GET, OPTIONS')
        self.set_header('Access-Control-Allow-Credentials', 'true')
        self.set_header('Access-Control-Allow-Headers', '*')

    def get(self):
        pid = self.get_cookie("pid")
        print 'pid=', pid, "from: ", self.request.headers['referer']
        if not pid:
            self.set_cookie("pid", "1234567890", expires=None, expires_days=None)
        self.write("")
        self.finish()

    def post(self):
        pid = self.get_cookie("pid")
        print 'pid=', pid, "from: ", self.request.headers['referer']
        if not pid:
            self.set_cookie("pid", "1234567890", expires=None, expires_days=None)
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
