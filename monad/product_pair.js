// ============ 基础工具函数 ============

// 序对构造函数（纯函数式）
const pair = a => b => f => f(a)(b);

// 访问第一个元素
const first = p => p(a => b => a);

// 访问第二个元素
const second = p => p(a => b => b);

// ============ 生成器核心实现 ============

// 创建生成器步进函数
const createStep = value => next => pair(value)(next);

// 获取当前值
const getValue = step => first(step);

// 获取下一步函数
const getNext = step => second(step);

// 初始生成器
const init = () => {
  // 使用闭包保持状态
  const makeSteps = counter => {
    // const currentValue = counter;
    // const nextStep = () => makeSteps(counter + 1);
    
    return createStep(counter)(() => makeSteps(counter + 1));
  };
  
  return makeSteps(0);
};

// ============ 高阶生成器操作 ============

// 生成器映射
const mapGenerator = generator => transform => {
  const mapStep = step => {
    const value = getValue(step);
    const next = getNext(step);
    
    const transformedValue = transform(value);
    const mappedNext = () => mapStep(next());
    
    return createStep(transformedValue)(mappedNext);
  };
  
  return mapStep(generator);
};

// 生成器过滤
const filterGenerator = generator => predicate => {
  const filterStep = step => {
    const value = getValue(step);
    const next = getNext(step);
    
    if (predicate(value)) {
      const filteredNext = () => filterStep(next());
      return createStep(value)(filteredNext);
    } else {
      // 跳过当前值，继续下一个
      return filterStep(next());
    }
  };
  
  return filterStep(generator);
};

// 生成器取前 n 个元素
const takeGenerator = generator => n => {
  const takeStep = (step, count) => {
    if (count >= n) {
      return createStep(null)(() => createStep(null)(() => null));
    }
    
    const value = getValue(step);
    const next = getNext(step);
    
    const takeNext = () => takeStep(next(), count + 1);
    return createStep(value)(takeNext);
  };
  
  return takeStep(generator, 0);
};

// 生成器折叠（归约）
const foldGenerator = generator => initial => reducer => {
  const foldStep = (step, accumulator) => {
    const value = getValue(step);
    const next = getNext(step);
    
    if (value === null && next === null) {
      return accumulator;
    }
    
    const newAccumulator = reducer(accumulator)(value);
    return foldStep(next(), newAccumulator);
  };
  
  return foldStep(generator, initial);
};

// 生成器转换为数组
const generatorToList = generator => {
  const collect = (step, result) => {
    const value = getValue(step);
    const next = getNext(step);
    
    if (value === null && next === null) {
      return result;
    }
    
    return collect(next(), result(value));
  };
  
  return collect(generator, list => list);
};

// ============ 组合生成器 ============

// 生成器组合（类似 flatMap）
const flatMapGenerator = generator => f => {
  const flatMapStep = (outerStep, innerStep) => {
    if (innerStep !== null) {
      const innerValue = getValue(innerStep);
      const innerNext = getNext(innerStep);
      
      const newInnerValue = innerValue;
      const newInnerNext = () => flatMapStep(outerStep, innerNext());
      
      return createStep(newInnerValue)(newInnerNext);
    } else {
      const outerValue = getValue(outerStep);
      const outerNext = getNext(outerStep);
      
      const innerGenerator = f(outerValue);
      return flatMapStep(outerNext(), innerGenerator);
    }
  };
  
  return flatMapStep(generator, null);
};

// 生成器拼接
const concatGenerators = generator1 => generator2 => {
  const concatStep = step => {
    const value = getValue(step);
    const next = getNext(step);
    
    if (value === null && next === null) {
      // 切换到第二个生成器
      return generator2;
    }
    
    const concatNext = () => concatStep(next());
    return createStep(value)(concatNext);
  };
  
  return concatStep(generator1);
};

// ============ 实用生成器 ============

// 范围生成器
const rangeGenerator = start => end => {
  const rangeStep = current => {
    if (current > end) {
      return createStep(null)(() => createStep(null)(() => null));
    }
    
    const next = () => rangeStep(current + 1);
    return createStep(current)(next);
  };
  
  return rangeStep(start);
};

// 斐波那契生成器
const fibonacciGenerator = () => {
  const fibStep = (a, b) => {
    const next = () => fibStep(b, a + b);
    return createStep(a)(next);
  };
  
  return fibStep(0)(1);
};

// 无限自然数生成器
const naturalNumbers = () => {
  const natStep = n => {
    const next = () => natStep(n + 1);
    return createStep(n)(next);
  };
  
  return natStep(0);
};

// ============ 测试函数 ============

// 测试基本生成器
const testBasicGenerator = () => {
  console.log('=== 基本生成器测试 ===');
  
  // 创建初始生成器
  const step0 = init();
  console.log('初始值:', getValue(step0));  // 0
  
  // 执行下一步
  const next1 = getNext(step0);
  const step1 = next1();
  console.log('第一次调用 next():', getValue(step1));  // 1
  
  // 继续执行
  const next2 = getNext(step1);
  const step2 = next2();
  console.log('第二次调用 next():', getValue(step2));  // 2
  
  // 可以继续无限执行
  const next3 = getNext(step2);
  const step3 = next3();
  console.log('第三次调用 next():', getValue(step3));  // 3
};

