#!/usr/bin/env python
# coding:utf-8

import math, random

class MoneyPackage(object):
    def __init__(self, remainSize, remainMoney):
        self.remainSize = remainSize
        self.remainMoney = remainMoney

def getRandomMoney(_leftMoneyPackage):
    if _leftMoneyPackage.remainSize == 1:
        _leftMoneyPackage.remainSize = _leftMoneyPackage.remainSize - 1
        money = round(_leftMoneyPackage.remainMoney * 100) / 100
        return int(money)

    min_limit = 1
    max_limit = _leftMoneyPackage.remainMoney / _leftMoneyPackage.remainSize * 2
    money = random.random() * max_limit
    money = 1 if money < min_limit else money
    money = int(math.floor(money * 100) / 100)
    _leftMoneyPackage.remainSize = _leftMoneyPackage.remainSize -1
    _leftMoneyPackage.remainMoney = _leftMoneyPackage.remainMoney - money
    return money

if __name__ == '__main__':
    total_money = 100
    total_package = 10
    moneypackage = MoneyPackage(total_package, total_money)
    data = []
    for one in range(total_package):
        money = getRandomMoney(moneypackage)
        data.append(money)
        print "get package %d" % (one + 1), money
    print 'total', sum(data)
