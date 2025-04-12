// https://zhuanlan.zhihu.com/p/24237196

    // function Generator(genFunc) {
    //     this._genFunc = genFunc;
    // }
    // Generator.prototype.next = function (success) {
    //     this._genFunc(function (result, nextGenFunc) {
    //         this._genFunc = nextGenFunc;
    //         success(result);
    //     });
    // };

function toGenerator(i) {
  function next(yield) {
    yield(i++, next);
  }
  return GeneratorMonad(next);
}

function GeneratorMonad(generator){
  generator.next = function(callback) {
    generator(function(v, next){
      callback(v);
    })
    // GeneratorMonad(function(yield){
    //   generator(function(v, next){
    //     yield(v++, next);
    //   })
    // })
  }
  generator.fmap = function(func){
    return GeneratorMonad(function(yield){
      generator(function(v, next){
        yield(func(v), next);
      })
    })
  }
  return generator
}

var g = toGenerator(1).fmap(x => x * x);
g.next(function(result) {
    console.log(result);
});
g.next(function(result) {
    console.log(result);
});
g.next(function(result) {
    console.log(result);
});



// function toGenerator(i) {
//   function(next) {
//     function(yield) {
//         yield(i++, next)
//     }
//   }(function(yield) {
//         yield(i++, next)
//   })


//   nextStep = function(callback) {
//     callback(i++, nextStep);
//   }
//   return ContinuationMonad(function(callback) {
//     callback(v)
//   })
// }

// function Generator(genFunc) {
//     this._genFunc = genFunc;
// }

// function countUpCps() {
//     var i=0;
//     function nextStep(yieldIt) {
//         yieldIt(i++, nextStep);
//     }
//     return new Generator(nextStep);
// }

// Generator.prototype.next = function(success) {
//     this._genFunc(function(result, nextGenFunc) {
//         this._genFunc = nextGenFunc;
//         success(result);
//     });
// };

// var g = countUpCps();
// g.next(function(result) {
//     console.log(result);

//     g.next(function(result) {
//         console.log(result);

//         g.next(function(result) {
//             console.log(result);
//             // etc.
//         });
//     });
// });

// g.next(function(result) {
//     console.log(result);
//     // etc.
// });
