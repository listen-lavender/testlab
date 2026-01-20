// function one() {
//   return 1;
// }

// function transform(fun) {
//     return function() {
//         return 1 + fun()
//     }
// }

// function fmap(ori, fun) {
//     return function() {
//         return fun(ori())
//     }
// }

// function fmap(ori, fun) {
//     return fun(ori())
// }



const f = x=>x * x;
// console.log(f(4));     // prints 16

const f1 = (x, callback)=>callback(x*x);

// f1(4, console.log);

const squareCont = x=>callback=>callback(x*x);

// squareCont(4)(console.log);

// const toCont = v=>callback=>callback(v);

// // Continuation Monad构造函数
// const Cont = run => ({
//     // 运行延续
//     runCont: run,
    
//     // Monad的chain操作（也称为bind或flatMap）
//     chain: f => Cont(k => run(x => f(x).runCont(k))),
    
//     // Functor的map操作
//     map: f => this.chain(x => Cont.of(f(x)))
// })

// // of方法（也称为pure或return）
// Cont.of = x => Cont(k => k(x))

// // callWithCurrentContinuation (call/cc)实现
// Cont.callCC = f => Cont(k => f(a => Cont(_ => k(a))).runCont(k))

function toCont(v) {
  return ContinuationMonad(function(callback) {
    callback(v)
  })
}

function ContinuationMonad(cont) {
  cont.fmap = function(func){
    return ContinuationMonad(function(callback) {
        cont(function(v){
             console.log("=======", v);
             callback(func(v))
        })
    })
  }
  cont.applying = function(otherCont){
    return ContinuationMonad(function(callback) {
        cont(function(v){
           otherCont(function(func){
               console.log("=======", v);
               callback(func(v))
           })
        })
    })
  }
  cont.bind = function(func){
    return ContinuationMonad(function(callback) {
        cont(function(v){
             console.log("=======def", v);
             console.log("=======abc", func(v));
             func(v)(callback)
        })
    })
  }
  return cont
}

ContinuationMonad.of = function(v) {
    return ContinuationMonad(function(callback) {
        callback(v)
    })
}

ContinuationMonad.callCC = function(func){
    return ContinuationMonad(function(callback1){
        exit = function(a){
            console.log("======a", a)
            return ContinuationMonad(function(callback2){
                console.log("======b", callback2)
                console.log("======k", callback1)
                callback1(a)
            })
        }
        monad = func(exit)
        monad(callback1)
    })
}

const example2 = ContinuationMonad.callCC(exit => 
    ContinuationMonad.of(10)
        .bind(x => ContinuationMonad.of(x + 100)) // 这行不会执行
        .bind(x => exit(x * 2))  // 这里会提前退出
        .bind(x => ContinuationMonad.of(x + 100)) // 这行不会执行
)

example2(console.log) // 输出: 20 (而不是120)

// 使用call/cc实现类似try-catch的控制流
const tryDivide = (a, b) => ContinuationMonad.callCC(exit => 
    b === 0 
        ? exit("除数不能为零") 
        : ContinuationMonad.of(a / b)
)

const computation = tryDivide(10, 2)
    .bind(result => ContinuationMonad.of(`结果是: ${result}`))
    .bind(console.log)

computation(console.log) // 输出: "结果是: 5"

const errorComputation = tryDivide(10, 0)
    .bind(result => ContinuationMonad.of(`结果是: ${result}`))
    .bind(console.log)

errorComputation(console.log) // 输出: "除数不能为零"


// function toCont(v) {
//   return function(callback) {
//     callback(v)
//   }
// }

// var bind = (cont1, func) => {
//     return function(c) {
//         cont1(function(v){
//              console.log("=======", v);
//              func(v)(c)
//         })
//     }
// };

// var applying = (cont1, cont2) => {
//     return function(c) {
//         cont2(function(func){
//            cont1(function(v){
//                console.log("=======", v);
//                c(func(v))
//            })
//         })
//     }
// };

// var fmap = (cont1, func) => {
//     return function(c) {
//         cont1(function(v){
//              console.log("=======", v);
//              c(func(v))
//         })
//     }
// };

