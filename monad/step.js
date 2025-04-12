// 标识当前缓存位置
let pos = 0;
// 缓存任务运行结果
const cache = [];

const task1 = () => {
  console.log(pos, cache[pos]);
  if (cache[pos]) {
    return cache[pos++];
  } else {
    throw new Promise(resolve => setTimeout(() => resolve("task1"), 1000));
  }
};

const task2 = () => {
  console.log(pos, cache[pos]);
  if (cache[pos]) {
    return cache[pos++];
  } else {
    throw new Promise(resolve => setTimeout(() => resolve("task2"), 1000));
  }
};

const main = () => {
  const step = () => {
    pos = 0;
    try {
      console.log(pos, cache, "=====1111");
      const ret1 = task1();
      console.log(pos, cache, "=====2222");
      const ret2 = task2();
      console.log(pos, cache, "=====3333");
      console.log(ret1, ret2);
      console.log(pos, cache, "=====4444");
    } catch (task) {
      task.then(value => {
        cache[pos++] = value;
        // 重启流程
        // return step();
        step();
      });
    }
  };

  // return step();
  step();
};

main();
