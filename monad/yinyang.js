// ============ 基础 Continuation 工具 ============

// Continuation 构造函数（纯函数式）
const cont = continuation => value => next => continuation(value)(next);

// 创建 continuation 值
const makeCont = value => cont(k => k(value));

// 应用 continuation（CPS 风格）
const applyCont = continuation => value => continuation(value);

// 绑定 continuation（类似 Haskell 的 >>=）
const bindCont = m => f => cont(k => 
  applyCont(m)(value => 
    applyCont(f(value))(k)
  )
);

// Continuation 组合器
const mapCont = f => m => bindCont(m)(value => makeCont(f(value)));
const liftA2 = f => ma => mb => bindCont(ma)(a => 
  bindCont(mb)(b => makeCont(f(a)(b)))
);

// 恒等 continuation
const idCont = k => k;

// ============ 阶乘的 Continuation 实现 ============

// 基本 CPS 阶乘
const factorialCPS = n => {
  const iter = product => counter => k =>
    counter > n
      ? k(product)
      : iter(product * counter)(counter + 1)(k);
  
  return iter(1)(1)(idCont);
};

// 可暂停的 CPS 阶乘（返回 continuation）
const factorialPausable = n => {
  const makeStep = product => counter => nextStep =>
    counter > n
      ? cont(k => k({
          done: true,
          value: product,
          next: () => makeStep(product)(counter)(nextStep)
        }))
      : cont(k => k({
          done: false,
          product: product * counter,
          counter: counter + 1,
          max: n,
          next: () => makeStep(product * counter)(counter + 1)(nextStep)
        }));
  
  return makeStep(1)(1)(() => makeStep(1)(1)(null));
};

// 测试可暂停阶乘
const testPausableFactorial = () => {
  console.log('\n=== 可暂停阶乘测试 ===');
  
  const fact5 = factorialPausable(5);
  
  // 手动分步执行
  const step1 = applyCont(fact5)(value => value);
  console.log('步骤1:', step1.done ? '完成' : '继续');
  
  if (!step1.done) {
    const nextStep = step1.next;
    console.log('>>>>', nextStep, step1);
    const step2 = applyCont(nextStep())(value => value);
    console.log('步骤2: product =', step2.product, ', counter =', step2.counter);
    
    // 继续执行直到完成
    let current = nextStep;
    let steps = 2;
    let result = null;
    
    while (!result) {
      const stepResult = applyCont(current())(v => v);
      if (stepResult.done) {
        result = stepResult.value;
        console.log(`步骤${steps + 1}: 完成, 结果 = ${result}`);
      } else {
        steps++;
        current = stepResult.next;
        console.log(`步骤${steps}: product = ${stepResult.product}, counter = ${stepResult.counter}`);
      }
    }
  }
};

// 纯函数式状态传递阶乘
const factorialState = n => {
  const initialState = () => cont(k => k({
    type: 'iter',
    product: 1,
    counter: 1
  }));
  
  const stepState = state => cont(k => 
    state.counter > n
      ? k({
          done: true,
          result: state.product
        })
      : k({
          done: false,
          next: () => cont(k2 => k2({
            type: 'iter',
            product: state.product * state.counter,
            counter: state.counter + 1
          }))
        })
  );
  
  return initialState();
};

// ============ Continuation Monad ============

// Continuation Monad 实现
const ContMonad = {
  // return :: a -> Cont r a
  of: value => cont(k => k(value)),
  
  // bind :: Cont r a -> (a -> Cont r b) -> Cont r b
  bind: m => f => cont(k => 
    applyCont(m)(a => 
      applyCont(f(a))(k)
    )
  ),
  
  // map :: (a -> b) -> Cont r a -> Cont r b
  map: f => m => ContMonad.bind(m)(a => ContMonad.of(f(a))),
  
  // ap :: Cont r (a -> b) -> Cont r a -> Cont r b
  ap: mf => ma => cont(k =>
    applyCont(mf)(f =>
      applyCont(ma)(a =>
        k(f(a))
      )
    )
  )
};

// do-notation 风格的 continuation 组合
const doCont = gen => {
  const generator = gen();
  
  const step = value => {
    const result = generator.next(value);
    if (result.done) {
      return ContMonad.of(result.value);
    }
    return ContMonad.bind(result.value)(step);
  };
  
  return step();
};

// ============ 纯函数式分步执行器 ============

// 执行 continuation 的一步
const runStep = continuation => 
  applyCont(continuation)(value => value);

// 执行 continuation 直到完成
const runCont = continuation => {
  const run = current => 
    applyCont(current)(value => 
      value.done
        ? value.result
        : run(value.next())
    );
  
  return run(continuation);
};

// 异步分步执行器（依然纯函数）
const asyncStepRunner = continuation => {
  const createRunner = () => {
    let current = continuation;
    let steps = 0;
    
    const step = () => cont(k =>
      applyCont(current)(value => {
        if (value.done) {
          return k({
            done: true,
            result: value.result,
            steps: steps
          });
        }
        
        current = value.next();
        steps = steps + 1;
        
        return k({
          done: false,
          steps: steps,
          hasMore: true
        });
      })
    );
    
    return step;
  };
  
  return createRunner();
};

// ============ 纯函数式 Trampoline ============

const trampoline = thunk => {
  const run = bounce => 
    applyCont(bounce)(value => 
      value.done
        ? value.value
        : run(value.next)
    );
  
  return run(thunk);
};

