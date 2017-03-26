## http long polling

- 单个请求的超时时间尽量长一些
- 每个请求在接收回调里面发起下一个请求
- 服务端对于该请求用一个全局队列缓存数据

````bash
[lavenderuni@~]# cd longpolling
[lavenderuni@~]# python run.py

浏览器打开http://localhost:8888/static/demo.html
````

## http comet streaming

- 请求发送完成后，保持连接keep-alive
- 保持连接过程中，服务端可以不断推送数据到客户端
- 结束标志是发送了多少数据，是否到了业务边界
- web端目前靠xmlhttprequest的readyState=3来接收数据

````bash
[lavenderuni@~]# cd streaming
[lavenderuni@~]# python run.py

浏览器打开http://localhost:8888/static/demo.html
````

## http1.1 pipeline

通过同一个tcp数据包发送多个http请求，http管线化向网络上发送更少的tcp数据包，以便减轻网络负载

````bash

````

## http2

- 一个HTTP/2连接可保持多个stream
- 流可被客户端或服务器单独或共享创建和使用
- 流可被任一端关闭
- 在流内发送和接收数据都要按照顺序
- 流的标识符自然数表示，1~2^31-1区间，有创建流的终端分配
- 流与流之间逻辑上是并行、独立存在
- 流有优先级（priority），优先级高的先处理
- 流的并发数最好多于100
- 流量控制阀协调网络带宽资源利用，由接收端提出发送端遵守其规则
- 流具有完整的生命周期，从创建到最终关闭，经历不同阶段


````bash

````

## websocket

双工协议


````bash

````

## csrf是跨域伪造请求攻击


````bash
[lavenderuni@~]# python run1.py # 开启http://127.0.0.1:8888/
[lavenderuni@~]# python run2.py # 开启http://127.0.0.1:9999/

1. 浏览器打开http://localhost:8888/static/demo1.html
2. 点击跳转到http://127.0.0.1:9999/static/demo2.html
3. 观察demo2.html页面的模拟ajax get请求传到localhost:8888的cookie
4. 观察demo2.html页面的img src请求传到localhost:8888的cookie
5. 点击evil way发起模拟ajax post请求传到localhost:8888的cookie
6. 点击iframe里面的evil way发起form请求传到localhost:8888的cookie
````