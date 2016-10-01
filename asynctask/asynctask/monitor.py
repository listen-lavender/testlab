#!/usr/bin/env python
# coding=utf-8

import os
import time
from watchdog.observers import Observer
from watchdog.tricks import Trick
from setting import PIDFILE, LOGFILE, CELERY_HOST

if not PIDFILE.startswith('/'):
    PIDFILE = os.path.abspath(os.path.join('..', PIDFILE))

CURR = os.path.dirname(os.path.abspath(__file__))
TIMESPAN = 5

class FilterHandler(Trick):
    last = time.time() - 10 * TIMESPAN

    def __init__(self, patterns=None, ignore_patterns=None, ignore_directories=False, case_sensitive=False):
        super(FilterHandler, self).__init__(patterns, ignore_patterns, ignore_directories, case_sensitive)
        self.continous = False

    def on_any_event(self, event):
        path = event.src_path.replace(CURR, '').strip('/')
        now = time.time()
        induring = (now - self.last) < TIMESPAN
        print 'on_any_event', path, induring
        if induring:
            self.continous = False
        elif event.is_directory:
            self.continous = False
        elif not path.split('.')[-1] in ('py', 'json', 'yaml', 'conf', 'cfg'):
            self.continous = False
        else:
            if path in ('setting.py', 'celery.py', 'db.py') or path.startswith('cron/') or path.startswith('unit/') or path.startswith('lib/'):
                self.last = now
                self.continous = True
            else:
                self.continous = False

    def on_modified(self, event):
        print 'on_modified', self.continous
        if self.continous:
            os.system('cd .. && celery multi stop 1 --pidfile=%s' % PIDFILE)
            os.system('cd .. && celery multi start 1 -A asynctask -l info -c4 --pidfile=%s --logfile=%s --hostname=%s' % (PIDFILE, LOGFILE, CELERY_HOST))

    def on_created(self, event):
        print 'on_created', self.continous
        if self.continous:
            os.system('cd .. && celery multi stop 1 --pidfile=%s' % PIDFILE)
            os.system('cd .. && celery multi start 1 -A asynctask -l info -c4 --pidfile=%s --logfile=%s --hostname=%s' % (PIDFILE, LOGFILE, CELERY_HOST))

    def on_deleted(self, event):
        print 'on_deleted', self.continous
        if self.continous:
            os.system('cd .. && celery multi stop 1 --pidfile=%s' % PIDFILE)
            os.system('cd .. && celery multi start 1 -A asynctask -l info -c4 --pidfile=%s --logfile=%s --hostname=%s' % (PIDFILE, LOGFILE, CELERY_HOST))

    def on_moved(self, event):
        print 'on_moved', self.continous
        if self.continous:
            os.system('cd .. && celery multi stop 1 --pidfile=%s' % PIDFILE)
            os.system('cd .. && celery multi start 1 -A asynctask -l info -c4 --pidfile=%s --logfile=%s --hostname=%s' % (PIDFILE, LOGFILE, CELERY_HOST))


if __name__ == '__main__':
    
    path = CURR
    
    event_handler = FilterHandler()
    # event_handler = FilterHandler(patterns=['unit/*.py', 'cron/*.py'], ignore_patterns=['test.py'])
    observer = Observer()
    observer.schedule(event_handler, path, recursive=True)
    observer.start()
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
    observer.join()

