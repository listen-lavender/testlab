package test

import (
	"math/rand"
	"time"
)

const (
	Order_ASCEND  = 1
	Order_RANDOM  = 0
	Order_DESCEND = -1
)

var (
	hope_10_empty_case       []int
	hope_10_3_case           []int
	hope_10_5_case           []int
	hope_10_8_case           []int
	hope_10000_8_case        []int
	hope_10000_500_case      []int
	hope_10000_990_case      []int
	hope_10000_3333_case     []int
	hope_10000_9527_case     []int
	hope_10_more_beyond_case []int
	hope_10_less_beyond_case []int
	hope_ascend_full_case    []int
	hope_random_full_case    []int
	hope_descend_full_case   []int

	a_10_empty_case       []int
	a_10_3_case           []int
	a_10_5_case           []int
	a_10_8_case           []int
	a_10000_8_case        []int
	a_10000_500_case      []int
	a_10000_990_case      []int
	a_10000_3333_case     []int
	a_10000_9527_case     []int
	a_10_more_beyond_case []int
	a_10_less_beyond_case []int
	a_ascend_full_case    []int
	a_random_full_case    []int
	a_descend_full_case   []int

	b_10_empty_case       []int
	b_10_3_case           []int
	b_10_5_case           []int
	b_10_8_case           []int
	b_10000_8_case        []int
	b_10000_500_case      []int
	b_10000_990_case      []int
	b_10000_3333_case     []int
	b_10000_9527_case     []int
	b_10_more_beyond_case []int
	b_10_less_beyond_case []int
	b_ascend_full_case    []int
	b_random_full_case    []int
	b_descend_full_case   []int

	c_10_empty_case       []int
	c_10_3_case           []int
	c_10_5_case           []int
	c_10_8_case           []int
	c_10000_8_case        []int
	c_10000_500_case      []int
	c_10000_990_case      []int
	c_10000_3333_case     []int
	c_10000_9527_case     []int
	c_10_more_beyond_case []int
	c_10_less_beyond_case []int
	c_ascend_full_case    []int
	c_random_full_case    []int
	c_descend_full_case   []int

	a_bench_10_empty_case       []int
	a_bench_10_3_case           []int
	a_bench_10_5_case           []int
	a_bench_10_8_case           []int
	a_bench_10000_8_case        []int
	a_bench_10000_500_case      []int
	a_bench_10000_990_case      []int
	a_bench_10000_3333_case     []int
	a_bench_10000_9527_case     []int
	a_bench_10_more_beyond_case []int
	a_bench_10_less_beyond_case []int
	a_bench_ascend_full_case    []int
	a_bench_random_full_case    []int
	a_bench_descend_full_case   []int

	b_bench_10_empty_case       []int
	b_bench_10_3_case           []int
	b_bench_10_5_case           []int
	b_bench_10_8_case           []int
	b_bench_10000_8_case        []int
	b_bench_10000_500_case      []int
	b_bench_10000_990_case      []int
	b_bench_10000_3333_case     []int
	b_bench_10000_9527_case     []int
	b_bench_10_more_beyond_case []int
	b_bench_10_less_beyond_case []int
	b_bench_ascend_full_case    []int
	b_bench_random_full_case    []int
	b_bench_descend_full_case   []int

	c_bench_10_empty_case       []int
	c_bench_10_3_case           []int
	c_bench_10_5_case           []int
	c_bench_10_8_case           []int
	c_bench_10000_8_case        []int
	c_bench_10000_500_case      []int
	c_bench_10000_990_case      []int
	c_bench_10000_3333_case     []int
	c_bench_10000_9527_case     []int
	c_bench_10_more_beyond_case []int
	c_bench_10_less_beyond_case []int
	c_bench_ascend_full_case    []int
	c_bench_random_full_case    []int
	c_bench_descend_full_case   []int

	res []int
	err error
)

func equalCase(src, dest []int, err error) bool {
	if err != nil {
		return err.Error() == "beyond index"
	}
	srcExist := make(map[int]struct{}, len(src))
	destExist := make(map[int]struct{}, len(dest))
	for _, val := range src {
		srcExist[val] = struct{}{}
	}
	for _, val := range dest {
		if _, ok := srcExist[val]; !ok {
			return false
		}
		destExist[val] = struct{}{}
	}
	for _, val := range src {
		if _, ok := destExist[val]; !ok {
			return false
		}
	}
	return true
}

