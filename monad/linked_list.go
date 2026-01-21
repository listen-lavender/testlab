package main

import (
	"fmt"
)

// 定义函数类型，模仿JavaScript的函数式风格
type PairFunc func(func(interface{}) func(interface{}) interface{}) interface{}

// 序对构造函数 - 使用闭包实现
func pair(a interface{}) func(interface{}) PairFunc {
	return func(b interface{}) PairFunc {
		return func(f func(interface{}) func(interface{}) interface{}) interface{} {
			return f(a)(b)
		}
	}
}

// 访问第一个元素
func first(p PairFunc) interface{} {
	return p(func(a interface{}) func(interface{}) interface{} {
		return func(b interface{}) interface{} {
			return a
		}
	})
}

// 访问第二个元素
func second(p PairFunc) interface{} {
	return p(func(a interface{}) func(interface{}) interface{} {
		return func(b interface{}) interface{} {
			return b
		}
	})
}

// 定义链表类型
type List interface{}

// 创建链表节点
func makeNode(value interface{}) func(List) PairFunc {
	return func(next List) PairFunc {
		return pair(value)(next)
	}
}

// 获取节点值
func getValue(node List) interface{} {
	if node == nil {
		return nil
	}
	return first(node.(PairFunc))
}

// 获取后继节点
func getNext(node List) List {
	if node == nil {
		return nil
	}
	return second(node.(PairFunc))
}

// 判断链表是否为空
func isEmpty(list List) bool {
	return list == nil
}

// 在头部插入节点（纯函数式）
func insertHead(list List, value interface{}) List {
	if isEmpty(list) {
		return makeNode(value)(nil)
	}
	return makeNode(value)(list)
}

// 在尾部插入节点（递归实现）
func insertTail(list List, value interface{}) List {
	if isEmpty(list) {
		return makeNode(value)(nil)
	}
	currentValue := getValue(list)
	next := getNext(list)
	return makeNode(currentValue)(insertTail(next, value))
}

// 遍历链表
func traverseForward(list List) []interface{} {
	var traverse func(node List, result []interface{}) []interface{}

	traverse = func(node List, result []interface{}) []interface{} {
		if isEmpty(node) {
			return result
		}
		return traverse(getNext(node), append(result, getValue(node)))
	}

	return traverse(list, []interface{}{})
}

// 获取链表长度
func length(list List) int {
	var count func(node List, acc int) int

	count = func(node List, acc int) int {
		if isEmpty(node) {
			return acc
		}
		return count(getNext(node), acc+1)
	}

	return count(list, 0)
}

// 查找节点值
func find(list List, value interface{}) bool {
	var search func(node List) bool

	search = func(node List) bool {
		if isEmpty(node) {
			return false
		}
		if getValue(node) == value {
			return true
		}
		return search(getNext(node))
	}

	return search(list)
}

// 反转链表（纯函数式）
func reverse(list List) List {
	var reverseHelper func(current List, reversed List) List

	reverseHelper = func(current List, reversed List) List {
		if isEmpty(current) {
			return reversed
		}
		value := getValue(current)
		next := getNext(current)
		return reverseHelper(next, makeNode(value)(reversed))
	}

	return reverseHelper(list, nil)
}

// 删除头部节点
func deleteHead(list List) List {
	if isEmpty(list) {
		return nil
	}
	return getNext(list)
}

// 删除尾部节点（递归实现）
func deleteTail(list List) List {
	if isEmpty(list) || isEmpty(getNext(list)) {
		return nil
	}
	currentValue := getValue(list)
	return makeNode(currentValue)(deleteTail(getNext(list)))
}

// 将值转换为整数（辅助函数）
func toInt(value interface{}) int {
	switch v := value.(type) {
	case int:
		return v
	default:
		return 0
	}
}

// 映射操作：对链表中每个元素应用函数
func mapList(list List, f func(interface{}) interface{}) List {
	var mapHelper func(node List) List

	mapHelper = func(node List) List {
		if isEmpty(node) {
			return nil
		}
		value := getValue(node)
		next := getNext(node)
		return makeNode(f(value))(mapHelper(next))
	}

	return mapHelper(list)
}

