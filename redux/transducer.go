package main

import (
	"strings"
)

type Words []string

type Reducer func(interface{}, interface{}) interface{}

// type ConcatReducer func(string, string) string

// func (w Words) Filter(judge func(str string) bool) Words {
// 	v := make(Words, 0, len(w))
// 	for _, s := range w {
// 		if judge(s) {
// 			v = append(v, s)
// 		}
// 	}
// 	return v
// }

// func (w Words) Map(transform func(str string) string) Words {
// 	v := make(Words, 0, len(w))
// 	for _, s := range w {
// 		v = append(v, transform(s))
// 	}
// 	return v
// }

func (w Words) Map(r Reducer) Words {
	v := make(Words, 0, len(w))
	for _, s := range w {
		newV := r(v, s)
		v = newV.(Words)
	}
	return v
}

func FilterReducer(judge func(str string) bool, combine Reducer) Reducer {
	return func(v interface{}, s interface{}) interface{} {
		sStr := s.(string)
		if judge(sStr) {
			// v = append(v, s)
			// listCombine(v, s)
			return combine(v, s)
		}
		return v
	}
}

func MapReducer(transform func(str string) string, combine Reducer) Reducer {
	return func(v interface{}, s interface{}) interface{} {
		// v = append(v, transform(s))
		// return v
		// return listCombine(v, transform(s))
		sStr := s.(string)
		return combine(v, transform(sStr))
	}
}

func filterReducer(judge func(str string) bool) func(Reducer) Reducer {
	return func(combine Reducer) Reducer {
		return func(v interface{}, s interface{}) interface{} {
			sStr := s.(string)
			if judge(sStr) {
				// v = append(v, s)
				// return v
				// listCombine(v, s)
				return combine(v, s)
			}
			return v
		}
	}
}

func mapReducer(transform func(str string) string) func(Reducer) Reducer {
	return func(combine Reducer) Reducer {
		return func(v interface{}, s interface{}) interface{} {
			// v = append(v, transform(s))
			// return v
			// listCombine(v, transform(s))
			sStr := s.(string)
			return combine(v, transform(sStr))
		}
	}
}

var listCombine Reducer = func(old interface{}, s interface{}) interface{} {
	v := old.(Words)
	sStr := s.(string)
	v = append(v, sStr)
	return v
}

var concat Reducer = func(old interface{}, s interface{}) interface{} {
	oldStr := old.(string)
	sStr := s.(string)
	return oldStr + ":" + sStr
}

// 定义可柯里化函数形式
type function func(...interface{}) interface{}

// 通用柯里化函数
func (f function) curry(i interface{}) function {
	return func(values ...interface{}) interface{} {
		values = append([]interface{}{i}, values...)
		return f(values...)
	}
}

func main() {

	upper := func(str string) string {
		return strings.ToUpper(str)
	}

	isLongEnough := func(str string) bool {
		return len(str) >= 5
	}

	isShortEnough := func(str string) bool {
		return len(str) <= 10
	}

	var mapCurry function = func(values ...interface{}) interface{} {
		return MapReducer(values[0].(func(str string) string), values[1].(Reducer))
	}

	var filterCurry function = func(values ...interface{}) interface{} {
		return FilterReducer(values[0].(func(str string) bool), values[1].(Reducer))
	}

	// mapDo := mapCurry.curry(upper).curry(listCombine)()
	// mapUpper, _ := mapDo.(func(Words, string))
	mapUpper := mapCurry.curry(upper).curry(listCombine)().(Reducer)
	var (
		words    Words
		newWords interface{}
	)
	newWords = mapUpper(words, "aaa")
	words = newWords.(Words)
	newWords = mapUpper(words, "bbb")
	words = newWords.(Words)
	newWords = mapUpper(words, "ccc")
	words = newWords.(Words)
	println(len(words), ": ", strings.Join(words, ","))

	// isCorrectLength := func(str string) bool{
	// 	return isLongEnough(str) && isShortEnough(str)
	// }

	words = Words{}

	filterLong := filterCurry.curry(isLongEnough).curry(mapUpper)().(Reducer)
	filterShort := filterCurry.curry(isShortEnough).curry(filterLong)().(Reducer)
	newWords = filterShort(words, "You")
	words = newWords.(Words)
	newWords = filterShort(words, "have")
	words = newWords.(Words)
	newWords = filterShort(words, "written")
	words = newWords.(Words)
	newWords = filterShort(words, "something")
	words = newWords.(Words)
	newWords = filterShort(words, "very")
	words = newWords.(Words)
	newWords = filterShort(words, "interesting")
	words = newWords.(Words)
	println(len(words), ": ", strings.Join(words, ","))

	isLongEnoughReducer := filterReducer(isLongEnough)
	isShortEnoughReducer := filterReducer(isShortEnough)
	upperReducer := mapReducer(upper)
	upperLongAndShortEnoughReducer := isLongEnoughReducer(isShortEnoughReducer(upperReducer(listCombine)))
	words = Words{}
	newWords = upperLongAndShortEnoughReducer(words, "You")
	words = newWords.(Words)
	newWords = upperLongAndShortEnoughReducer(words, "have")
	words = newWords.(Words)
	newWords = upperLongAndShortEnoughReducer(words, "written")
	words = newWords.(Words)
	newWords = upperLongAndShortEnoughReducer(words, "something")
	words = newWords.(Words)
	newWords = upperLongAndShortEnoughReducer(words, "very")
	words = newWords.(Words)
	newWords = upperLongAndShortEnoughReducer(words, "interesting")
	words = newWords.(Words)
	println(len(words), ": ", strings.Join(words, ","))

	words = []string{"You", "have", "written", "something", "very", "interesting"}
	println(len(words), ": ", strings.Join(words.Map(upperLongAndShortEnoughReducer), ","))
}
