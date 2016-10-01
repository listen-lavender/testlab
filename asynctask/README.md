# asynctask
[![Build Status](https://api.travis-ci.org/listen-lavender/asynctask.svg?branch=master)](https://api.travis-ci.org/listen-lavender/asynctask)
Best practice of celery.

## celery app connection setting
asynctask/setting.py

## task connection setting
asynctask/db.py

# Getting started

No example now.

## Installation

To install asynctask, simply:

````bash

    $ cd asynctask
    $ python setup.py install
````

````bash

    $ celery -A asynctask worker --loglevel=info
````

````python

    >>> from asynctask.unit.project.test import add, insert
    >>> add()
    >>> insert()
    >>> add.apply_async()
    >>> insert.apply_async()
````
or
````python

    >>> from asynctask.celery import send_task
    >>> send_task('asynctask.unit.project.test.add')
````

````mongo

    > use test
    > db.resource.find()
````

## Discussion and support

Report bugs on the *GitHub issue tracker <https://github.com/listen-lavender/asynctask/issues*. 