// 过滤操作：过滤链表中满足条件的元素
func filterList(list List, predicate func(interface{}) bool) List {
	var filterHelper func(node List) List

	filterHelper = func(node List) List {
		if isEmpty(node) {
			return nil
		}
		value := getValue(node)
		next := getNext(node)
		if predicate(value) {
			return makeNode(value)(filterHelper(next))
		}
		return filterHelper(next)
	}

	return filterHelper(list)
}

// 归约操作：将链表归约为单个值
func reduceList(list List, initial interface{}, reducer func(interface{}, interface{}) interface{}) interface{} {
	var reduceHelper func(node List, acc interface{}) interface{}

	reduceHelper = func(node List, acc interface{}) interface{} {
		if isEmpty(node) {
			return acc
		}
		value := getValue(node)
		next := getNext(node)
		return reduceHelper(next, reducer(acc, value))
	}

	return reduceHelper(list, initial)
}

// 链表拼接
func concatLists(list1 List, list2 List) List {
	if isEmpty(list1) {
		return list2
	}
	currentValue := getValue(list1)
	next := getNext(list1)
	return makeNode(currentValue)(concatLists(next, list2))
}

// 测试函数
func test() {
	fmt.Println("=== Go语言纯函数式单向链表测试 ===")

	// 创建链表: 10 -> 20 -> 30 -> 40
	var list List = nil
	list = insertTail(list, 10)
	list = insertTail(list, 20)
	list = insertHead(list, 30) // 在头部插入30
	list = insertTail(list, 40)

	fmt.Printf("原始链表遍历: %v\n", traverseForward(list)) // [30 10 20 40]
	fmt.Printf("链表长度: %d\n", length(list))            // 4
	fmt.Printf("查找20: %v\n", find(list, 20))          // true
	fmt.Printf("查找50: %v\n", find(list, 50))          // false

	// 反转链表
	reversedList := reverse(list)
	fmt.Printf("反转链表: %v\n", traverseForward(reversedList)) // [40 20 10 30]

	// 删除操作
	list2 := deleteHead(list)
	fmt.Printf("删除头部: %v\n", traverseForward(list2)) // [10 20 40]

	list3 := deleteTail(list2)
	fmt.Printf("删除尾部: %v\n", traverseForward(list3)) // [10 20]

	// 空链表测试
	fmt.Printf("空链表遍历: %v\n", traverseForward(nil)) // []
	fmt.Printf("空链表长度: %d\n", length(nil))          // 0

	// 测试映射操作
	fmt.Println("\n=== 高阶函数测试 ===")

	// 将链表中的每个数字乘以2
	multiplyByTwo := func(x interface{}) interface{} {
		return toInt(x) * 2
	}
	mappedList := mapList(list, multiplyByTwo)
	fmt.Printf("映射操作（每个元素乘以2）: %v\n", traverseForward(mappedList))

	// 过滤大于15的元素
	greaterThan15 := func(x interface{}) bool {
		return toInt(x) > 15
	}
	filteredList := filterList(list, greaterThan15)
	fmt.Printf("过滤操作（大于15的元素）: %v\n", traverseForward(filteredList))

	// 求和
	sumReducer := func(acc interface{}, val interface{}) interface{} {
		return toInt(acc) + toInt(val)
	}
	sum := reduceList(list, 0, sumReducer)
	fmt.Printf("归约操作（求和）: %v\n", sum)

	// 链表拼接
	var listA List = nil
	listA = insertTail(listA, 1)
	listA = insertTail(listA, 2)

	var listB List = nil
	listB = insertTail(listB, 3)
	listB = insertTail(listB, 4)

	concatenated := concatLists(listA, listB)
	fmt.Printf("链表拼接: %v\n", traverseForward(concatenated))

	fmt.Println("\n=== 函数式编程特性演示 ===")
	// 链式操作示例
	result := traverseForward(
		filterList(
			mapList(list, multiplyByTwo),
			func(x interface{}) bool { return toInt(x) > 50 },
		),
	)
	fmt.Printf("链式操作（映射后过滤）: %v\n", result)
}

func main() {
	test()
}
