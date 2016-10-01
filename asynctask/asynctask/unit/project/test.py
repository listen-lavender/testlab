#!/usr/bin/env python
# coding=utf-8

from ...celery import app

@app.task
def add(a, b):
    return a + b


@app.task
def tsum(numbers):
    return sum(numbers)


@app.task
def trange(limit):
    return range(limit)