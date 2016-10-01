#!/usr/bin/env python
# coding=utf8

"""
    安装包工具
"""

import os
from setuptools import setup, find_packages

here = os.path.abspath(os.path.dirname(__file__))
README = open(os.path.join(here, 'README.rst')).read()
CHANGES = open(os.path.join(here, 'CHANGES.rst')).read()

install_requires = [
    'celery',
    'flower',
    'pyzmq>=13.1.0',
    ]

asynctask = __import__('asynctask')
setup(name='asynctask',
version=asynctask.__version__,
description='wecatch asynctask',
author='haokuan',
author_email='jingdaohao@gmail.com',
url='https://github.com/listen-lavender/asynctask',
keywords='wecatch > ',
packages=find_packages(exclude=['asynctask.cron.*', 'asynctask.unit.*']),
install_requires=install_requires
)

