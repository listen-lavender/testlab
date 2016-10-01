#!/usr/bin/env python
# coding=utf8

from celery import Celery
from celery.task import Task
from celery.execute import send_task

from celery.worker import consumer
from celery.worker.consumer import Consumer

import inspect
from celery.exceptions import NotRegistered
from celery.app import registry
from celery.app.registry import TaskRegistry
from celery.app import base

class TransferConsumer(Consumer):

    def create_task_handler(self):
        strategies = self.strategies
        on_unknown_message = self.on_unknown_message
        on_unknown_task = self.on_unknown_task
        on_invalid_task = self.on_invalid_task
        callbacks = self.on_task_message

        def on_task_received(body, message):
            try:
                name = body['task']
            except (KeyError, TypeError):
                return on_unknown_message(body, message)

            try:
                if '.' in name and name.split('.')[-2] == 'run':
                    name = name.split('.')
                    args = list(message.payload['args'])
                    args.insert(0, name[-1])
                    message.payload['args'] = tuple(args)
                    body['args'] = tuple(args)
                    name = '.'.join(name[:-1])
                    # message.payload['task'] = name
                    # body['task'] = name

                strategies[name](message, body,
                                 message.ack_log_error,
                                 message.reject_log_error,
                                 callbacks)
            except KeyError as exc:
                on_unknown_task(body, message, exc)
            except InvalidTaskError as exc:
                on_invalid_task(body, message, exc)

        return on_task_received

consumer.Consumer = TransferConsumer


class DefaultRegistry(TaskRegistry):

    def __missing__(self, key):
        if '.' in key and key.split('.')[-2] == 'run':
            key = '.'.join(key.split('.')[:-1])
            if key in self:
                return self[key]
        raise self.NotRegistered(key)


registry.TaskRegistry = DefaultRegistry
base.TaskRegistry = DefaultRegistry
