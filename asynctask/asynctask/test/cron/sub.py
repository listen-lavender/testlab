#!/usr/bin/env python
# coding=utf-8
import unittest
from asynctask.cron.project.test import sub

class TestSub(unittest.TestCase):

    def setUp(self):
        pass

    def test_sub(self):
        self.assertEqual(sub(), 3)


if __name__ == '__main__':
    pass
