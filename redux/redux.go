package main

type State int

type Action struct {
    state State
    Type  string
}

type Reducer func(State, Action) State

func reducer(oldState State, action Action) State {
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

func main(){
    dispatch, getState := redux(reducer, 0)
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