// var contComposition2 = bind(toCont(4), x => toCont((x => x + " hello")(x)));
// var contComposition3 = bind(contComposition2, x => toCont((x => x + " world")(x)));
// var contComposition4 = bind(contComposition3, x => toCont((x => x.length)(x)));
// contComposition2(console.log);
// contComposition3(console.log);
// contComposition4(console.log);

toCont(4).bind(y => toCont((x => x + " hello")(y))).bind(y => toCont((x => x + " world")(y))).bind(y => toCont((x => x.length)(y)))(console.log);
toCont(4).bind(y => toCont(y + " hello")).bind(y => toCont(y + " world")).bind(y => toCont(y.length))(console.log);


// var contCompositiona = fmap(toCont(4), x => x + " hello");
// var contCompositionb = fmap(contCompositiona, x => x + " world");
// var contCompositionc = fmap(contCompositionb, x => x.length);
// contCompositiona(console.log);
// contCompositionb(console.log);
// contCompositionc(console.log);

toCont(4).fmap(x => x + " hello").fmap(x => x + " world").fmap(x => x.length)(console.log);


// var contCompositionA = applying(toCont(4), toCont(x => x + " hello"));
// var contCompositionB = applying(contCompositionA, toCont(x => x + " world"));
// var contCompositionC = applying(contCompositionB, toCont(x => x.length));
// contCompositionA(console.log);
// contCompositionB(console.log);
// contCompositionC(console.log);

toCont(4).applying(toCont(x => x + " hello")).applying(toCont(x => x + " world")).applying(toCont(x => x.length))(console.log);

// toCont(z=>z+1);

// toCont(x=>x*x)
//         (y=>toCont(y(4))
//           (t=>console.log(t)));

// toCont(x=>x*x)
//         (y=>toCont(y(4))
//           (console.log));

// var vbind = (cont1, func) => (c => cont1(contResult => func(contResult)(c)));

// var vbind = (cont1, func) => {
//     return function(c) {
//         return cont1(function(op){
//              console.log("=======", op(5));
//              return func(op)(c)
//         })
//     }
// };

// //the initial 
// toCont(x => x * x)
//   (y => toCont(y(4))
//     (t => console.log(t)));


// //rewriting it with the vbind function
// var contComposition = vbind(toCont(x => x * x), y => toCont(y(4)));
// //this is still not evaluated. its a deffered computation 

// //here we actually collapse the continuation structure and observe the result 
// contComposition(t => console.log(t));

// function compose(f1, f2) {
//   return function(x) {
//     return f1(f2(x))
//   }
// }

// function impureAddOne(x){
//   return x + 1
// }

// function impureMultiplyByTwo(x) {
//   return 2 * x
// }

// function lazycompose(f1, f2) {
//   return function(x) {
//     return f1(f2(x)())()
//   }
// }

// function lazyImpureAddOne(x){
//   return function() {
//     console.log('add one!')
//     return x + 1
//   }
// }

// // Java 代码看多了之后我也学会取长变量名了^_^
// function lazyImpureMultiplyByTwo(x) {
//   return function() {
//     console.log('multiply by two!')
//     return 2 * x  
//   }
// }

// function Effect(f) {
//   return {
//     map: function(g) {
//       return Effect(function(x){
//         return g(f(x))
//       })
//     },
//     runWith: function(x) {
//       return f(x)
//     }
//   }
// }

// console.log(Effect(impureAddOne).map(impureMultiplyByTwo).runWith(2))

function toStream(callback) {
  return StreamMonad(function(getResult) {
    callback(getResult())
  })
}

function StreamMonad(cont) {
  cont.fmap = function(func){
    return StreamMonad(function(getResult) {
        cont(func(getResult))
    })
  }
  cont.applying = function(otherCont){
    return StreamMonad(function(getResult) {
        otherCont(function(func){
            console.log("=======", getResult);
            cont(func(getResult))
        })
    })
  }
  cont.bind = function(func){
    return StreamMonad(function(getResult) {
        func(getResult)(getResult)
        cont(getResult)
    })
  }
  return cont
}

