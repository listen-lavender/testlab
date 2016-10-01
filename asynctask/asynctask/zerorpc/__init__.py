#!/usr/bin/env python
# coding=utf-8

import json
import functools
from datetime import datetime, date

class DatetimeEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, datetime):
            return obj.strftime('%Y-%m-%d %H:%M:%S')
        elif isinstance(obj, date):
            return obj.strftime('%Y-%m-%d')
        else:
            return json.JSONEncoder.default(self, obj)

serializer = functools.partial(json.dumps, cls=DatetimeEncoder)
deserializer = json.loads

class Error(Exception):
    def __init__(self, error_type, message='', tb=None):
        self.error_type = error_type
        self.message = message
        self.traceback = tb
    def __repr__(self):
        return "%s('%s')" % (self.error_type, self.message)
    __str__ = __repr__
