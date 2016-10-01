#!/usr/bin/env python
# coding=utf-8

from datetime import timedelta

BACKEND = 'redis://localhost:6379/1'
BROKER = 'redis://localhost:6379/0'

CELERYD_POOL_RESTARTS = True
CELERY_TASK_RESULT_EXPIRES=3600
# BROKER_URL = 'amqp://'
# CELERY_RESULT_BACKEND = 'rpc://'

# CELERY_TASK_SERIALIZER = 'json'
# CELERY_RESULT_SERIALIZER = 'json'
# CELERY_ACCEPT_CONTENT=['json']
# CELERY_TIMEZONE = 'Europe/Oslo'
# CELERY_ENABLE_UTC = True
# CELERY_ROUTES={
#     "proj.tasks.add":{"queue":"hipri"},# 把add任务放入hipri队列
# }

PIDFILE = 'asynctask.pid'
LOGFILE = 'asynctask.log'
CELERY_HOST = 'localhost'