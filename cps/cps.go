package main

import (
	"errors"
	"fmt"
)

// https://blog.jcoglan.com/2011/03/05/translation-from-haskell-to-javascript-of-selected-portions-of-the-best-introduction-to-monads-ive-ever-read/

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

func iteration(n int, A func(int) int) func(int) int {
	if n == 0 {
		return A
	} else {
		// return func(x int) int {
		// 	return iteration(n-1, A)(x)
		// }
		return iteration(n-1, func(n_1 int) int {
			return n + A(n_1)
		})
	}
}

func initial(a int) int {
	return a + 1
}

func iteration2(n int, A func(int) int, first int) int {
	if n == 0 {
		return A(first)
	} else {
		// return func(x int) int {
		// 	return iteration(n-1, A)(x)
		// }
		return iteration2(n-1, func(n_1 int) int {
			return n + A(n_1)
		}, first)
	}
}

func unitErr(in int) (out int, err error) {
	return in, nil
}

func pureErr(in int) (out int) {
	out = out / in
	return out
}

func liftErr(pure func(int) int) func(int) (int, error) {
	return func(in int) (int, error) {
		out := pure(in)
		return unitErr(out)
	}
}

func appErr(in int) (out int, err error) {
	out = out / in
	return out, errors.New("divide error")
}

func bindErr(app func(int) (int, error)) func(int, error) (int, error) {
	return func(in int, inErr error) (int, error) {
		out, outErr := app(in)
		return out, fmt.Errorf("%w, %s", inErr, outErr.Error())
	}
}

func composeErr(f func(int, error) (int, error), g func(int, error) (int, error)) func(int, error) (int, error) {
	return func(in int, inErr error) (int, error) {
		out, outErr := f(in, inErr)
		return g(out, outErr)
	}
}

func pureErr1(a int) int {
	return a + 2
}

func pureErr2(a int) int {
	return a * 3
}

func Writer(in int) {

}

func unitWrite(in int) (out int, writer func(int)) {
	return in, func(prev int) {
		Writer(in + prev)
	}
}

func pureWrite(in int) (out int) {
	out = out / in
	return out
}

func liftWrite(pure func(int) int) func(int) (int, func(int)) {
	return func(in int) (int, func(int)) {
		out := pure(in)
		return unitWrite(out)
	}
}

func appWrite(in int) (out int, writer func(int)) {
	out = out / in
	return out, func(prev int) {
		writer(out + prev)
	}
}

func bindWrite(app func(int) (int, func(int))) func(int, func(int)) (int, func(int)) {
	return func(in int, inWriter func(int)) (int, func(int)) {
		out, outWriter := app(in)
		return out, func(prev int) {
			inWriter(in + prev)
			outWriter(out + prev)
		}
	}
}

func composeWrite(f func(int, func(int)) (int, func(int)), g func(int, func(int)) (int, func(int))) func(int, func(int)) (int, func(int)) {
	return func(in int, inWriter func(int)) (int, func(int)) {
		out, outWriter := f(in, inWriter)
		return g(out, outWriter)
	}
}

func pureWrite1(a int) int {
	return a + 2
}

func pureWrite2(a int) int {
	return a * 3
}

// func appRead(in int, env func() int) (out int) {
// 	out = in + env()
// 	return out
// }

// func Env() int {
// 	return 0
// }

// func bindRead(pure func(int, int) int) func(int) int {
// 	return func(in int) int {
// 		out := pure(in, Env())
// 		return out
// 	}
// }

// func pureRead1(a int, envVal int) int {
// 	return a + envVal
// }

// func pureRead2(a int, envVal int) int {
// 	return a * envVal
// }

func Reader() int {
	return 0
}

func unitRead(in int) (out int, reader func() int) {
	return in, func() int {
		return in + Reader()
	}
}

func pureRead(in int) (out int) {
	out = out / in
	return out
}

func liftRead(pure func(int) int) func(int) (int, func() int) {
	return func(in int) (int, func() int) {
		out := pure(in)
		return unitRead(out)
	}
}

