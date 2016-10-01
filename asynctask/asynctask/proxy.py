#!/usr/bin/env python
# coding=utf-8

import unit
import importlib

class Safeworker(object):

    def __init__(self, path):
        self.path = path
        self.module = importlib.import_module(self.path)

    def __getattr__(self, name):
        function = getattr(self.module, name, None)
        if function is None:
            def function(*args, **kwargs):
                pass
        if not hasattr(function, 'apply_async'):
            def apply_async(*args, **kwargs):
                pass
            function.apply_async = apply_async
        return function


if __name__ == "__main__":
    pass