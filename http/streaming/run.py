#!/usr/bin/env python
# coding=utf8

import time, os, random
from tornado import httpserver
from tornado import web, httpserver, ioloop, gen
from tornado.options import options, define

define(name='port', default=8888, type=int)
static_path = os.path.dirname(os.path.abspath(__file__))

class StreamingHandler(web.RequestHandler):

	def initialize(self):
		self.sent = []

	@web.asynchronous
	def post(self):
		self.continue_send()

	def random_data(self):
		num = random.randint(1, 100)
		return " Server says: %d " % num

	def continue_send(self):
		data = self.random_data()
		self.sent.append(data)

		if len(self.sent) > 10 or self.request.connection.stream.closed():
			self.finish()
			return

		self.write(data)
		self.flush()

		tornado.ioloop.IOLoop.instance().add_timeout(
			time.time() + 3,
			lambda: self.continue_send()
		)

if __name__ == "__main__":
	
	options.parse_command_line()
	app = web.Application([
			("/streaming", StreamingHandler),
			("/static/(.*)", web.StaticFileHandler, {'path': static_path})])

	http_server = httpserver.HTTPServer(app)
	http_server.listen(options.port)
	print "[start server on port:%s]..." % options.port
	ioloop.IOLoop.instance().start()
