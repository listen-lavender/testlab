package main

type A interface {
    Print()
    Global() int
    Function(int) int
}

type B interface {
    Print()
    Global() int
}

type c struct{}

func (c *c) Print()           {}
func (c *c) Global() int      { return 0 }
func (c *c) Function(int) int { return 0 }

func Covariance(a A) B {
    return a
}

// func Contravariance(b B) A {
//  return b
// }
