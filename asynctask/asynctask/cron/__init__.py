#!/usr/bin/env python
# coding=utf8
from datetime import timedelta

CELERYBEAT_SCHEDULE={
    "add": {
        "task": "asynctask.cron.project.test.add",
        "schedule": timedelta(hours=7), # 每3小时的12分钟执行
        # "schedule": timedelta(seconds=2),
        # "schedule": crontab(minute="*/1"),
        # "args": (16, 16),
    }
}