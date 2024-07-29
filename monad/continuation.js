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
console.log(f(4));     // prints 16

const f1 = (x, callback)=>callback(x*x);

f1(4, console.log);

const squareCont = x=>callback=>callback(x*x);

squareCont(4)(console.log);

// const toCont = v=>callback=>callback(v);

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
             console.log("=======", v);
             func(v)(callback)
        })
    })
  }
  return cont
}

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

