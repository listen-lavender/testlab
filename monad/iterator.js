// 基于之前的 Continuation Monad 和 callCC
const Cont = (runCont) => ({
  runCont,
  chain: (f) => Cont((resolve) => runCont((a) => f(a).runCont(resolve))),
  map: (f) => Cont((resolve) => runCont((a) => resolve(f(a)))),
});

Cont.of = (value) => Cont((resolve) => resolve(value));

const callCC = (f) =>
  Cont((resolve) => {
    f((value) => Cont(() => resolve(value))).runCont(resolve);
  });

// 生成器构造函数
function generator(genFn) {
  let step = null; // 保存当前 continuation
  let done = false;

  // 迭代器对象
  const iterator = {
    next: (value) =>
      callCC((escape) => {
        if (done) return Cont.of({ value: undefined, done: true });

        // 首次执行或恢复执行
        const cont = step ? step(value) : genFn(escape);
        return cont.chain((result) => {
          if (result && result.done) {
            done = true;
            return Cont.of({ value: result.value, done: true });
          } else {
            step = (val) => genFn(escape).chain((res) => Cont.of(res)); // 保存 continuation
            return Cont.of({ value: result, done: false });
          }
        });
      }).runCont((x) => x), // 立即执行，返回同步结果
  };

  return iterator;
}

// 示例：生成器函数
function* naturalNumbers() {
  let n = 0;
  while (true) {
    yield n++;
  }
}

// 用我们的实现模拟
const natGen = generator(function (yieldValue) {
  let n = 0;
  return callCC(function (escape) {
    const loop = () =>
      Cont.of(n++)
        .chain((val) => yieldValue(val)) // 模拟 yield
        .chain(() => loop()); // 循环

    return loop();
  });
});

// 使用迭代器
natGen.next()(console.log); // { value: 0, done: false }
console.log("====2", natGen.next()); // { value: 1, done: false }
console.log("====3", natGen.next()); // { value: 2, done: false }

