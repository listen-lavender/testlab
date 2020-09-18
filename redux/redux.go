package main

type State int

type Action struct {
	state State
	Type  string
}

type Reducer func(State, Action) State

func multiReducer(oldState State, action Action) State {
	var newState State
	if action.Type == "add" {
		newState = oldState + action.state
	} else if action.Type == "sub" {
		newState = oldState - action.state
	} else {
		newState = action.state
	}
	return newState
}

func addReducer(oldState State, action Action) State {
	var newState State
	newState = oldState + action.state
	return newState
}

func reduce(reducer Reducer, init State, actions []Action) State {
	acc := init
	for _, action := range actions {
		acc = reducer(acc, action)
	}
	return acc
}

func redux(reducer Reducer, init State) (func(Action), func() State) {
	state := init

	dispatch := func(action Action) {
		state = reducer(state, action)
	}
	getState := func() State {
		return state
	}

	return dispatch, getState
}

func main() {
	println(reduce(addReducer, 0, []Action{
		Action{Type: "", state: 2},
		Action{Type: "", state: 3},
		Action{Type: "", state: 4},
		Action{Type: "", state: 5},
	}))
	dispatch, getState := redux(multiReducer, 0)
	dispatch, getState = redux(addReducer, 0)
	dispatch(Action{
		Type:  "add",
		state: 1,
	})
	println(getState())
	dispatch(Action{
		Type:  "add",
		state: 10,
	})
	println(getState())
	dispatch(Action{
		Type:  "sub",
		state: 2,
	})
	println(getState())
	dispatch(Action{
		Type:  "new",
		state: 7,
	})
	println(getState())
}
