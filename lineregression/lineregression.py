# -*- coding: utf-8 -*-
"""
Created on Wed Sep 19 18:45:05 2016

@author: kanungo
"""

# results in warning if kept here, so moved up
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from matplotlib import cm
from mpl_toolkits.mplot3d import *
from sklearn.cross_validation import train_test_split
from sklearn.linear_model import LinearRegression
from sklearn.metrics import r2_score

def fileInput(path):
    data = np.genfromtxt(path, delimiter=',')
    return data


def regress(data, independs=[], depends='', intercept=''):

    if len(data) < 2:
        raise Exception("It is not a matrix, but an array.")
    length = len(data[0])
    for index, one in enumerate(data[1:]):
        if not len(one) == length:
            raise Exception("It is not a right matrix, length of array line %d is wrong." % (index + 2))
            break

    if not independs or not len(independs) == length - 1:
        raise Exception("Please give independent variable names for column 1 to n.")

    if not depends:
        raise Exception("Please give dependent variable names for column 0.")

    Y = pd.Series(data[:, 0]) # depends
    X = pd.DataFrame(data[:, 1:], columns=independs)
    X_train, _test, Y_train, _test = train_test_split(X, Y, random_state=1)
    linreg = LinearRegression()
    linreg.fit(X_train, Y_train)
    coef = list(linreg.coef_)

    Y_pred = []
    for i in data[:, 1:]:
        line_total = linreg.intercept_
        for index, j in enumerate(i):
            line_total += j * coef[index]
        Y_pred.append(line_total)

    r2 = r2_score(Y, Y_pred, multioutput='variance_weighted') 

    coef = [1, linreg.intercept_] + coef + [r2, ]
    ins = [depends, intercept] + independs + ['R­-Square', ]
    return dict(zip(ins, coef))


def myPlot(myData, b):
    """
    Input = numpy array with three columns
            Column 1 is the dependent variable
            Columns 2 and 3 are the independent variables
            and
            a column vector with the b coefficients
    Returns = Noting
    Output = 3D plot of the actual data and 
             the surface plot of the linear model
    """        
    # http://stackoverflow.com/questions/15229896/matplotlib-3d-plots-combining-scatter-plot-with-surface-plot

    fig = plt.figure()
    ax = fig.gca(projection='3d')               # to work in 3d
    plt.hold(True)
    
    x_max = max(myData[:,1])    
    y_max = max(myData[:,2])   
    
    b0 = float(b[0])
    b1 = float(b[1])
    b2 = float(b[2])   
    
    x_surf=np.linspace(0, x_max, 100)                # generate a mesh
    y_surf=np.linspace(0, y_max, 100)
    x_surf, y_surf = np.meshgrid(x_surf, y_surf)
    z_surf = b0 + b1*x_surf +b2*y_surf         # ex. function, which depends on x and y
    ax.plot_surface(x_surf, y_surf, z_surf, cmap=cm.hot, alpha=0.2);    # plot a 3d surface plot
    
    x=myData[:,1]
    y=myData[:,2]
    z=myData[:,0]
    ax.scatter(x, y, z);                        # plot a 3d scatter plot
    
    ax.set_xlabel('x1')
    ax.set_ylabel('y2')
    ax.set_zlabel('y')

    plt.show()


def display_coef(independs, vector, formular=False):
    txt = []
    for key in independs:
        val = vector.get(key, '')
        if val is None:
            val = ''
        elif type(val) == float or type(val) == np.float64:
            val = str(round(val, 2))
        else:
            val = str(val)
        if formular:
            txt.append('%s%s' % (val, key))
        else:
            txt.append('%s = %s' % (key, val))
    if formular:
        return ' + '.join(txt)
    else:
        return '\n'.join(txt)


def format(data, vector, brief=False, verbose=False, independs=[], depends='', intercept=''):
    fill = 10
    if verbose:
        txt = 'The equation is:\n'
        txt += "y = %s + %s\n" % (str(round(vector.get(intercept, 0), 2)), display_coef(independs, vector, formular=True))
        txt += "The R­square value is:  %s\n" % str(round(vector.get('R­-Square', 0), 2))
        txt += "The formatted input data is shown below: %s %s\n" % (depends, ' '.join(independs))
        txt += "=========================\n"
        cols = [depends, ] + independs
        txt += "%s\n" % "".join([one.ljust(fill, ' ') for one in cols])
        for one in data:
            line_txt = ""
            for index, _ in enumerate(cols):
                line_txt += str(one[index]).ljust(fill, ' ')
            txt += "%s\n" % line_txt
    elif brief:
        txt = display_coef([intercept, ] + independs, vector)
    else:
        txt = display_coef([intercept, ] + independs + ['R­-Square', ], vector)
    return txt


def test():
    data = pd.read_csv('sampledata.csv', sep=',')
    features = ['x1', 'x2']

    # use the list to select a subset of the original DataFrame
    X = data[features]
    Y = data['y']
    X_train, X_test, y_train, y_test = train_test_split(X, Y, random_state=1)
    linreg = LinearRegression()
    linreg.fit(X_train, y_train)
    return zip(features, linreg.coef_)


if __name__ == "__main__":
    from docopt import docopt
    helpdoc = """Multiple Line Regression, default output contains the coefficients of the regression equation and R2 value.
        Usage:
          lineregression.py <inpath>
          lineregression.py <inpath> (-b | --brief)
          lineregression.py <inpath> (-v | --verbose)
          lineregression.py <inpath> (-p | --plot)
          lineregression.py <inpath> (-o | --output) <outpath>

        Options:
          -h --help     helper
          -b --brief    the brief output contains the coefficients of the regression equation
          -v --verbose  the verbose  output contains
                            * the description of the equation
                            * the R2    value
                            * actual data values with column titles
          -p --plot     the plot the regression – actual values as an XY­plot and the estimated values as a hyperplane
          -o --output   the output to send the output ( verbose  only) to a file with the name of the file provided by the user.
    """

    params = docopt(helpdoc)
    inpath = params.get('<inpath>')
    brief = params.get('-b') or params.get('--brief')
    verbose = params.get('-v') or params.get('--verbose')
    plot = params.get('-p') or params.get('--plot')
    output = params.get('-o') or params.get('--output')
    outpath = params.get('<outpath>')

    data = fileInput(inpath)
    vector = regress(data, independs=['x1', 'x2'], depends='y', intercept='x0')

    if plot:
        cols = ['x0', 'x1', 'x2']
        myPlot(data, [vector.get(one, 0) for one in cols])
    elif output:
        txt = format(data, vector, brief=False, verbose=True, independs=['x1', 'x2'], depends='y', intercept='x0')
        fi = open(outpath, 'w')
        fi.write(txt)
        fi.close()
    elif brief:
        print format(data, vector, brief=True, verbose=False, independs=['x1', 'x2'], depends='y', intercept='x0')
    elif verbose:
        print format(data, vector, brief=False, verbose=True, independs=['x1', 'x2'], depends='y', intercept='x0')
    else:
        print format(data, vector, brief=False, verbose=False, independs=['x1', 'x2'], depends='y', intercept='x0')
    
    