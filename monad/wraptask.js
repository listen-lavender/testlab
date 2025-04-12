// 当前执行的位置
let pos = 0;

// 任务执行结果缓存
let cache = [];

const wrapTask = task => {
  if (cache[pos]) {
    return cache[pos++];
  }
  throw task;
};

const runner = process => {
  // cache = [];
  // return step();
  step();

  function step() {
    // 每一步都从头执行
    pos = 0;
    try {
      const ret = process();
      console.log("========1111", ret);
      // return ret;
    } catch (task) {
      task.then(value => {
      // return task.then(value => {
        cache[pos] = value;
        step();
        // return step();
      });
    }
  }
};

const createRunner = (of, chain) => process => {
  return step();

  function step() {
    // pos = 0;
    try {
      return of(process());
    } catch (task) {
      return chain(task, value => {
        // 缓存当前执行结果
        cache[pos] = value;
        // 重启流程
        return step();
      });
    } finally {
      // 每次执行完，释放对上下文的掌控
      console.log("abc");
    }
  }
};

const run = createRunner(v => Promise.resolve(v), (m, f) => m.then(f));


const task1 = () =>
  wrapTask(new Promise(resolve => setTimeout(() => resolve("task1"), 1000)));

const task2 = () =>
  wrapTask(new Promise(resolve => setTimeout(() => resolve("task2"), 1000)));

const main = () => {
  // const ret1 = run(task1);
  // const ret2 = run(task2);
  console.log("=======aaaa");
  const ret1 = task1();
  console.log("=======bbbb");
  const ret2 = task2();
  console.log("=======cccc");
  console.log("result is", ret1, ret2);
};

// run(main);
runner(main);
