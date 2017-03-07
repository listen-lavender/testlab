#!/usr/bin/env python
# coding=utf8

from airflow import DAG
from airflow.operators import BashOperator

from datetime import datetime, timedelta

a_minute_ago = datetime.combine(datetime.today() - timedelta(minutes=1),
                                datetime.min.time())

default_args = {
  'owner': 'airflow',         
  'depends_on_past': True, 
  'start_date': a_minute_ago,
  'email': ['*******@163.com'],
  'email_on_failure': True, 
  'email_on_retry': True, 
  'retries': 5, 
  'retry_delay': timedelta(hours=30), 
  #'queue': 'bash_queue',
  #'pool': 'backfill', 
  #'priority_weight': 10, 
  #'end_date': datetime(2016, 5, 29, 11, 30), 
}

dag = DAG('ct2', default_args=default_args,
  schedule_interval="@once")

t1 = BashOperator(
  task_id='run1', 
  bash_command='(cd ~; echo "abc" > abc;) ', 
  dag=dag)

t2 = BashOperator(
  task_id='run2', 
  bash_command='(cd ~; echo "de" > de;) ', 
  dag=dag)

t2.set_upstream(t1)