func randomOne(limit int, exist map[int]struct{}, retryLimit int) int {
	n := 0
	for k := 0; k < retryLimit; k++ {
		n = rand.Intn(limit-1) + 1
		if _, ok := exist[n]; !ok {
			break
		}
	}
	return n
}

func randomCase(totalLimit int, limit int, retryLimit int) ([]int, []int) {
	rand.Seed(time.Now().UnixNano())
	res := make([]int, 0, totalLimit)
	hope := make([]int, 0, limit-totalLimit)
	valExist := make(map[int]struct{}, totalLimit)
	for k := 0; k < totalLimit; k++ {
		val := randomOne(limit, valExist, retryLimit)
		if val == 0 {
			continue
		} else {
			valExist[val] = struct{}{}
			res = append(res, val)
		}
	}
	for k := 0; k < limit; k++ {
		if _, ok := valExist[k+1]; !ok {
			hope = append(hope, k+1)
		}
	}
	return res, hope
}

func emptyCase(limit int) ([]int, []int) {
	res := make([]int, 0, limit)
	for k := 0; k < limit; k++ {
		res = append(res, k+1)
	}
	return nil, res
}

func fullCase(limit int, order int) ([]int, []int) {
	res := make([]int, 0, limit)
	if order == Order_DESCEND {
		for k := 0; k < limit; k++ {
			res = append(res, limit-k)
		}
	} else {
		for k := 0; k < limit; k++ {
			res = append(res, k+1)
		}
	}
	if order == Order_RANDOM {
		rand.Seed(time.Now().UnixNano())
		for k := 0; k < limit; k++ {
			n := rand.Intn(limit - 1)
			i, j := res[n], res[k]
			res[k], res[n] = i, j
		}
	}
	return res, nil
}

func copyCase(src []int) []int {
	dest := make([]int, len(src))
	copy(dest, src)
	return dest
}

func beyondCase(totalLimit int, limit int) ([]int, []int) {
	res := make([]int, 0, limit)
	if totalLimit < limit {
		res = append(res, limit+1)
	} else {
		for k := 0; k < limit+1; k++ {
			res = append(res, k+1)
		}
	}
	return res, nil
}

