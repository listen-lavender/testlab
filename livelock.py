# -*- coding: utf-8 -*-

import threading

# 共享资源
lock1 = threading.Lock()
lock2 = threading.Lock()

def worker1():
    while True:
        if not lock1.acquire(False):
            print("Worker1 未获取 lock1")
            continue
        print("Worker1 获取 lock1")
        if lock2.acquire(False):  # 非阻塞尝试获取lock2
            print("Worker1 获取 lock2，完成任务")
            lock2.release()
            lock1.release()
            break
        else:
            print("Worker1 释放 lock1，重试...")
            lock1.release()  # 释放lock1，退让

def worker2():
    while True:
        if not lock2.acquire(False):
            print("Worker12 未获取 lock2")
            continue
        print("Worker2 获取 lock2")
        if lock1.acquire(False):  # 非阻塞尝试获取lock1
            print("Worker2 获取 lock1，完成任务")
            lock1.release()
            lock2.release()
            break
        else:
            print("Worker2 释放 lock2，重试...")
            lock2.release()  # 释放lock2，退让

t1 = threading.Thread(target=worker1)
t2 = threading.Thread(target=worker2)
t1.start()
t2.start()
t1.join()
t2.join()