func appRead(in int) (out int, reader func() int) {
	out = out / in
	return out, func() int {
		return out + Reader()
	}
}

func bindRead(app func(int) (int, func() int)) func(int, func() int) (int, func() int) {
	return func(in int, inReader func() int) (int, func() int) {
		out, outReader := app(in)
		return out, func() int {
			return out + outReader() + inReader()
		}
	}
}

func composeRead(f func(int, func() int) (int, func() int), g func(int, func() int) (int, func() int)) func(int, func() int) (int, func() int) {
	return func(in int, inReader func() int) (int, func() int) {
		out, outReader := f(in, inReader)
		return g(out, outReader)
	}
}

func pureRead1(a int) int {
	return a + 2
}

func pureRead2(a int) int {
	return a * 3
}

// function countUpCps() {
// 	var i=0;
// 	function nextStep(yieldIt) {
// 		yieldIt(i++, nextStep);
// 	}
// 	return new Generator(nextStep);
// }

// var g = countUpCps();
// g.next(function (result) {
// 	console.log(result);
// 	g.next(function (result) {
// 		console.log(result);
// 		// etc.
// 	});
// });

type Gen func(f func(result int, nextGen Gen))
type Generator struct {
	gen Gen
	j   int
}

func NewGenerator(gen Gen) *Generator {
	return &Generator{
		gen: gen,
	}
}

func (g *Generator) next(succeed func(int)) {
	g.gen(func(result int, nextGen Gen) {
		g.gen = nextGen
		succeed(result)
	})
}

func (g *Generator) Next(succeed func(int)) {
	g.j = g.j + 1
	succeed(g.j)
}

func NewCountGenerator() *Generator {
	var i = 0
	// function nextStep(yieldIt) {
	// 	yieldIt(i++, nextStep);
	// }
	var nextStep Gen
	nextStep = func(f func(result int, nextGen Gen)) {
		i = i + 1
		f(i, nextStep)
	}
	return NewGenerator(nextStep)
}

// function Generator(genFunc) {
// 	this._genFunc = genFunc;
// }
// Generator.prototype.next = function (success) {
// 	this._genFunc(function (result, nextGenFunc) {
// 		this._genFunc = nextGenFunc;
// 		success(result);
// 	});
// };

func main() {
	println("===========recursive")
	println(fact(4000000))
	println("===========cps")
	println(factCPS(4000000, last_continuation_cps))
	println("===========tail recursive")
	println(trampoline(factTail(4000000, nil)))
	// println(trampoline(factTail(4, nil)))
	println("===========iteration without initial data")
	println(iteration(5, initial)(0))
	println("===========iteration with initial data")
	println(iteration2(5, initial, 2))

	// composeErr(bindErr(liftErr(pureErr1)), bindErr(liftErr(pureErr2)))(unitErr(3))
	// composeWrite(bindWrite(liftWrite(pureWrite1)), bindWrite(liftWrite(pureWrite2)))(unitWrite(3))
	// composeRead(bindRead(liftRead(pureRead1)), bindRead(liftRead(pureRead2)))(unitRead(3))
	g := NewCountGenerator()

	g.next(func(result int) {
		println("=======1", result)
		g.next(func(result int) {
			println("=======2", result)
		})
		g.next(func(result int) {
			println("=======3", result)
			g.next(func(result int) {
				println("=======4", result)
			})
		})
	})
	g.next(func(result int) {
		println("=======5", result)
	})
	g.next(func(result int) {
		println("=======6", result)
	})

	g.Next(func(result int) {
		println("-------1", result)
		g.Next(func(result int) {
			println("-------2", result)
		})
		g.Next(func(result int) {
			println("-------3", result)
			g.Next(func(result int) {
				println("-------4", result)
			})
		})
	})
	g.Next(func(result int) {
		println("-------5", result)
	})
	g.Next(func(result int) {
		println("-------6", result)
	})
	return
}
