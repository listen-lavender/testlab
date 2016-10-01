#!/usr/bin/env python
# coding=utf-8

import os
import pymongo

current = os.path.abspath(__file__)

if current.endswith('pyc') and '/lib' in current:
    resource = None
else:
    mc = pymongo.MongoClient('localhost')
    emoji = mc['test']
    resource = emoji['resource']

if __name__ == "__main__":
    pass