func init() {
	retry := 5

	a_10_empty_case, hope_10_empty_case = emptyCase(10)
	a_10_3_case, hope_10_3_case = randomCase(3, 10, retry)
	a_10_5_case, hope_10_5_case = randomCase(3, 10, retry)
	a_10_8_case, hope_10_8_case = randomCase(8, 10, retry)
	a_10000_8_case, hope_10000_8_case = randomCase(8, 10000, retry)
	a_10000_500_case, hope_10000_500_case = randomCase(500, 10000, retry)
	a_10000_990_case, hope_10000_990_case = randomCase(990, 10000, retry)
	a_10000_3333_case, hope_10000_3333_case = randomCase(3333, 10000, retry)
	a_10000_9527_case, hope_10000_9527_case = randomCase(9527, 10000, retry)
	a_10_more_beyond_case, hope_10_more_beyond_case = beyondCase(11, 10)
	a_10_less_beyond_case, hope_10_less_beyond_case = beyondCase(1, 10)
	a_ascend_full_case, hope_ascend_full_case = fullCase(10, Order_ASCEND)
	a_random_full_case, hope_random_full_case = fullCase(10, Order_RANDOM)
	a_descend_full_case, hope_descend_full_case = fullCase(10, Order_DESCEND)

	b_10_empty_case = copyCase(a_10_empty_case)
	b_10_3_case = copyCase(a_10_3_case)
	b_10_5_case = copyCase(a_10_5_case)
	b_10_8_case = copyCase(a_10_8_case)
	b_10000_8_case = copyCase(a_10000_8_case)
	b_10000_500_case = copyCase(a_10000_500_case)
	b_10000_990_case = copyCase(a_10000_990_case)
	b_10000_3333_case = copyCase(a_10000_3333_case)
	b_10000_9527_case = copyCase(a_10000_9527_case)
	b_10_more_beyond_case = copyCase(a_10_more_beyond_case)
	b_10_less_beyond_case = copyCase(a_10_less_beyond_case)
	b_ascend_full_case = copyCase(a_ascend_full_case)
	b_random_full_case = copyCase(a_random_full_case)
	b_descend_full_case = copyCase(a_descend_full_case)

	c_10_empty_case = copyCase(a_10_empty_case)
	c_10_3_case = copyCase(a_10_3_case)
	c_10_5_case = copyCase(a_10_5_case)
	c_10_8_case = copyCase(a_10_8_case)
	c_10000_8_case = copyCase(a_10000_8_case)
	c_10000_500_case = copyCase(a_10000_500_case)
	c_10000_990_case = copyCase(a_10000_990_case)
	c_10000_3333_case = copyCase(a_10000_3333_case)
	c_10000_9527_case = copyCase(a_10000_9527_case)
	c_10_more_beyond_case = copyCase(a_10_more_beyond_case)
	c_10_less_beyond_case = copyCase(a_10_less_beyond_case)
	c_ascend_full_case = copyCase(a_ascend_full_case)
	c_random_full_case = copyCase(a_random_full_case)
	c_descend_full_case = copyCase(a_descend_full_case)

	a_bench_10_empty_case = copyCase(a_10_empty_case)
	a_bench_10_3_case = copyCase(a_10_3_case)
	a_bench_10_5_case = copyCase(a_10_5_case)
	a_bench_10_8_case = copyCase(a_10_8_case)
	a_bench_10000_8_case = copyCase(a_10000_8_case)
	a_bench_10000_500_case = copyCase(a_10000_500_case)
	a_bench_10000_990_case = copyCase(a_10000_990_case)
	a_bench_10000_3333_case = copyCase(a_10000_3333_case)
	a_bench_10000_9527_case = copyCase(a_10000_9527_case)
	a_bench_10_more_beyond_case = copyCase(a_10_more_beyond_case)
	a_bench_10_less_beyond_case = copyCase(a_10_less_beyond_case)
	a_bench_ascend_full_case = copyCase(a_ascend_full_case)
	a_bench_random_full_case = copyCase(a_random_full_case)
	a_bench_descend_full_case = copyCase(a_descend_full_case)

	b_bench_10_empty_case = copyCase(b_10_empty_case)
	b_bench_10_3_case = copyCase(b_10_3_case)
	b_bench_10_5_case = copyCase(b_10_5_case)
	b_bench_10_8_case = copyCase(b_10_8_case)
	b_bench_10000_8_case = copyCase(b_10000_8_case)
	b_bench_10000_500_case = copyCase(b_10000_500_case)
	b_bench_10000_990_case = copyCase(b_10000_990_case)
	b_bench_10000_3333_case = copyCase(b_10000_3333_case)
	b_bench_10000_9527_case = copyCase(b_10000_9527_case)
	b_bench_10_more_beyond_case = copyCase(b_10_more_beyond_case)
	b_bench_10_less_beyond_case = copyCase(b_10_less_beyond_case)
	b_bench_ascend_full_case = copyCase(b_ascend_full_case)
	b_bench_random_full_case = copyCase(b_random_full_case)
	b_bench_descend_full_case = copyCase(b_descend_full_case)

	c_bench_10_empty_case = copyCase(c_10_empty_case)
	c_bench_10_3_case = copyCase(c_10_3_case)
	c_bench_10_5_case = copyCase(c_10_5_case)
	c_bench_10_8_case = copyCase(c_10_8_case)
	c_bench_10000_8_case = copyCase(c_10000_8_case)
	c_bench_10000_500_case = copyCase(c_10000_500_case)
	c_bench_10000_990_case = copyCase(c_10000_990_case)
	c_bench_10000_3333_case = copyCase(c_10000_3333_case)
	c_bench_10000_9527_case = copyCase(c_10000_9527_case)
	c_bench_10_more_beyond_case = copyCase(c_10_more_beyond_case)
	c_bench_10_less_beyond_case = copyCase(c_10_less_beyond_case)
	c_bench_ascend_full_case = copyCase(c_ascend_full_case)
	c_bench_random_full_case = copyCase(c_random_full_case)
	c_bench_descend_full_case = copyCase(c_descend_full_case)
}
