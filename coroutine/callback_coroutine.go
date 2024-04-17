package main
// 顶层task是callback，会不断产生新的callback到任务列表

import (
	"time"
)

// 4
// 2   6
// 1 3 5 7

var tasks []*Task

func eventLoop() {
	for {
		for _, task := range tasks {
			if task.state == true {
				task.f()
				task.state = false
			}
		}
		time.Sleep(2 * time.Second)
	}
}

func traverse(node *TreeNode) {
	if node == nil {
		return
	}
	addTask(func() {
		traverse(node.left)
	})
	println(node.v)
	addTask(func() {
		traverse(node.right)
	})
}

func addTask(t func()) {
	tasks = append(tasks, &Task{
		f:     t,
		state: true,
	})
}

type Task struct {
	f     func()
	state bool
}

type TreeNode struct {
	v     int
	left  *TreeNode
	right *TreeNode
}

func main() {
	node := &TreeNode{
		v: 4,
		left: &TreeNode{
			v: 2,
			left: &TreeNode{
				v: 1,
			},
			right: &TreeNode{
				v: 3,
			},
		},
		right: &TreeNode{
			v: 6,
			left: &TreeNode{
				v: 5,
			},
			right: &TreeNode{
				v: 7,
			},
		},
	}
	addTask(func() {
		traverse(node)
	})
	// eventLoop()
}

