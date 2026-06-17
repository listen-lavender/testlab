```
function trampoline(fn) {
  return function (...args) {
    let result = fn(...args);           // 第一步可能返回一个“延续”（函数）
    while (typeof result === 'function') { // 循环驱动，不递归
      result = result();                 // 执行一步，拿到下一步的延续或最终结果
    }
    return result;                       // 最终结果
  };
}

// 尾递归风格 + 返回“延续”而非直接递归
function factorial(n, acc = 1) {
  if (n <= 1) return acc;
  return () => factorial(n - 1, n * acc); // 返回 thunk，不压栈
}

const trampolinedFactorial = trampoline(factorial);
console.log(trampolinedFactorial(100000)); // 安全运行
```
thunk


```JavaScript
function factorial(n) {
  function iter(product, counter) {
    return counter > n ? product: iter(counter * product,counter + 1);
  }
  product, counter = iter(1, 1)
  next = function() {
    iter(product, counter)
  }
  return next;
}
```
请将以上iterative process，改造成JavaScript continuation的纯函数式，可暂停，可继续调用