// 测试高阶操作
const testHighOrderOperations = () => {
  console.log('\n=== 高阶操作测试 ===');
  
  // 创建范围生成器
  const range = rangeGenerator(1)(5);
  
  // 映射：每个值乘以2
  const doubled = mapGenerator(range)(x => x * 2);
  
  // 过滤：只保留大于5的值
  const filtered = filterGenerator(doubled)(x => x > 5);
  
  // 取前3个
  const taken = takeGenerator(filtered)(3);
  
  console.log('原始范围 [1-5]:', generatorToList(range));
  console.log('映射后（乘以2）:', generatorToList(doubled));
  console.log('过滤后（>5）:', generatorToList(filtered));
  console.log('取前3个:', generatorToList(taken));
  
  // 测试折叠操作
  const sum = foldGenerator(taken)(0)(acc => value => acc + value);
  console.log('求和结果:', sum);
};

// 测试组合生成器
const testCombinedGenerators = () => {
  console.log('\n=== 组合生成器测试 ===');
  
  // 斐波那契数列
  const fib = fibonacciGenerator();
  const fibFirst10 = takeGenerator(fib)(10);
  console.log('前10个斐波那契数:', generatorToList(fibFirst10));
  
  // 映射和过滤组合
  const natural = naturalNumbers();
  const transformed = mapGenerator(natural)(n => n * n);
  const evenSquares = filterGenerator(transformed)(x => x % 2 === 0);
  const first5EvenSquares = takeGenerator(evenSquares)(5);
  
  console.log('前5个偶数的平方:', generatorToList(first5EvenSquares));
  
  // 生成器拼接
  const range1 = rangeGenerator(1)(3);
  const range2 = rangeGenerator(4)(6);
  const concatenated = concatGenerators(range1)(range2);
  console.log('拼接两个范围:', generatorToList(concatenated));
};

// 测试复杂操作链
const testComplexPipeline = () => {
  console.log('\n=== 复杂操作链测试 ===');
  
  // 创建复杂的处理管道
  const result = foldGenerator(
    takeGenerator(
      filterGenerator(
        mapGenerator(
          naturalNumbers()
        )(n => n * 3 + 1)
      )(x => x % 2 === 0)
    )(5)
  )(0)(acc => value => acc + value);
  
  console.log('复杂管道计算结果:', result);
  
  // 另一种写法（更函数式）
  const pipeline = generator => 
    foldGenerator(
      takeGenerator(
        filterGenerator(
          mapGenerator(generator)(n => n * 3 + 1)
        )(x => x % 2 === 0)
      )(5)
    )(0)(acc => value => acc + value);
  
  const pipelineResult = pipeline(naturalNumbers());
  console.log('管道函数计算结果:', pipelineResult);
};

// 测试自定义步进
const testCustomStepping = () => {
  console.log('\n=== 自定义步进测试 ===');
  
  // 创建一个每次增加2的生成器
  const stepBy2 = () => {
    const step = n => {
      const next = () => step(n + 2);
      return createStep(n)(next);
    };
    return step(0);
  };
  
  const generator = stepBy2();
  const first5 = takeGenerator(generator)(5);
  console.log('每次增加2的前5个数:', generatorToList(first5));
  
  // 创建指数增长的生成器
  const exponential = () => {
    const step = (base, exponent) => {
      const value = Math.pow(base, exponent);
      const next = () => step(base, exponent + 1);
      return createStep(value)(next);
    };
    return step(2, 0);
  };
  
  const expGen = exponential();
  const first5Exp = takeGenerator(expGen)(5);
  console.log('2的幂次方（前5个）:', generatorToList(first5Exp));
};

// 运行所有测试
const runAllTests = () => {
  console.log('======= 纯函数式生成器测试 =======\n');
  
  testBasicGenerator();
  // testHighOrderOperations();
  // testCombinedGenerators();
  // testComplexPipeline();
  // testCustomStepping();
  
  console.log('\n======= 所有测试完成 =======');
};

// 执行测试
runAllTests();

// ============ 实用示例 ============

// 生成器分页示例
const createPagedGenerator = pageSize => {
  const fetchPage = pageNumber => {
    // 模拟获取页面数据
    const start = pageNumber * pageSize;
    const end = start + pageSize;
    return rangeGenerator(start)(end - 1);
  };
  
  const pagedStep = pageNumber => {
    const pageData = fetchPage(pageNumber);
    const nextPage = () => pagedStep(pageNumber + 1);
    
    return concatGenerators(pageData)(nextPage);
  };
  
  return pagedStep(0);
};

// 惰性求值示例
const lazyEvaluationExample = () => {
  console.log('\n=== 惰性求值示例 ===');
  
  // 创建一个昂贵的计算（实际上不会立即执行）
  const expensiveComputation = n => {
    console.log(`执行昂贵计算: ${n}`);
    return n * n;
  };
  
  // 创建生成器时不会立即计算
  const generator = mapGenerator(naturalNumbers())(expensiveComputation);
  
  // 只在需要时计算
  const first3 = takeGenerator(generator)(3);
  console.log('计算结果:', generatorToList(first3));
};

// 状态机生成器示例
const stateMachineGenerator = initialState => transition => {
  const step = state => {
    const nextState = transition(state);
    const next = () => step(nextState);
    return createStep(state)(next);
  };
  
  return step(initialState);
};
