````bash
$ python lineregression.py sampledata
x0 = 0.89
x1 = 0.28
x2 = -0.03
R­-Square = 0.19

$ python lineregression.py sampledata.txt ­-b
x0 = 0.89
x1 = 0.28
x2 = -0.03

$ python lineregression.py sampledata.txt ­-v
The equation is:
y = 0.89 + 0.28x1 + -0.03x2
The R­square value is:  0.19
The formatted input data is shown below: y x1 x2
=========================
y         x1        x2
1.0       2.0       3.0
2.0       3.0       4.0
1.0       3.0       4.0
2.0       3.0       2.0
2.0       3.0       6.0
1.0       3.0       5.0
2.0       5.0       4.0
1.0       3.0       7.0

$ python lineregression.py sampledata.txt ­-p
绘图

$ python lineregression.py sampledata.txt ­-o result.txt
将verbose生成result.txt
````