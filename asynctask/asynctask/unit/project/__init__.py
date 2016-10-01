#!/usr/bin/env python
# coding=utf8

from ...lib import Task
from ... import voidrun
from ...celery import app
from ...zerorpc.client import Client as ZerorpcClient
from .. import RPC_CLIENT

proj = __name__.split('.')[-1]

class BaseTask(Task):
    abstract = True
    _project_proxy = ZerorpcClient(host=RPC_CLIENT['host'], port=RPC_CLIENT['port'])

    def after_return(self, status, retval, task_id, args, kwargs, einfo):
        """
        Clean up resource after the task is finished.

        """
        pass


@app.task(bind=True)
def run(self, func_name, *args, **kwargs):
    project_proxy = ZerorpcClient(host=RPC_CLIENT['host'], port=RPC_CLIENT['port'])
    return (getattr(project_proxy, '%s.%s' % (proj, func_name)) or voidrun)(*args, **kwargs)
    del project_proxy