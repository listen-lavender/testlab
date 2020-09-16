package main

import "time"

type HandlePack struct {
	resolve func(result interface{}) (interface{}, *Error)
	reject  func(err *Error) *Error
	result  interface{}
	err     *Error
}

// Error .
type Error struct {
	Code   int
	Reason string
}

// Promise .
type Promise struct {
	Handlers  map[int]*HandlePack
	FuncIndex int
	DataIndex int
}

// NewPromise .
func NewPromise() *Promise {
	promise := Promise{
		Handlers: make(map[int]*HandlePack),
	}
	return &promise
}

// Await .
func (promise *Promise) Await() (interface{}, *Error) {
	return nil, nil
}

// Then .
func (promise *Promise) Then(resolve func(result interface{}) (interface{}, *Error), reject func(err *Error) *Error) *Promise {
	handlePack, ok := promise.Handlers[promise.FuncIndex]
	if !ok {
		handlePack = &HandlePack{resolve: resolve, reject: reject}
		promise.Handlers[promise.FuncIndex] = handlePack
		promise.FuncIndex = promise.FuncIndex + 1
		return promise
	}
	handlePack.resolve = resolve
	handlePack.reject = reject
	if handlePack.err != nil {
		err := handlePack.reject(handlePack.err)
		if err != nil {
			promise.Reject(promise.FuncIndex+1, err)
		}
	} else if handlePack.result != nil {
		result, err := handlePack.resolve(handlePack.result)
		if err != nil {
			promise.Reject(promise.FuncIndex+1, err)
		} else if result != nil {
			promise.Resolve(promise.FuncIndex+1, result)
		}
	}
	return promise
}

// Resolve .
func (promise *Promise) Resolve(index int, result interface{}) {
	handlePack, ok := promise.Handlers[index]
	promise.DataIndex = index
	if ok {
		handlePack.result = result
		result, err := handlePack.resolve(handlePack.result)
		if err != nil {
			go func() {
				promise.Reject(index+1, err)
			}()
		} else if result != nil {
			go func() {
				promise.Resolve(index+1, result)
			}()
		}
	} else {
		promise.Handlers[index] = &HandlePack{
			result: result,
		}
	}
}

// Reject .
func (promise *Promise) Reject(index int, err *Error) {
	handlePack, ok := promise.Handlers[index]
	promise.DataIndex = index
	if ok {
		handlePack.err = err
		err := handlePack.reject(handlePack.err)
		if err != nil {
			go func() {
				promise.Reject(index+1, err)
			}()
		}
	} else {
		promise.Handlers[index] = &HandlePack{
			err: err,
		}
	}
}

func main() {
	NewPromise().Then(func(result interface{}) (interface{}, *Error) {
		println("====0", result.(int))
		return "abc", nil
	}, func(err *Error) *Error {
		if err != nil {
			println("======--0", err.Code, err.Reason)
		}
		return err
	}).Then(func(result interface{}) (interface{}, *Error) {
		println("====1", result.(string))
		return 1.2, nil
	}, func(err *Error) *Error {
		if err != nil {
			println("======--1", err.Code, err.Reason)
		}
		return err
	}).Then(func(result interface{}) (interface{}, *Error) {
		println("====2", result.(float64))
		return nil, &Error{
			Code:   1,
			Reason: "hkhk",
		}
	}, func(err *Error) *Error {
		if err != nil {
			println("======--2", err.Code, err.Reason)
		}
		return err
	}).Then(func(result interface{}) (interface{}, *Error) {
		println("====3", result.(float64))
		return map[string]string{
			"a": "1",
			"b": "2",
		}, nil
	}, func(err *Error) *Error {
		if err != nil {
			println("======--3", err.Code, err.Reason)
		}
		return err
	}).Resolve(0, 9527)
	time.Sleep(1 * time.Second)
}
