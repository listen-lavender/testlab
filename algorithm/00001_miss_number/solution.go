package test

import (
	"errors"
)

func findMissA(limit int, nums []int) ([]int, error) {
	if len(nums) > limit {
		return []int{}, errors.New("beyond index")
	}
	if len(nums) == limit {
		return nil, nil
	}
	res := make([]int, 0, limit-len(nums))
	if len(nums) == 0 {
		for k := 0; k < limit; k++ {
			res = append(res, k+1)
		}
		return res, nil
	}
	beyond := (limit + 1)
	for _, val := range nums {
		if val > limit {
			return []int{}, errors.New("beyond index")
		}
	}
	extra := make(map[int]struct{}, limit-len(nums))
	for ind, val := range nums {
		i := val%beyond - 1
		if i < 0 {
			continue
		}

		if i >= len(nums) {
			extra[i+1] = struct{}{}
			continue
		}
		j := nums[i] % beyond
		if i > ind {
			nums[i] = j + beyond
		} else {
			nums[i] = beyond
		}
	}
	for ind := 0; ind < limit; ind++ {
		if ind < len(nums) {
			if nums[ind] < beyond {
				res = append(res, ind+1)
			}
		} else {
			if _, ok := extra[ind+1]; !ok {
				res = append(res, ind+1)
			}
		}
	}
	return res, nil
}

func findMissB(limit int, nums []int) ([]int, error) {
	if len(nums) > limit {
		return []int{}, errors.New("beyond index")
	}
	if len(nums) == limit {
		return nil, nil
	}
	existTable := make(map[int]int, limit)
	for _, val := range nums {
		if val > limit {
			return []int{}, errors.New("beyond index")
		}
		existTable[val] = 1
	}
	var res []int
	for ind := 0; ind < limit; ind++ {
		if _, ok := existTable[ind+1]; !ok {
			res = append(res, ind+1)
		}
	}
	return res, nil
}

func findMissC(limit int, nums []int) ([]int, error) {
	if len(nums) > limit {
		return []int{}, errors.New("beyond index")
	}
	if len(nums) == limit {
		return nil, nil
	}
	var res []int
	for ind := 0; ind < limit; ind++ {
		flag := false
		for _, val := range nums {
			if ind+1 == val {
				flag = true
				break
			} else if val > limit {
				return []int{}, errors.New("beyond index")
			}
		}
		if !flag {
			res = append(res, ind+1)
		}
	}
	return res, nil
}
