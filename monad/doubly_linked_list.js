// 序对构造函数
const pair = a => b => f => f(a)(b);

// 访问第一个元素
const first = p => p(a => b => a);

// 访问第二个元素
const second = p => p(a => b => b);

// 创建链表节点（包含前驱节点、值和后继节点）
const makeNode = prev => value => next => 
  pair(prev)(pair(value)(next));

// 获取前驱节点
const getPrev = node => first(node);

// 获取节点值
const getValue = node => first(second(node));

// 获取后继节点
const getNext = node => second(second(node));

// 设置前驱节点
const setPrev = node => prev => makeNode(prev)(getValue(node))(getNext(node));

// 设置后继节点
const setNext = node => next => makeNode(getPrev(node))(getValue(node))(next);

// 空链表
const emptyList = () => null;

// 在头部插入节点（纯函数式版本）
const insertHead = list => value => 
  list === null 
    ? makeNode(null)(value)(null)
    : (() => {
        const newHead = makeNode(null)(value)(list);
        const updatedSecond = setPrev(list)(newHead);
        return setNext(newHead)(updatedSecond);
      })();

// 在尾部插入节点（纯函数式递归版本）
const insertTail = list => value => {
  const insertRecursive = node => {
    if (node === null) {
      return makeNode(null)(value)(null);
    }
    
    if (getNext(node) === null) {
      const newNode = makeNode(node)(value)(null);
      const updatedNode = setNext(node)(newNode);
      return updatedNode;
    }
    
    const updatedNext = insertRecursive(getNext(node));
    return setNext(node)(updatedNext);
  };
  
  return insertRecursive(list);
};

// 向前遍历（纯函数式递归版本）
const traverseForward = list => {
  const traverse = (node, result) => 
    node === null 
      ? result
      : traverse(getNext(node), [...result, getValue(node)]);
  
  return traverse(list, []);
};

// 向后遍历（纯函数式版本）
const traverseBackward = list => {
  const findLast = node => 
    getNext(node) === null ? node : findLast(getNext(node));
  
  const traverseBackwardFrom = (node, result) => 
    node === null
      ? result
      : traverseBackwardFrom(getPrev(node), [...result, getValue(node)]);
  
  return list === null 
    ? [] 
    : traverseBackwardFrom(findLast(list), []);
};

// 删除头节点
const deleteHead = list => 
  list === null 
    ? null
    : (() => {
        const nextNode = getNext(list);
        return nextNode === null 
          ? null 
          : setPrev(nextNode)(null);
      })();

// 删除尾节点
const deleteTail = list => {
  const findLast = node => 
    getNext(node) === null ? node : findLast(getNext(node));
  
  const deleteLast = node => {
    if (node === null) return null;
    
    if (getNext(node) === null) {
      const prevNode = getPrev(node);
      return prevNode === null 
        ? null 
        : setNext(prevNode)(null);
    }
    
    const updatedNext = deleteLast(getNext(node));
    return setNext(node)(updatedNext);
  };
  
  return deleteLast(list);
};

// 查找节点（按值）
const findNode = list => value => {
  const find = node => 
    node === null 
      ? null
      : getValue(node) === value 
        ? node 
        : find(getNext(node));
  
  return find(list);
};

// 获取链表长度
const length = list => {
  const count = (node, acc) => 
    node === null ? acc : count(getNext(node), acc + 1);
  
  return count(list, 0);
};

// 反转链表（纯函数式）
const reverse = list => {
  const reverseHelper = (current, prev) => 
    current === null
      ? prev
      : (() => {
          const nextNode = getNext(current);
          const updatedCurrent = setPrev(current)(nextNode);
          const updatedCurrent2 = setNext(updatedCurrent)(prev);
          return reverseHelper(nextNode, updatedCurrent2);
        })();
  
  return reverseHelper(list, null);
};

// 链表的函数式组合器
const map = list => fn => {
  const mapRecursive = node => 
    node === null 
      ? null 
      : (() => {
          const mappedNode = makeNode(getPrev(node))(fn(getValue(node)))(null);
          const mappedNext = mapRecursive(getNext(node));
          const updatedMappedNext = mappedNext === null 
            ? mappedNode 
            : setPrev(mappedNext)(mappedNode);
          return setNext(mappedNode)(updatedMappedNext);
        })();
  
  return mapRecursive(list);
};

// 测试
const test = () => {
  let list = emptyList();
  list = insertHead(list)(10);
  list = insertHead(list)(20);
  list = insertTail(list)(30);
  list = insertTail(list)(40);
  
  console.log("向前遍历:", traverseForward(list));  // [20, 10, 30, 40]
  console.log("向后遍历:", traverseBackward(list)); // [40, 30, 10, 20]
  console.log("链表长度:", length(list));          // 4
  
  const found = findNode(list)(30);
  console.log("查找30:", found ? getValue(found) : "未找到"); // 30
  
  list = deleteHead(list);
  console.log("删除头后向前遍历:", traverseForward(list)); // [10, 30, 40]
  
  list = deleteTail(list);
  console.log("删除尾后向前遍历:", traverseForward(list)); // [10, 30]
  
  const doubledList = map(list)(x => x * 2);
  console.log("映射加倍后:", traverseForward(doubledList)); // [20, 60]
  
  const reversedList = reverse(list);
  console.log("反转后:", traverseForward(reversedList)); // [30, 10]
};

test();