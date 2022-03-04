package main

// 递归求和
func fact(n int) int {
	if n == 1 {
		return 1
	}
	return fact(n-1) + n
}

// cps变换
// last_continuation_cps 对应函数式中的id函数
func last_continuation_cps(res int) int {
	return res
	// return init_continuation_cps(res)
}

func factCPS(n int, continuation func(int) int) int {
	if n == 1 {
		return continuation(1)
	}
	n_1continuation := func(n_1res int) int {
		return continuation(n_1res + n)
	}
	return factCPS(n-1, n_1continuation)
}

// 尾递归优化
type Wrap struct {
	continuation Continuation
	res          int
}

type Continuation func(int) *Wrap

func last_continuation_tail(res int) *Wrap {
	return &Wrap{
		continuation: nil,
		res:          res,
	}
}

func factTail(n int, continuation Continuation) *Wrap {
	if n == 1 {
		return &Wrap{
			continuation: continuation,
			res:          1,
		}
	} else {
		n_1continuation := func(res int) *Wrap {
			return factTail(n-1, func(n_1res int) *Wrap {
				// println("------", n)
				return &Wrap{
					continuation: continuation,
					res:          n_1res + n,
				}
			})
		}
		return &Wrap{
			continuation: n_1continuation,
			res:          0,
		}
	}
}

// 尾递归链表遍历
func trampoline(wrap *Wrap) int {
	for {
		if wrap == nil {
			return 0
		}
		if wrap.continuation != nil {
			// println("=====", wrap.res)
			wrap = wrap.continuation(wrap.res)
		} else {
			break
		}
	}
	return wrap.res
}

func main() {
	println("===========recursive")
	println(fact(4000000))
	println("===========cps")
	println(factCPS(4000000, last_continuation_cps))
	println("===========tail recursive")
	println(trampoline(factTail(4000000, nil)))
	// println(trampoline(factTail(4, nil)))
	return
}
