#!/usr/bin/env python
# coding=utf8
import zmq
from asynctask.zerorpc.client import ZeroRpcClient
project_proxy = ZeroRpcClient()
print project_proxy.add(1, 2)
del project_proxy