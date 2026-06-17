// ============ 基础工具函数 ============

// 序对构造函数（纯函数式）
const pair = a => b => f => f(a)(b);

// 访问第一个元素
const first = p => p(a => b => a);

// 访问第二个元素
const second = p => p(a => b => b);

// ============ Monad 通用接口 ============

// Monad类型构造函数
const Monad = M => ({
  // unit/return :: a -> M a
  unit: M.unit,
  
  // bind/flatMap :: M a -> (a -> M b) -> M b
  bind: M.bind,
  
  // map :: (a -> b) -> M a -> M b
  map: f => ma => M.bind(ma)(a => M.unit(f(a))),
  
  // join :: M (M a) -> M a
  join: mma => M.bind(mma)(ma => ma),
  
  // ap :: M (a -> b) -> M a -> M b
  ap: mf => ma => M.bind(mf)(f => M.bind(ma)(a => M.unit(f(a)))),
});

// ============ Reader Monad ============

// Reader类型构造函数：r -> a
const Reader = run => ({
  runReader: run,
  
  // unit :: a -> Reader r a
  unit: a => Reader(_ => a),
  
  // bind :: Reader r a -> (a -> Reader r b) -> Reader r b
  bind: f => Reader(r => {
    const a = run(r);
    return f(a).runReader(r);
  }),
  
  // ask :: Reader r r
  ask: Reader(r => r),
  
  // local :: (r -> r') -> Reader r' a -> Reader r a
  local: f => Reader(r => run(f(r))),
});

// Reader Monad实例
const ReaderM = Monad(Reader);

// ============ Writer Monad (使用字符串日志) ============

// Writer类型构造函数：pair(value)(log)
const Writer = p => ({
  getValue: () => first(p),
  getLog: () => second(p),
  
  // unit :: a -> Writer a
  unit: a => Writer(pair(a)("")),
  
  // bind :: Writer a -> (a -> Writer b) -> Writer b
  bind: f => {
    const currentValue = first(p);
    const currentLog = second(p);
    
    const nextWriter = f(currentValue);
    const nextValue = first(nextWriter);
    const nextLog = second(nextWriter);
    
    return Writer(pair(nextValue)(currentLog + nextLog));
  },
  
  // tell :: String -> Writer ()
  tell: msg => Writer(pair(undefined)(msg)),
});

// Writer Monad实例
const WriterM = Monad(Writer);

// ============ State Monad ============

// State类型构造函数：s -> pair(a)(s')
const State = run => ({
  runState: run,
  
  // unit :: a -> State s a
  unit: a => State(s => pair(a)(s)),
  
  // bind :: State s a -> (a -> State s b) -> State s b
  bind: f => State(s => {
    const result = run(s);
    const a = first(result);
    const s1 = second(result);
    return f(a).runState(s1);
  }),
  
  // get :: State s s
  get: State(s => pair(s)(s)),
  
  // put :: s -> State s ()
  put: s1 => State(_ => pair(undefined)(s1)),
  
  // modify :: (s -> s) -> State s ()
  modify: f => State(s => pair(undefined)(f(s))),
});

// State Monad实例
const StateM = Monad(State);

// ============ 辅助函数 ============

// 复合函数
const compose = (...fns) => x => fns.reduceRight((acc, fn) => fn(acc), x);

// 柯里化函数
const curry = (fn, arity = fn.length, ...args) =>
  arity <= args.length ? fn(...args) : curry.bind(null, fn, arity, ...args);

// ============ Reader Monad 示例 ============

// 环境配置读取示例
const readerExample = () => {
  console.log('=== Reader Monad 示例 ===');
  
  // 创建Reader计算
  const readerComputation = Reader(r => {
    // 从环境中读取配置
    const config = r;
    return `Hello, ${config.name}! You are ${config.age} years old.`;
  });
  
  // 使用do-notation风格
  const complexReader = readerComputation.bind(greeting => 
    Reader(r => {
      const config = r;
      return `${greeting} You work at ${config.company}.`;
    })
  ).bind(finalMessage => 
    Reader(_ => finalMessage.toUpperCase())
  );
  
  // 运行Reader
  const config = { name: 'Alice', age: 30, company: 'TechCorp' };
  const result = complexReader.runReader(config);
  console.log('Reader结果:', result);
  
  // 使用ask访问环境
  const withAsk = ReaderM.ask.bind(env =>
    ReaderM.unit(`Environment has ${Object.keys(env).length} keys`)
  );
  
  console.log('使用ask:', withAsk.runReader(config));
  
  // 使用local修改局部环境
  const localExample = Reader(r => `Original: ${r.value}`)
    .local(env => ({ ...env, value: 'MODIFIED' }))
    .bind(result1 => 
      Reader(r => `After local: ${result1}, Current: ${r.value}`)
    );
  
  const env = { value: 'ORIGINAL' };
  console.log('local示例:', localExample.runReader(env));
};

// ============ Writer Monad 示例 ============

