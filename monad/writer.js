// function sine(x) {
//   return Math.sin(x);
// }

// function debugSine(x) {
//     return function(callback) {
//         callback(sine(x), 'sine was called.')
//     }
// }

// function twice(x) {
//     return x + x
// }

// function debugTwice(x) {
//     return function(callback) {
//         callback(twice(x), 'twice was called.')
//     }
// }

// function square(x) {
//     return x * x
// }

// function debugSquare(x) {
//     return function(callback) {
//         callback(square(x), 'square was called.')
//     }
// }

// function cube(x) {
//     return x * x * x
// }

// function debugCube(x) {
//     return function(callback) {
//         callback(cube(x), 'cube was called.')
//     }
// }

// function log(val, msg){
//   console.log("=======333");
//   console.log(val, msg)
// }

// // function writerID(val) {
// //   return WriterMonad([val, ""])
// // }


function writer(val) {
    return [val, '']
}

function WriterMonad(writer) {
  writer.fmap = function(func){
    return WriterMonad(func(val) {
        result = writer(val);
        val1 = func(result[0]);
        return [val1, result[1] + '' + val1];
    })
  }
  writer.applying = function(otherWriter){
    return WriterMonad(func(val) {
        result = writer(val);
        val1 = func(result[0]);
        return [val1, result[1] + '' + val1];
    })
  }
  writer.bind = function(lifter){
    return WriterMonad(func(val) {
        result = writer(val);
        result1 = lifter(result[0]);
        return [result1[0], result[1] + '' + result1[1]];
    })
  }
  return writer
}

writerID(1).applying(writerID(debugCube)).fmap(debugTwice).fmap(debugSquare)(log)

// function toCont(setter) {
//     return WriterMonad(function(v) {
//         return setter(v);
//     });
// }

// https://book.realworldhaskell.org/read/monad-transformers.html
// https://www.codeproject.com/Tips/801173/State-Monad-in-javascript
// https://blog.klipse.tech/javascript/2016/08/31/monads-javascript.html
// https://medium.com/@dtipson/functional-types-in-js-writing-a-writer-3bcd7eee2cb4
// https://blog.jcoglan.com/2011/03/05/translation-from-haskell-to-javascript-of-selected-portions-of-the-best-introduction-to-monads-ive-ever-read/
// http://blog.sigfpe.com/2006/08/you-could-have-invented-monads-and.html
// https://stackoverflow.com/questions/2704652/monad-in-plain-english-for-the-oop-programmer-with-no-fp-background


// function toCont(func) {
//     return WriterMonad(function(v){
//         return {
//             "val": func(v),
//             "log": v + ' ',
//         }
//     });
// }

// function WriterMonad(cont) {
//   cont.fmap = function(func){
//     return WriterMonad(function(v) {
//         const {val, log} = cont(v)
//         return {
//             "val": func(v),
//             "log": log + v + ' ',
//         }
//     })
//   }
//   cont.applying = function(otherCont){
//     return WriterMonad(function(v) {
//         const {val, log1} = cont(v)
//         const val = otherCont(val)
//         return {
//             "val": val,
//             "log": log1 + v + ' ',
//         }
//     })
//   }
//   cont.bind = function(func){
//     return WriterMonad(function(v) {
//         const {val, log1} = cont(v)
//         const {val, log2} = func(val)
//         return {
//             "val": val,
//             "log": log1 + log2,
//         }
        
//     })
//   }
//   return cont
// }

// console.log(toCont(0).fmap(x => x + 1).fmap(x => x + 2).fmap(x => x + 3)());

// console.log(toCont(0).applying(toCont(x => x + 1)).applying(toCont(x => x + 2)).applying(toCont(x => x + 3))());

// console.log(toCont(0).bind(y => toCont(y + 1)).bind(y => toCont(y + 2)).bind(y => toCont(y + 3))());

