#!/usr/bin/env python
# coding=utf-8
import unittest

TEST_MODULES = [
'unit',
'cron'
]

def main():
    testSuite = unittest.TestSuite()
    for module in TEST_MODULES:
        suite = unittest.TestLoader().discover(module, pattern='[!__init__]*.py')
        testSuite.addTest(suite)

    return testSuite

if __name__ == '__main__':
    unittest.TextTestRunner(verbosity=2).run(main())