// 带日志的计算示例
const writerExample = () => {
  console.log('\n=== Writer Monad 示例 ===');
  
  // 简单的Writer计算
  const logAddition = a => b => {
    const result = a + b;
    return Writer(pair(result)(`Added ${a} + ${b} = ${result}\n`));
  };
  
  const logMultiplication = a => b => {
    const result = a * b;
    return Writer(pair(result)(`Multiplied ${a} * ${b} = ${result}\n`));
  };
  
  // 组合Writer计算
  const computation = logAddition(3)(4).bind(result1 =>
    logMultiplication(result1)(2).bind(result2 =>
      Writer(pair(result2)(`Final result: ${result2}\n`))
    )
  );
  
  // 获取结果和日志
  const value = computation.getValue();
  const log = computation.getLog();
  
  console.log('Writer值:', value);
  console.log('Writer日志:');
  console.log(log);
  
  // 使用tell添加日志
  const withTell = Writer(pair(42)("Initialized\n"))
    .bind(x => WriterM.tell(`Got value: ${x}\n`).bind(_ =>
      Writer(pair(x * 2)(`Doubled to ${x * 2}\n`))
    ));
  
  console.log('使用tell的值:', withTell.getValue());
  console.log('使用tell的日志:');
  console.log(withTell.getLog());
};

// ============ State Monad 示例 ============

// 状态管理示例
const stateExample = () => {
  console.log('\n=== State Monad 示例 ===');
  
  // 计数器状态操作
  const increment = StateM.modify(s => s + 1);
  const decrement = StateM.modify(s => s - 1);
  
  // 获取并操作状态
  const getAndIncrement = StateM.get.bind(current =>
    increment.bind(_ =>
      StateM.unit(current)
    )
  );
  
  // 运行状态计算
  const computation = getAndIncrement.bind(oldValue =>
    StateM.get.bind(newValue =>
      StateM.unit(`Old: ${oldValue}, New: ${newValue}`)
    )
  );
  
  const initialState = 0;
  const result = computation.runState(initialState);
  const finalValue = first(result);
  const finalState = second(result);
  
  console.log('State结果:', finalValue);
  console.log('最终状态:', finalState);
  
  // 栈操作示例
  const push = value => State(stack => pair(undefined)([value, ...stack]));
  
  const pop = State(stack => {
    if (stack.length === 0) {
      return pair(null)(stack);
    }
    const [top, ...rest] = stack;
    return pair(top)(rest);
  });
  
  // 栈操作序列
  const stackComputation = push(10).bind(_ =>
    push(20).bind(_ =>
      pop.bind(top1 =>
        push(30).bind(_ =>
          pop.bind(top2 =>
            StateM.unit(`Popped: ${top1}, ${top2}`)
          )
        )
      )
    )
  );
  
  const stackResult = stackComputation.runState([]);
  console.log('栈操作结果:', first(stackResult));
  console.log('最终栈:', second(stackResult));
};

// ============ Monad组合示例 ============

// Reader + Writer组合
const combinedExample = () => {
  console.log('\n=== Monad组合示例 ===');
  
  // 模拟ReaderT Writer Monad转换器
  const ReaderWriter = run => ({
    runReaderWriter: run,
    
    unit: a => ReaderWriter(_ => Writer(pair(a)(""))),
    
    bind: f => ReaderWriter(r => {
      const writer = run(r);
      const value = writer.getValue();
      const log1 = writer.getLog();
      
      const next = f(value).runReaderWriter(r);
      const nextValue = next.getValue();
      const log2 = next.getLog();
      
      return Writer(pair(nextValue)(log1 + log2));
    }),
    
    ask: ReaderWriter(r => Writer(pair(r)(""))),
    
    tell: msg => ReaderWriter(_ => Writer(pair(undefined)(msg))),
  });
  
  // 组合计算示例
  const combined = ReaderWriter(r => {
    const config = r;
    return Writer(pair(config.base)(`Starting with base=${config.base}\n`));
  }).bind(base =>
    ReaderWriter.tell(`Processing with base ${base}\n`).bind(_ =>
      ReaderWriter(r => {
        const multiplier = r.multiplier || 1;
        const result = base * multiplier;
        return Writer(pair(result)(`Multiplied: ${base} * ${multiplier} = ${result}\n`));
      })
    )
  );
  
  const config = { base: 10, multiplier: 3 };
  const result = combined.runReaderWriter(config);
  
  console.log('组合Monad值:', result.getValue());
  console.log('组合Monad日志:');
  console.log(result.getLog());
};

// ============ 测试函数 ============

// 运行所有测试
const runAllTests = () => {
  console.log('======= 纯函数式 Monad 实现 =======\n');
  
  readerExample();
  writerExample();
  stateExample();
  combinedExample();
  
  console.log('\n======= 所有测试完成 =======');
  console.log('\nMonad总结:');
  console.log('1. Reader Monad: 用于读取共享环境，避免显式传递参数');
  console.log('2. Writer Monad: 用于收集日志和辅助输出，保持纯函数性');
  console.log('3. State Monad: 用于管理可变状态，提供确定性的状态转换');
  console.log('4. 所有实现都是纯函数式的，无副作用');
};

// 执行测试
runAllTests();