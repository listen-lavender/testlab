// https://www.reddit.com/r/ProgrammingLanguages/comments/ieyl8r/exceptions_without_stack_unwinding_and_vice_versa/

console.log("=============CPS变换一阶");

function logArray(arr) {
   forEachCpsRec(0, arr, function(ele, next) {  // (*)
       console.log(ele);
       next();
   });
}

function forEachCpsRec(index, arr, callback) {
   if (index < arr.length) {
       callback(arr[index], function() {
           forEachCpsRec(index+1, arr, callback);
       });
   } else {
   }
}

logArray([1,2,3,4]);

console.log("=============CPS变换二阶");

arrayNext = function(index) {
    return function(arr) {
        return function(callback) {
            if (index < arr.length) {
               callback(arr[index], function() {
                   arrayNext(index+1)(arr)(callback);
               });
           } else {
           }
        }
    }
}

arrayNext(0)([1,2,3,4])(function(ele, next) {  // (*)
   console.log(ele);
   next();
})


console.log("=============CPS变换三阶");

const factorial = (num) => num == 0 ? 1 : factorial(num-1)*num;
console.log(factorial(5));

function unit(v) {
    return v;
}

console.log(function(self) {
    return function(num) {
        console.log("=======", num);
        return num == 0 ? 1 : self(num-1)*num
    }
}(unit)(5));

// console.log(((self) =>(num) => num == 0 ? 1 : self(num-1)*num)(unit)(5));
// console.log(((self) =>(num) => num == 0 ? 1 : self(self)(num-1)*num)((self) =>(num) => num == 0 ? 1 : self(self)(num-1)*num)(5));

// console.log(((self) =>(num) => num == 0 ? 1 : self(self)(num-1)*num)
// ((self) =>(num) => num == 0 ? 1 : self(self)(num-1)*num)
// (5));

// ((f) => (g=>g(g))((x) => f((y) => x(x)(y))))
//     (self => num => num == 0 ? 1 : self(num-1)*num)
//     (5);

// const YJ =  function(f) {
//                 g = function(x) {
//                     return f(function(y){
//                         console.log(">>>>>>>>>>>>>>", y);
//                         return x(x)(y)
//                     })
//                 }
//                 return function(g) {
//                     return g(g)
//                 }(g)
//             }(function(self) {
//                 return function(num) {
//                     return num == 0 ? 1 : self(num-1)*num;
//                 }
//             });

// console.log(YJ(5));

// console.log(((f) => (g=>g(g))((x) => f((y) => x(x)(y))))
//     (self => num => num == 0 ? 1 : self(num-1)*num)
//     (5));

// function(num) {
//     return [num, function(){
//         return [num-1, function(){
//             return [num-2, function(){
//                 return [num-3, function(){
                    
//                 }]
//             }]
//         }]
//     }]
// }

// function GN(num) {
//     return function(callback) {
//         callback(num)
//         return function(callback) {
//             callback(num-1)
//             return function(callback) {

//             }
//         }
//     }
// }

// next = GN(5)
// next = next(console.log)
// next = next(console.log)



// (function(f){
//     return f(f);
// })(function(f){
//     return function(s){
//         return n => n < 2 ? 1 : n + s(n - 1);
//     }(f(f));
// });

// (f => f(f))(f => (s => n => n < 2 ? 1 : n + s(n - 1))((a) => f(f)(a)))(5);

(function(f){
    return f(f);
})(function(f){
    // return function(s){
    //     return n => n < 2 ? 1 : n + s(n - 1);
    // }((a) => f(f)(a));
    return n => n < 2 ? 1 : n + f(f)(n - 1);
})(5);

// (f => f(f))(f => (s => n => n < 2 ? 1 : n + s(n - 1))((a) => f(f)(a)))(5);

(t => (f => f(f))(f => t((a) => f(f)(a)))) (s => n => n < 2 ? 1 : n + s(n - 1))(5);



console.log("=============call/cc1阶");
// const callCCK1 = (f, k) => f((v, ignoredK) => k(v), k);

const callCCK1 = function(f, k) {
    return f(function(v, ignoredK){
        // return k(v);
        console.log("结束了");
    }, k);
};

