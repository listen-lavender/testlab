// 序对构造函数
const pair = a => b => f => f(a)(b);

// 访问第一个元素
const first = p => p(a => b => a);

// 访问第二个元素
const second = p => p(a => b => b);

// 创建单向链表节点（只包含值和后继节点）
const makeNode = value => next => pair(value)(next);

// 获取节点值
const getValue = node => first(node);

// 获取后继节点
const getNext = node => second(node);

// 设置节点值（可选，保持函数式风格）
const setValue = node => value => makeNode(value)(getNext(node));

// 设置后继节点
const setNext = node => next => makeNode(getValue(node))(next);

// 空链表
const emptyList = () => null;

// 判断链表是否为空
const isEmpty = head => head === null;

// 在头部插入节点（纯函数式版本）
const insertHead = head => value => 
  isEmpty(head)
    ? makeNode(value)(null)
    : makeNode(value)(head);

// 在尾部插入节点（递归实现）
const insertTail = head => value => {
  if (isEmpty(head)) {
    return makeNode(value)(null);
  }
  return makeNode(getValue(head))(insertTail(getNext(head))(value));
};

// 向前遍历（纯函数式递归版本）
const traverseForward = head => {
  const traverse = (node, result) => 
    isEmpty(node)
      ? result
      : traverse(getNext(node), [...result, getValue(node)]);
  
  return traverse(head, []);
};

// 获取链表长度
const length = head => {
  const count = (node, acc) => 
    isEmpty(node)
      ? acc
      : count(getNext(node), acc + 1);
  
  return count(head, 0);
};

// 查找节点值
const find = head => value => {
  const search = node => 
    isEmpty(node)
      ? false
      : getValue(node) === value || search(getNext(node));
  
  return search(head);
};

// 反转链表（纯函数式版本）
const reverse = head => {
  const reverseHelper = (current, reversed) => 
    isEmpty(current)
      ? reversed
      : reverseHelper(getNext(current), makeNode(getValue(current))(reversed));
  
  return reverseHelper(head, null);
};

// 删除头部节点
const deleteHead = head => 
  isEmpty(head)
    ? null
    : getNext(head);

// 删除尾部节点（递归实现）
const deleteTail = head => {
  if (isEmpty(head) || isEmpty(getNext(head))) {
    return null;
  }
  return makeNode(getValue(head))(deleteTail(getNext(head)));
};

// 测试
const test = () => {
  // 创建链表: 10 -> 20 -> 30 -> 40
  let list = emptyList();
  list = insertTail(list)(10);
  list = insertTail(list)(20);
  list = insertHead(list)(30);  // 在头部插入30
  list = insertTail(list)(40);
  
  console.log("原始链表遍历:", traverseForward(list));  // [30, 10, 20, 40]
  console.log("链表长度:", length(list));              // 4
  console.log("查找20:", find(list)(20));              // true
  console.log("查找50:", find(list)(50));              // false
  
  // 反转链表
  const reversedList = reverse(list);
  console.log("反转链表:", traverseForward(reversedList));  // [40, 20, 10, 30]
  
  // 删除操作
  const list2 = deleteHead(list);
  console.log("删除头部:", traverseForward(list2));  // [10, 20, 40]
  
  const list3 = deleteTail(list2);
  console.log("删除尾部:", traverseForward(list3));  // [10, 20]
  
  // 空链表测试
  console.log("空链表遍历:", traverseForward(emptyList()));  // []
  console.log("空链表长度:", length(emptyList()));           // 0
};

// 运行测试
test();