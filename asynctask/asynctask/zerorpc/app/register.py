#!/usr/bin/env python
# coding=utf-8
import os, sys
from setting import RPC_SERVER
sys.path.append(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))))

from asynctask.zerorpc.server import Server
server = Server(host=RPC_SERVER['host'], port=RPC_SERVER['port'], backend_domain=RPC_SERVER['backend_domain'], worknum=RPC_SERVER['worknum'])

def init():
    fi = open('test.log', 'w')
    fi.write('ok')
    fi.close()

def add(a, b):
    return a + b

def sub(a, b):
    return a - b

def insert():
    resource.insert({'a':'adfhdkfhkd', 'b':1, 'atime':datetime.datetime.now()})

def remove():
    resource.remove({})

server.register('project', add)
server.register('emoji', insert)
server.register('emoji', remove)