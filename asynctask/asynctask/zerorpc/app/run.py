#!/usr/bin/env python
# coding=utf8
import os
from daemon import Daemon

path = os.path.abspath('.')

class PeriodMonitor(Daemon):

    def add_server(self, server):
        self.server = server

    def _run(self):
        self.server.start()

def main():
    rpc = PeriodMonitor(os.path.join(path, 'log', 'rpc.pid'), stdout=os.path.join(
        path, 'log', 'rpc.out'), stderr=os.path.join(path, 'log', 'rpc.err'))
    if os.path.exists(os.path.join(path, 'log', 'rpc.pid')):
        print "PeriodMonitor stop successfully."
        rpc.stop()
    else:
        from register import server
        rpc.add_server(server)
        print "PeriodMonitor start successfully."
        rpc.start()

if __name__ == '__main__':
    main()

