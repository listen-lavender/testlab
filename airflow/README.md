airflow是一个基于celery任务DAG管理工具，包括
- manage system
- worker
- scheduler

支持redis、rabbitmq、mongo、mysql、SQLite、postgresql作为queue，支持同时使用flower作为可视化任务的管理后台。
管理平台的数据落地支持mongo、mysql、SQLite、postgresql.

## 安装依赖
````bash
[lavenderuni@~]# pip install airflow
pip install "airflow[crypto, password]"
pip install "airflow[mysql]"
pip install "airflow[hive]"
pip install "airflow[celery]"
pip install "airflow[rabbitmq]"
````

## 初始化管理平台的数据结构到mysql

````sql
    create database airflow;
    grant all privileges on airflow.* to 'ct'@'localhost' identified by '152108';
    flush privileges;
    select * from mysql.user;
    airflow initdb # 任意目录下执行
    cd ~/airflow/
    vi airflow.cfg # 替换sql_alchemy_conn = mysql://ct:152108@localhost/airflow
    airflow initdb # 再次执行
````

## 初始化数据
````bash
    cd ~/airflow/
    vi airflow.cfg # 替换broker_url = redis://127.0.0.1:6379/15
    和 celery_result_backend = redis://127.0.0.1:6379/16
    
    airflow webserver --debug &
    airflow scheduler
````

## 设置执行任务的broker即queue
````bash
    cd ~/airflow/
    vi airflow.cfg # 替换broker_url = redis://127.0.0.1:6379/15
    和 celery_result_backend = redis://127.0.0.1:6379/16
    
    airflow webserver --debug &
    airflow scheduler
````

## 启动三大组件
````bash
    airflow webserver --debug & #启动管理平台，访问http://127.0.0.1:8080/
    airflow worker #启动工作者
    airflow scheduler #启动任务调度分发器
````


