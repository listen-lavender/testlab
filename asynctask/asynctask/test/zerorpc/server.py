#!/usr/bin/env python
# coding=utf8

import asynctask.zerorpc.server as zserver
import pymongo, datetime

mc = pymongo.MongoClient('localhost')
resource = mc['test']['resource']

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

server = zserver.Server()
server.register(add)
server.register(sub)
server.register(init)
server.register(insert)
server.register(remove)
server.start(5)

