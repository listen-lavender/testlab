function MockPromise(cont) {
    ab = {}
    ab.then = null;
    ab.callbackd = null;
    cont(function(v) {
        if (ab.callbackd == null) {
            ab.then = function(callback) {
                requestAnimationFrame(function() {
                    callback(v);
                });
            }
        } else {
            ab.callbackd(v);
        }
    })
    if (ab.then == null) {
        ab.then = function(callback) {
            ab.callbackd = callback;
        }
    }
    return ab;
}

MockPromise.all = function(...promises) {
    ab1 = {}
    ab1.result = []
    ab1.flag = []
    ab1.callback = null

    index = 0;
    for (const p of promises) {
        ab1.result.push(0);
        ab1.flag.push(false);
        index = index + 1;
    }

    index = 0;
    for (const p of promises) {
        var cb = function(ind){
            return function(v) {
                ab1.result[ind] = v;
                ab1.flag[ind] = true;
                if (ab1.flag.indexOf(false) == -1 && ab1.callback != null) {
                    ab1.callback(...ab1.result);
                    ab1.callback = null;
                }
            }
        }(index);
        p.then(cb)
        index = index + 1;
    }
    ab1.then = function(callback) {
        requestAnimationFrame(function() {
            if (ab1.flag.indexOf(false) == -1) {
                callback(...ab1.result);
            } else {
                ab1.callback = callback;
            }
        });
    }
    return ab1;
}
MockPromise.any = function(...promises) {
    ab2 = {}
    ab2.result = []
    ab2.flag = []
    ab2.callback = null

    index = 0;
    for (const p of promises) {
        ab2.result.push(0);
        ab2.flag.push(false);
        index = index + 1;
    }

    index = 0;
    for (const p of promises) {
        var cb = (function(ind) {
            return function(v) {
                ab2.result[ind] = v;
                ab2.flag[ind] = true;
                anyIndex = ab2.flag.indexOf(true);
                if (anyIndex > -1 && ab2.callback != null) {
                    ab2.callback(ab2.result[anyIndex]);
                    ab2.callback = null;
                }
            }
        })(index);
        p.then(cb);
        index = index + 1;
    }
    ab2.then = function(callback) {
        requestAnimationFrame(function() {
            anyIndex = ab2.flag.indexOf(true);
            if (anyIndex > -1) {
                callback(ab2.result[anyIndex]);
            } else {
                ab2.callback = callback;
            }
        });
    }
    return ab2;
}
ab = new MockPromise(function(resolve) {
    resolve(10);
});
ab.then(function(result) {
        console.log("do", result);
    });

ab1 = MockPromise(function(resolve) {
    resolve(50);
});
ab2 = MockPromise(function(resolve) {
    resolve(60);
});
MockPromise.all(ab1, ab2)
    .then(function(...result) {
        total = 0;
        for (const e of result) {
            total = e + total;
        }
        console.log("all", total);
    })

ab3 = MockPromise(function(resolve) {
    setTimeout(function() {
        resolve(5);
    }, 0);
});
ab4 = MockPromise(function(resolve) {
    setTimeout(function() {
        resolve(6);
    }, 2);
});
MockPromise.any(ab3, ab4)
    .then(function(result) {
        console.log("any", result);
    })
