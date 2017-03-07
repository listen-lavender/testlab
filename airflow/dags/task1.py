#!/usr/bin/env python
# coding=utf8

from airflow import DAG
from airflow.operators import BashOperator, MySqlOperator

from datetime import datetime, timedelta

a_minute_ago = datetime.combine(datetime.today() -
timedelta(minutes=1), datetime.min.time())

default_args = {
  'owner': 'airflow',         

  #为了测试方便，起始时间一般为当前时间减去schedule_interval
  'start_date': datetime.now(), 
  'email': ['*******@163.com'],
  'email_on_failure': False, 
  'email_on_retry': False, 
  'depends_on_past': False, 
  'retries': 1, 
  'retry_delay': timedelta(minutes=5), 
  #'queue': 'bash_queue',
  #'pool': 'backfill', 
  #'priority_weight': 10, 
  #'end_date': datetime(2016, 5, 29, 11, 30), 
}


# DAG id 'ct1'必须在airflow中是unique的, 一般与文件名相同


# 多个用户时可加用户名做标记

dag = DAG('ct1', default_args=default_args,
  schedule_interval="@once")

t1 = BashOperator(
  task_id='print_date', 
  bash_command='date', 
  dag=dag)


#cmd = "/home/test/test.bash " 注意末尾的空格

t2 = BashOperator(
  task_id='echo', 
  bash_command='echo "test" ', 
  retries=3, 
  dag=dag)

templated_command = """
  {% for i in range(2) %}
      echo "{{ ds }}" 
      echo "{{ macros.ds_add(ds, 7) }}"
      echo "{{ params.my_param }}"
  {% endfor %}
"""
t3 = BashOperator(
  task_id='templated', 
  bash_command=templated_command, 
  params={'my_param': "Parameter I passed in"}, 
  dag=dag)

t2.set_upstream(t1)
t3.set_upstream(t1)
