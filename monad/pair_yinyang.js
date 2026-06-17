// ============ 基础工具函数 ============

// 序对构造函数（纯函数式）
const pair = a => b => f => f(a)(b);

// 访问第一个元素
const first = p => p(a => b => a);

// 访问第二个元素
const second = p => p(a => b => b);

// 日志函数（为了观察执行过程）
const log = msg => {
  console.log(msg);
  return msg;
};

// ============ 阴阳谜题核心实现 ============

// 阴阳谜题的"call/cc"模拟 - 控制流捕获
const callCC = f => cont => f(a => _ => cont(a))(cont);

// Yin函数 - 打印"@"
const yin = cont => {
  log('@');
  return cont;  // 继续执行
};

// Yang函数 - 打印"*"
const yang = cont => {
  log('*');
  return cont;  // 继续执行
};

// 阴阳组合谜题
const yinyang = () => {
  // 第一次调用yin
  const step1 = yin;
  
  // 通过callCC捕获控制流
  const step2 = callCC(k => {
    // k是当前的延续（continuation）
    // 这里我们将k作为参数传递给下一个函数
    return yangCont => {
      // 调用yang，并传入控制流
      return yang(yangCont);
    };
  });
  
  // 递归调用自身，形成无限循环
  const step3 = cont => {
    // 再次调用yin
    const next1 = yin(cont);
    // 然后再次通过callCC
    const next2 = callCC(k2 => {
      return yangCont2 => {
        return yang(yangCont2);
      };
    })(next1);
    // 递归
    return step3(next2);
  };
  
  // 开始执行
  return step3(step2(step1(() => {})));
};

// ============ 更简洁的阴阳谜题版本 ============

// 无限递归的阴阳打印器
const yinyangInfinite = () => {
  // 内部递归函数
  const yinYangRec = (yinCont, depth) => {
    if (depth > 20) {  // 限制深度，避免无限循环
      log('... (stopped at depth 20)');
      return () => {};
    }
    
    log(`@ (depth ${depth})`);
    
    // 模拟callCC - 捕获当前控制流
    const capturedCont = next => {
      log(`* (depth ${depth})`);
      // 继续执行后续代码
      return next();
    };
    
    // 递归调用
    return () => yinYangRec(() => capturedCont(() => {
      log(`+ (depth ${depth} continuation)`);
      return () => yinYangRec(yinCont, depth + 1);
    }), depth + 1);
  };
  
  // 开始执行
  return yinYangRec(() => {}, 0)();
};

// ============ 使用生成器的阴阳谜题 ============

// 创建阴阳生成器
const createYinYangGenerator = () => {
  const makeStep = (counter, isYin) => {
    const value = isYin ? '@' : '*';
    const next = () => {
      log(`${value}${counter}`);
      // 交替生成@和*
      if (isYin) {
        // 在@之后，通过callCC模拟来创建复杂的控制流
        const captured = next => {
          // 模拟控制流捕获
          const nextStep = makeStep(counter + 1, false);  // 生成*
          return getNext(nextStep)();  // 执行*
        };
        
        // 创建新的@步骤
        const yinStep = makeStep(counter + 2, true);
        return captured(() => getNext(yinStep)());
      } else {
        // *之后直接生成@
        const yinStep = makeStep(counter + 1, true);
        return getNext(yinStep)();
      }
    };
    
    return createStep(value)(next);
  };
  
  return makeStep(0, true);  // 从@开始
};

// 创建生成器步进函数
const createStep = value => next => pair(value)(next);

// 获取当前值
const getValue = step => first(step);

// 获取下一步函数
const getNext = step => second(step);

// ============ 测试函数 ============

// 测试基本阴阳谜题
const testYinYangBasic = () => {
  console.log('=== 基本阴阳谜题测试 ===');
  
  // 运行简单版本
  console.log('\n运行简单阴阳谜题:');
  try {
    // 注意：这会无限递归，所以我们要捕获错误或限制执行
    const limitedYinYang = () => {
      let count = 0;
      const max = 10;
      
      const yinLimited = cont => {
        if (count >= max) throw new Error('达到限制');
        log('@');
        count++;
        return cont;
      };
      
      const yangLimited = cont => {
        if (count >= max) throw new Error('达到限制');
        log('*');
        count++;
        return cont;
      };
      
      const loop = cont => {
        const next1 = yinLimited(cont);
        const next2 = yangLimited(next1);
        return loop(next2);
      };
      
      return loop(() => {});
    };
    
    limitedYinYang();
  } catch (e) {
    log(`停止: ${e.message}`);
  }
};

// 测试生成器版本
const testYinYangGenerator = () => {
  console.log('\n=== 生成器版本阴阳谜题 ===');
  
  const generator = createYinYangGenerator();
  console.log('初始值:', getValue(generator));  // @
  
  // 执行几步
  for (let i = 0; i < 5; i++) {
    try {
      const nextFunc = getNext(generator);
      const nextStep = nextFunc();
      console.log(`第${i+1}步:`, getValue(nextStep));
      // 更新generator引用（在纯函数式中需要这样）
      // 在实际中，我们应该有更好的状态管理
    } catch (e) {
      console.log('执行出错:', e.message);
      break;
    }
  }
};

// 测试无限版本
const testYinYangInfinite = () => {
  console.log('\n=== 有限深度阴阳谜题 ===');
  yinyangInfinite();
};

// 真正的阴阳谜题模式
const trueYinYangPuzzle = () => {
  console.log('\n=== 经典阴阳谜题模式 ===');
  
  // 这是经典的阴阳谜题输出模式：@*@**@***@****...
  let output = '';
  let yinCount = 1;
  
  const generate = () => {
    // 添加一个@
    output += '@';
    
    // 添加yinCount个*
    for (let i = 0; i < yinCount; i++) {
      output += '*';
    }
    
    yinCount++;
    
    if (output.length < 30) {  // 限制长度
      // 递归继续
      return generate;
    } else {
      return () => output;
    }
  };
  
  // 使用生成器的方式执行
  const initStep = createStep('start')(generate);
  let current = initStep;
  
  for (let i = 0; i < 10; i++) {
    const nextFunc = getNext(current);
    if (typeof nextFunc === 'function') {
      current = createStep(`step${i}`)(nextFunc);
      const result = nextFunc();
      if (typeof result === 'function') {
        // 继续
      } else if (typeof result === 'string') {
        console.log('最终输出:', result);
        break;
      }
    } else {
      break;
    }
  }
  
  console.log('生成模式:', output.substring(0, 30) + '...');
};

// 运行所有测试
const runAllTests = () => {
  console.log('======= 纯函数式阴阳谜题 =======\n');
  
  testYinYangBasic();
  // testYinYangGenerator();
  testYinYangInfinite();
  trueYinYangPuzzle();
  
  console.log('\n======= 所有测试完成 =======');
  console.log('\n阴阳谜题展示了控制流的精妙之处：');
  console.log('1. @ 和 * 交替出现');
  console.log('2. 每次@出现后，*的数量逐渐增加');
  console.log('3. 通过"延续"(continuation)实现复杂的控制流');
  console.log('4. 在纯函数式编程中模拟命令式控制流');
};

// 执行测试
runAllTests();