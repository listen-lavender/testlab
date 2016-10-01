#!/usr/bin/env python
# coding=utf-8
import unittest
from asynctask.unit.project.test import add

class TestAdd(unittest.TestCase):

    def setUp(self):
        pass

    def test_add(self):
        self.assertEqual(add(1, 2), 3)


if __name__ == '__main__':
    pass
