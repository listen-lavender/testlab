```
// ---------------------------
// 1. 定义 Update 类型
// ---------------------------
class Update {
  constructor(payload) {
    this.payload = payload; // 新状态或更新函数 (prevState => nextState)
    this.next = null;
  }
}

// ---------------------------
// 2. 组件基类
// ---------------------------
class Component {
  constructor(props) {
  
    this.props = props;
    this.state = {};
    this.__vnode = null;       // 上次渲染的虚拟 DOM
    this.__dom = null;         // 对应的真实 DOM 节点
    this.__updateQueue = null; // 更新队列（链表头）
    this.__lastUpdate = null;  // 队列尾
    this.__dirty = false;      // 是否需要重新渲染
  }

  setState(partialOrFn) {
    // 创建 Update 对象
    const update = new Update(partialOrFn);
    // 入队（简单链表）
    if (!this.__updateQueue) {
      this.__updateQueue = this.__lastUpdate = update;
    } else {
      this.__lastUpdate.next = update;
      this.__lastUpdate = update;
    }
    // 调度更新
    scheduleUpdateOnFiber(this);
  }

  // 更新组件状态（处理 updateQueue）
  __processUpdateQueue() {
    let update = this.__updateQueue;
    while (update) {
      const payload = update.payload;
      if (typeof payload === 'function') {
        this.state = payload(this.state);
      } else {
        this.state = { ...this.state, ...payload };
      }
      update = update.next;
    }
    this.__updateQueue = null;
    this.__lastUpdate = null;
  }

  // 渲染函数（子类必须重写）
  render() {
    throw new Error('render not implemented');
  }
}

// ---------------------------
// 3. 全局调度器
// ---------------------------
let isScheduled = false;
const dirtyComponents = new Set();

function scheduleUpdateOnFiber(component) {
  dirtyComponents.add(component);
  if (!isScheduled) {
    isScheduled = true;
    // 使用微任务实现异步批量更新
    Promise.resolve().then(performWork);
  }
}

function performWork() {
  // 将脏组件收集到数组中（避免遍历时增删）
  const components = [...dirtyComponents];
  dirtyComponents.clear();
  isScheduled = false;

  // 第一阶段：渲染阶段（Render Phase）
  // 对每个脏组件，处理更新队列并重新执行 render，生成新虚拟 DOM，并协调子组件
  for (const comp of components) {
    // 处理当前组件的更新队列，更新 state
    comp.__processUpdateQueue();
    // 执行渲染（返回新的虚拟 DOM）
    const newVNode = comp.render();
    // 获取旧虚拟 DOM
    const oldVNode = comp.__vnode;
    // 协调（reconcile）新旧虚拟 DOM，得到副作用列表（effect list）
    const effectList = reconcile(comp.__dom, oldVNode, newVNode, comp);
    // 记录新虚拟 DOM 以便下次使用
    comp.__vnode = newVNode;
    // 存储副作用（为简化，直接挂载到组件实例，提交阶段使用）
    comp.__effectList = effectList;
  }

  // 第二阶段：提交阶段（Commit Phase）
  // 对所有组件产生的副作用统一操作真实 DOM
  for (const comp of components) {
    if (comp.__effectList) {
      commitEffects(comp.__effectList);
      comp.__effectList = null;
    }
  }
}

// ---------------------------
// 4. 协调器（简化 Diff + 标记副作用）
// ---------------------------
// 副作用类型常量
const EFFECT_TYPES = {
  PLACEMENT: 1,  // 插入
  UPDATE: 2,     // 更新属性或子节点
  DELETION: 3,   // 删除
};

// 副作用节点结构
class Effect {
  constructor(type, node, parentDom, newVNode = null, oldVNode = null) {
    this.type = type;
    this.node = node;       // 真实 DOM 节点（对于 DELETION 是需要删除的节点，对于 PLACEMENT 是待插入的节点）
    this.parentDom = parentDom;
    this.newVNode = newVNode;
    this.oldVNode = oldVNode;
    this.next = null;
  }
}

function reconcile(parentDom, oldVNode, newVNode, component) {
  let firstEffect = null;
  let lastEffect = null;

  function addEffect(effect) {
    if (!firstEffect) firstEffect = effect;
    if (lastEffect) lastEffect.next = effect;
    lastEffect = effect;
  }

  function reconcileNode(parent, oldNode, newNode, idx) {
    if (!oldNode && !newNode) return null;
    if (oldNode && !newNode) {
      // 删除：从 DOM 树移除
      const domNode = oldNode.__dom;
      if (domNode) {
        addEffect(new Effect(EFFECT_TYPES.DELETION, domNode, parent, null, oldNode));
      }
      return null;
    }
    if (!oldNode && newNode) {
      // 插入：创建 DOM 并附加
      const domNode = createDomFromVNode(newNode);
      newNode.__dom = domNode;
      addEffect(new Effect(EFFECT_TYPES.PLACEMENT, domNode, parent, newNode, null));
      return domNode;
    }
    // 两者都存在，比较类型
    if (oldNode.type !== newNode.type) {
      // 类型不同：删除旧的，插入新的
      const oldDom = oldNode.__dom;
      if (oldDom) {
        addEffect(new Effect(EFFECT_TYPES.DELETION, oldDom, parent, null, oldNode));
      }
      const newDom = createDomFromVNode(newNode);
      newNode.__dom = newDom;
      addEffect(new Effect(EFFECT_TYPES.PLACEMENT, newDom, parent, newNode, null));
      return newDom;
    } else {
      // 类型相同：复用 DOM，更新属性
      const domNode = oldNode.__dom;
      newNode.__dom = domNode;
      // 属性更新（简化：只对比文本内容）
      if (typeof newNode.type === 'string') {
        if (oldNode.props?.children !== newNode.props?.children) {
          // 标记需要更新文本
          addEffect(new Effect(EFFECT_TYPES.UPDATE, domNode, parent, newNode, oldNode));
        }
      }
      // 递归协调子节点
      const oldChildren = oldNode.props?.children || [];
      const newChildren = newNode.props?.children || [];
      const maxLen = Math.max(oldChildren.length, newChildren.length);
      for (let i = 0; i < maxLen; i++) {
        const oldChild = oldChildren[i];
        const newChild = newChildren[i];
        reconcileNode(domNode, oldChild, newChild, i);
      }
      return domNode;
    }
  }

  // 开始协调根节点（parentDom 是组件挂载的真实容器）
  const rootDom = reconcileNode(parentDom, oldVNode, newVNode, 0);
  // 更新组件实例的真实 DOM 引用
  if (rootDom && component) component.__dom = rootDom;
  return firstEffect;
}

// ---------------------------
// 5. 虚拟 DOM 创建与真实 DOM 生成
// ---------------------------
function h(type, props, ...children) {
  // 简化：children 展平
  const flatChildren = children.flat(Infinity).filter(c => c != null && c !== false);
  return {
    type,
    props: { ...props, children: flatChildren },
    __dom: null,
  };
}

function createDomFromVNode(vnode) {
  if (vnode.type === undefined) return null;
  let dom;
  if (typeof vnode.type === 'string') {
    dom = document.createElement(vnode.type);
    // 设置属性（简化只处理文本）
    if (vnode.props && vnode.props.children) {
      const children = vnode.props.children;
      if (typeof children === 'string' || typeof children === 'number') {
        dom.innerText = children;
      } else if (Array.isArray(children)) {
        children.forEach(child => {
          if (typeof child === 'string' || typeof child === 'number') {
            dom.appendChild(document.createTextNode(child));
          } else if (child && child.type) {
            dom.appendChild(createDomFromVNode(child));
          }
        });
      }
    }
  } else if (typeof vnode.type === 'function') {
    // 如果是组件（类或函数），此处简化，实际会实例化或调用，我们的示例不使用组件嵌套，跳过
    dom = document.createComment('component placeholder');
  }
  return dom;
}

// ---------------------------
// 6. 提交阶段：执行真实 DOM 操作
// ---------------------------
function commitEffects(effectList) {
  let effect = effectList;
  while (effect) {
    const { type, node, parentDom, newVNode, oldVNode } = effect;
    switch (type) {
      case EFFECT_TYPES.PLACEMENT:
        parentDom.appendChild(node);
        break;
      case EFFECT_TYPES.DELETION:
        if (node.parentNode) node.parentNode.removeChild(node);
        break;
      case EFFECT_TYPES.UPDATE:
        // 更新文本内容
        if (newVNode && typeof newVNode.type === 'string') {
          const newText = newVNode.props?.children;
          if (typeof newText === 'string' || typeof newText === 'number') {
            node.innerText = newText;
          }
        }
        break;
    }
    effect = effect.next;
  }
}

// ---------------------------
// 7. 示例：一个简单的 Counter 组件
// ---------------------------
class Counter extends Component {
  constructor(props) {
    super(props);
    this.state = { count: 0 };
    // 初次渲染时，记录根 DOM 容器
    this.container = document.getElementById('app');
  }

  render() {
    // 返回虚拟 DOM
    return h('div', null,
      h('p', null, `Count: ${this.state.count}`),
      h('button', { onclick: () => this.setState({ count: this.state.count + 1 }) }, '+1'),
      h('button', { onclick: () => this.setState(prev => ({ count: prev.count - 1 })) }, '-1')
    );
  }
}

// ---------------------------
// 8. 初次渲染（手动执行）
// ---------------------------
const rootContainer = document.getElementById('app');
const app = new Counter();

// 初次渲染：生成虚拟 DOM，创建真实 DOM 并挂载
function initialRender(component, container) {
  const vnode = component.render();
  component.__vnode = vnode;
  const dom = createDomFromVNode(vnode);
  container.appendChild(dom);
  component.__dom = dom;
  // 绑定事件代理（简化：直接在真实 DOM 上监听）
  attachEventListeners(dom, vnode);
}

function attachEventListeners(dom, vnode) {
  if (!vnode.props) return;
  Object.keys(vnode.props).forEach(key => {
    if (key.startsWith('on') && typeof vnode.props[key] === 'function') {
      const eventType = key.slice(2).toLowerCase();
      dom.addEventListener(eventType, vnode.props[key]);
    }
  });
  // 递归子节点
  const children = vnode.props?.children || [];
  children.forEach((child, idx) => {
    if (child && child.__dom) {
      attachEventListeners(child.__dom, child);
    }
  });
}

initialRender(app, rootContainer);
```
在 React15 以前 React 的组件更新创建虚拟 DOM 和 Diff 的过程是不可中断，如果需要更新组件树层级非常深的话，在 Diff 的过程会非常占用浏览器的线程，而我们都知道浏览器执行JavaScript 的线程和渲染真实 DOM 的线程是互斥的，也就是同一时间内，浏览器要么在执行 JavaScript 的代码运算，要么在渲染页面，如果 JavaScript 的代码运行时间过长则会造成页面卡顿。
基于以上原因 React 团队在 React16 之后就改写了整个架构，将原来数组结构的虚拟DOM，改成叫 Fiber 的一种数据结构，基于这种 Fiber 的数据结构可以实现由原来不可中断的更新过程变成异步的可中断的更新。
在上面代码体现在哪里？