const sumEvenK1 = (n, ek) => {
  callCCK1((exitK, k) => {
    const loopK = (n, k) => {
      if (n === 0) {
        k(0);
      } else if (n < 0) {
        // k(0);
        exitK(0, k);
      } else {
        loopK(n - 2, loopResult => k(n + loopResult));
      }
    };
    loopK(n, k);
  }, ek);
}
sumEvenK1(6, v => console.log(v)); // prints 12
sumEvenK1(5, v => console.log(v)); // prints 0

console.log("=============call/cc2阶");
// const callCCK2 = (f, k) => f((v, ignoredK) => k(v), k);

const callCCK2 = function(f, k) {
    return f(function(v, ignoredK){
        // return k(v);
        console.log("结束了");
    }, k);
};

// function callcc(f,cc) { 
//   f(function(x, ret) {  },  cc)
// }


const sumEvenK2 = (n, ek) => {
  callCCK2((exitK, k) => {
    const loopK = (n, k) => {
      if (n === 0) {
        k(0);
      } else if (n < 0) {
        // k(0);
        exitK(0, k);
      } else {
        loopK(n - 2, loopResult => k(n + loopResult));
      }
    };
    loopK(n, k);
  }, ek);
}
sumEvenK2(6, v => console.log(v)); // prints 12
sumEvenK2(5, v => console.log(v)); // prints 0

console.log("=============call/cc3阶");
const sumEven3 = (n, ek) => {
  const loopK = (n, k) => {
    if (n === 0) {
      k(0);
    } else if (n < 0) {
      ek(0);
    } else {
      loopK(n - 2, loopResult => k(n + loopResult));
    }
  };
  loopK(n, ek);
}
sumEven3(6, v => console.log(v)); // prints 12
sumEven3(5, v => console.log(v)); // prints 0


// https://oychao.github.io/2018/10/23/javascript/37_y_combinator/
// https://stackoverflow.com/questions/75299540/call-cc-example-in-javascript
// https://mqcreaple.github.io/blog/2022/09/02/y-combinator.html?utm_source=pocket_shared
// https://mqcreaple.github.io/blog/2022/08/27/lambda.html
// https://en.m.wikipedia.org/wiki/Call_stack#STACK-FRAME

// function callcc (f,cc) { 
//   f(function(x,k) { cc(x) },cc)
// }


// Ω := ( (λx. (x x)) (λx. (x x)) )

//    = ( (λx. (x x)) (λx. (x x)) )


// ( (λx. f (x x)) (λx. f (x x)) )

// = f ( (λx. f (x x)) (λx. f (x x)) ) 
// = f ( f ( (λx. f (x x)) (λx. f (x x)) ) )
// = 

// (Y f) = (f (Y f))
//       = (f (f (Y f)))
//       = (f (f (f (Y f))))

console.log("=============call/cc4阶");
const callcc = (f,cc) => { f(cc,cc) }

const forEach = (f, lst, cc) => {
  const partialForEach = (f, lst, start, cc) => {
    if (start === lst.length) {
      cc();
    } else {
      f(lst[start], () => partialForEach(f, lst, start+1, cc));
    }
  }
  partialForEach(f, lst, 0, cc)
};

const generate_one_element_a_time = lst => {
  let control_state = (ret) => {
    forEach(
      (element, cc) => {
        callcc(
          (resume_here) => {
            control_state = resume_here
            ret(element)
          },
          c => {
            ret = c
            cc()
          }
        )
      }, 
      lst, 
      () => ret("you-fell-off-the-end")
    )
  }

  let generator = (cc) => {
    callcc(control_state, cc)
  }
  
  return generator
}

const generate_digit = generate_one_element_a_time([0,1,2])

generate_digit(console.log) // 0
generate_digit(console.log) // 1
generate_digit(console.log) // 2
generate_digit(console.log) // you-fell-off-the-end
generate_digit(console.log) // you-fell-off-the-end

// etcd https://blog.mrcroxx.com/posts/code-reading/etcdraft-made-simple/6-readonly/
// http://firefly.kim/blog/etcd2-zh/











