#!/usr/bin/env python
# coding=utf-8

from lib import Celery, send_task

from setting import BACKEND
from setting import BROKER
from setting import CELERYD_POOL_RESTARTS
from setting import CELERY_TASK_RESULT_EXPIRES
from unit import INCLUDE
from cron import CELERYBEAT_SCHEDULE
import db

from lib import registry
print registry.TaskRegistry

app = Celery("task", backend=BACKEND, broker=BROKER, include=INCLUDE)

app.conf.update(
    CELERYD_POOL_RESTARTS=CELERYD_POOL_RESTARTS,
    # CELERY_ROUTES={
    #     "proj.tasks.add":{"queue":"hipri"},# 把add任务放入hipri队列
    #     # 需要执行时指定队列 add.apply_async((2, 2), queue='hipri')
    #     },
    CELERY_TASK_RESULT_EXPIRES=3600,
    CELERYBEAT_SCHEDULE=CELERYBEAT_SCHEDULE
)


if __name__ == "__main__":
    app.start()