简介： 都说“双端对比算法”，那么双端对比算法，到底是怎么样的呢？跟 React 中的 Diff 算法又有什么不同呢？要了解这些，我们先了解 React 中的 Diff 算法，然后再了解 Vue3 中的 Diff 算法，最后讲一下 Vue2 中的 Diff 算法，才能去比较一下他们的区别。
前言
都说“双端对比算法”，那么双端对比算法，到底是怎么样的呢？跟 React 中的 Diff 算法又有什么不同呢？
要了解这些，我们先了解 React 中的 Diff 算法，然后再了解 Vue3 中的 Diff 算法，最后讲一下 Vue2 中的 Diff 算法，才能去比较一下他们的区别。
最后讲一下为什么 Vue 中不需要使用 Fiber 架构。
React 官方的解析
其实为什么 React 不采用 Vue 的双端对比算法，React 官方已经在源码的注释里已经说明了，我们来看一下 React 官方是怎么说的。
function reconcileChildrenArray(
returnFiber: Fiber,
currentFirstChild: Fiber | null,
newChildren: Array<*>,
expirationTime: ExpirationTime,
): Fiber | null {

// This algorithm can't optimize by searching from boths ends since we
// don't have backpointers on fibers. I'm trying to see how far we can get
// with that model. If it ends up not being worth the tradeoffs, we can
// add it later.

// Even with a two ended optimization, we'd want to optimize for the case
// where there are few changes and brute force the comparison instead of
// going for the Map. It'd like to explore hitting that path first in
// forward-only mode and only go for the Map once we notice that we need
// lots of look ahead. This doesn't handle reversal as well as two ended
// search but that's unusual. Besides, for the two ended optimization to
// work on Iterables, we'd need to copy the whole set.

// In this first iteration, we'll just live with hitting the bad case
// (adding everything to a Map) in for every insert/move.

// If you change this code, also update reconcileChildrenIterator() which
// uses the same algorithm.
｝

大概的意思就是说：
React 不能通过双端对比进行 Diff 算法优化是因为目前 Fiber 上没有设置反向链表，而且想知道就目前这种方案能持续多久，如果目前这种模式不理想的话，那么也可以增加双端对比算法。
即使是双端对比算法，我们也要对这种情况进行优化，我们应该使用 Map 这种数据结构方案去替代原来那种几乎没有什么变化也进行暴力比较的方案。它第一次搜索循环是通过 forward-only 这种模式（就是只从左向右查找），（第一次循环可能还没有结束，还有节点没有比对的时候）如果还要继续向前循环查找那么就要通过 Map 这种数据类型了。（就目前这个单向链表的数据结构，如果采用）双端对比查找算法比较难控制它反向查找的，但它确实是一种成功的算法。此外，双端对比算法的实现也在我们的工作迭代当中。
第一次迭代，我们就先将就使用这种不好的方案吧，每次新增/移动都要添加所有的数据到一个 Map 的数据类型对象中。
“we'd need to copy the whole set”，这一句，每一个单词都懂，但就是不知道他想说什么，所以就不翻译了，有知道的大神吗？
本人水平有限，错漏难免，如有错漏，恳请各位斧正。
React 的官方虽然解析了，但我们想要彻底理解到底为什么，还是要去详细了解 React 的 Diff 算法是怎么样的。在了解 React Diff 算法之前，我们首先要了解什么是 Fiber，为什么 React 中要使用 Fiber？
Fiber 的结构
在 React15 以前 React 的组件更新创建虚拟 DOM 和 Diff 的过程是不可中断，如果需要更新组件树层级非常深的话，在 Diff 的过程会非常占用浏览器的线程，而我们都知道浏览器执行JavaScript 的线程和渲染真实 DOM 的线程是互斥的，也就是同一时间内，浏览器要么在执行 JavaScript 的代码运算，要么在渲染页面，如果 JavaScript 的代码运行时间过长则会造成页面卡顿。
基于以上原因 React 团队在 React16 之后就改写了整个架构，将原来数组结构的虚拟DOM，改成叫 Fiber 的一种数据结构，基于这种 Fiber 的数据结构可以实现由原来不可中断的更新过程变成异步的可中断的更新。
Fiber 的数据结构主要长成以下的样子，主要通过 Fiber 的一些属性去保存组件相关的信息。
function FiberNode(
tag: WorkTag,
pendingProps: mixed,
key: null | string,
mode: TypeOfMode,
) {
// 作为静态数据结构的属性
this.tag = tag;
this.key = key;
this.elementType = null;
this.type = null;
this.stateNode = null;

// 用于连接其他Fiber节点形成Fiber树
this.return = null;
this.child = null;
this.sibling = null;
this.index = 0;

this.ref = null;

// 作为动态的工作单元的属性
this.pendingProps = pendingProps;
this.memoizedProps = null;
this.updateQueue = null;
this.memoizedState = null;
this.dependencies = null;

this.mode = mode;

this.effectTag = NoEffect;
this.nextEffect = null;

this.firstEffect = null;
this.lastEffect = null;

// 调度优先级相关
this.lanes = NoLanes;
this.childLanes = NoLanes;

// 指向该fiber在另一次更新时对应的fiber
this.alternate = null;
}

Fiber 主要靠以下属性连成一棵树结构的数据的，也就是 Fiber 链表。
// 指向父级Fiber节点
this.return = null;
// 指向子Fiber节点
this.child = null;
// 指向右边第一个兄弟Fiber节点
this.sibling = null;

举个例子，如下的组件结构：
function App() {
return (

<div>
  i am
  <span>Coboy</span>
</div>
)
}

对应的 Fiber 链表结构：

那么以上的 Fiber 链表的数据结构有什么特点，就是任何一个位置的 Fiber 节点，都可以非常容易知道它的父 Fiber, 第一个子元素的 Fiber,和它的兄弟节点 Fiber。却不容易知道它前一个 Fiber 节点是谁，这就是 React 中单向链表 Fiber 节点的特点。也正是因为这些即便在协调的过程被中断了，再恢复协调的时候，依然知道当前的 父节点和孩子节点等信息。
那么 React 是将对应组件怎么生成一个 Fiber 链表数据的呢？
Fiber 链表的生成
上面的组件在经过 JSX 的编译之后，初始化的时候会生成成一个类似于 React 15 或者 Vue 那种虚拟 DOM 的数据结构。然后创建一个叫 fiberRoot 的 Fiber 节点，然后开始从 fiberRoot 这个根 Fiber 开始进行协调，生成一棵 Fiber 树，这个棵树被称为：workInProgress Fiber 树 ，意思是正在工作的 Fiber 树，接下来我们详细了解一下具体是怎么生成一棵 Fiber 树的。要先了解 Fiber 树的生成原理才更好去理解 Fiber 树 diff 的过程。
以下是一段简单版的 Fiber 链表生成的代码片段。
这个协调子节点的函数接收两个参数，returnFiber 是父 Fiber，children 是这个节点的子元素的虚拟 DOM数据。
// 这个协调子节点的函数接收两个参数，returnFiber 是父 Fiber，children 是这个节点的子元素的虚拟 DOM数据。
export function reconcileChildren(returnFiber, children) {

// 如果是字符串或者数字则不创建 Fiber
if(isStringOrNumber(children)) {
    return
}
const newChildren = isArray(children) ? children : [children]
// 上一轮的 fiber 节点
let previousNewFiber = null;
// 初次渲染（false）还是更新（true）
let shouldTrackSideEffects = !!returnFiber.alternate
// 老 Fiber 节点
let oldFiber = returnFiber.alternate && returnFiber.alternate.child
let nextOldFiber = null
// 上一次协调返回的位置
let lastPlacedIndex = 0;
// 记录每个 fiber 节点的位置
let newIdx = 0;
// 如果不存在老 Fiber 则是初始化的过程，进行 Fiber 链表的创建
if(!oldFiber) {
    for (; newIdx < newChildren.length; newIdx++) {
        // 获取节点元素内容
        const newChild = newChildren[newIdx]
        // 如果节点为 null 则不需要创建 fiber 节点
        if(newChild === null) {
            continue
        }
        // 创建新 fiber 的时候记录了关键的父 fiber 等重要信息
        const newFiber = createFiber(newChild, returnFiber)
        // 记录当前每一个 fiber 的位置
        lastPlacedIndex = placeChild(
            newFiber,
            lastPlacedIndex,
            newIdx,
            shouldTrackSideEffects // 初次渲染（false）还是更新（true）
        )
        // 当上一轮的 fiber 节点为 null 的时候，这一轮的 fiber 就是头节点
        if(previousNewFiber === null) {
            // 父 fiber 的 child 就是第一个节点
            returnFiber.child = newFiber
        } else {
            // 如果不是第一个节点，那么就是兄弟节点
            // 上一轮 fiber 的兄弟节点是这一轮的 fiber 节点
            previousNewFiber.sibling = newFiber;
        }
       // 记录上一轮的 fiber，既是这一轮的 fiber 便是下一轮的上一轮 fiber
        previousNewFiber = newFiber
    }
    return
}
}

构建完的 workInProgress Fiber树 会在 commit阶段 渲染到页面。
在组件状态数据发生变更的时候，会根据最新的状态数据先会生成新的虚拟DOM，再去构建一棵新的 workInProgress Fiber 树 ，而在重新协调构建新的 Fiber 树的过程也就是 React Diff 发生的地方。接下来，我们就看看 React Diff 算法是怎么样的。