const factorialTrampoline = n => {
  const makeThunk = product => counter => () =>
    cont(k => 
      counter > n
        ? k({ done: true, value: product })
        : k({
            done: false,
            next: makeThunk(product * counter)(counter + 1)()
          })
    );
  
  return trampoline(makeThunk(1)(1));
};

// ============ 测试函数 ============

// 测试基本 CPS 阶乘
const testBasicCPS = () => {
  console.log('=== 基本 CPS 阶乘测试 ===');
  
  const result5 = factorialCPS(5);
  console.log('5! =', result5);  // 120
  
  // 使用 continuation 处理结果
  const withCallback = n => {
    const iter = product => counter => k =>
      counter > n
        ? k(product)
        : iter(product * counter)(counter + 1)(k);
    
    return iter(1)(1)(result => {
      console.log(`计算完成: ${n}! = ${result}`);
      return result * 2;  // 可以继续处理
    });
  };
  
  const processed = withCallback(5);
  console.log('处理后的结果:', processed);  // 240
};

// 测试 State 风格阶乘
const testStateFactorial = () => {
  console.log('\n=== State 风格阶乘测试 ===');
  
  const fact5 = factorialState(5);
  const result = runCont(fact5);
  console.log('5! =', result);
  
  // 分步执行演示
  console.log('\n分步执行演示:');
  let state = applyCont(factorialState(5))(v => v);
  let step = 1;
  
  while (!state.done) {
    console.log(`步骤${step}: product = ${state.product}, counter = ${state.counter}`);
    state = applyCont(state.next())(v => v);
    step++;
  }
  
  console.log(`步骤${step}: 完成, 结果 = ${state.result}`);
};

// 测试 Continuation Monad
const testContMonad = () => {
  console.log('\n=== Continuation Monad 测试 ===');
  
  // 使用 do notation 风格
  const computation = doCont(function* () {
    const a = yield ContMonad.of(5);
    const b = yield ContMonad.of(6);
    const c = yield bindCont(ContMonad.of(7))(value => 
      ContMonad.of(value * 2)
    );
    
    return a + b + c;
  });
  
  const result = applyCont(computation)(value => value);
  console.log('Monad 计算结果:', result);  // 5 + 6 + 14 = 25
  
  // 阶乘的 monadic 版本
  const factorialMonad = n => {
    const iter = product => counter =>
      counter > n
        ? ContMonad.of(product)
        : ContMonad.bind(ContMonad.of(product * counter))(newProduct =>
            ContMonad.bind(ContMonad.of(counter + 1))(newCounter =>
              iter(newProduct)(newCounter)
            )
          );
    
    return iter(1)(1);
  };
  
  const factResult = applyCont(factorialMonad(5))(v => v);
  console.log('Monadic 阶乘 5! =', factResult);
};

// 测试 Trampoline
const testTrampoline = () => {
  console.log('\n=== Trampoline 测试 ===');
  
  const result = factorialTrampoline(5);
  console.log('Trampoline 5! =', result);
  
  // 大数测试（避免栈溢出）
  const largeResult = factorialTrampoline(10000);
  console.log('10000! 的位数:', largeResult.toString().length);
};

// 测试异步执行器
const testAsyncRunner = () => {
  console.log('\n=== 异步执行器测试 ===');
  
  const runner = asyncStepRunner(factorialState(5));
  
  // 执行三步
  const step1 = applyCont(runner())(v => v);
  console.log('执行第一步:', step1);
  
  const step2 = applyCont(runner())(v => v);
  console.log('执行第二步:', step2);
  
  const step3 = applyCont(runner())(v => v);
  console.log('执行第三步:', step3);
  
  // 继续执行直到完成
  let currentStep = step3;
  while (!currentStep.done && currentStep.hasMore) {
    currentStep = applyCont(runner())(v => v);
    console.log('继续执行:', currentStep);
  }
};

// 高阶 Continuation 操作
const testHighOrderCont = () => {
  console.log('\n=== 高阶 Continuation 操作 ===');
  
  // 组合多个 continuation
  const composeCont = (...conts) => cont(k =>
    conts.reduceRight(
      (acc, c) => applyCont(c)(value => applyCont(acc)(fn => k(fn(value)))),
      k(x => x)
    )
  );
  
  // 应用示例
  const add = x => y => x + y;
  const mult = x => y => x * y;
  
  const computation = composeCont(
    makeCont(5),
    mapCont(add(2)),
    mapCont(mult(3))
  );
  
  const result = applyCont(computation)(v => v);
  console.log('组合计算: ((5 + 2) * 3) =', result);  // 21
  
  // Continuation 流水线
  const pipeline = (...transforms) => value =>
    transforms.reduce(
      (acc, transform) => bindCont(acc)(transform),
      makeCont(value)
    );
  
  const transformPipe = pipeline(
    x => makeCont(x + 1),
    x => makeCont(x * 2),
    x => makeCont(x - 3)
  );
  
  const pipeResult = applyCont(transformPipe(10))(v => v);
  console.log('流水线计算 (10 -> +1 -> *2 -> -3) =', pipeResult);  // 19
};

// 运行所有测试
const runAllTests = () => {
  console.log('======= 纯函数式 Continuation 测试 =======\n');
  
  testBasicCPS();
  testPausableFactorial();
  testStateFactorial();
  testContMonad();
  testTrampoline();
  testAsyncRunner();
  testHighOrderCont();
  
  console.log('\n======= 所有测试完成 =======');
};

// 执行测试
runAllTests();