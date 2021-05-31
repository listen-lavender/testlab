# find miss number of integer N


## C. nested loop

````bash

````

## B. hash filter

````bash

````

## A. position transform

````bash

````

## test

````bash
(workspace) ➜  00001_miss_number git:(master) ✗ go test -bench=.
a_10_empty_case:  true
a_10_3_case:  true
a_10_5_case:  true
a_10_8_case:  true
a_10000_8_case:  true
a_10000_500_case:  true
a_10000_990_case:  true
a_10000_3333_case:  true
a_10000_9527_case:  true
a_10_more_beyond_case:  true
a_10_less_beyond_case:  true
a_ascend_full_case:  true
a_random_full_case:  true
a_descend_full_case:  true
b_10_empty_case:  true
b_10_3_case:  true
b_10_5_case:  true
b_10_8_case:  true
b_10000_8_case:  true
b_10000_500_case:  true
b_10000_990_case:  true
b_10000_3333_case:  true
b_10000_9527_case:  true
b_10_more_beyond_case:  true
b_10_less_beyond_case:  true
b_ascend_full_case:  true
b_random_full_case:  true
b_descend_full_case:  true
c_10_empty_case:  true
c_10_3_case:  true
c_10_5_case:  true
c_10_8_case:  true
c_10000_8_case:  true
c_10000_500_case:  true
c_10000_990_case:  true
c_10000_3333_case:  true
c_10000_9527_case:  true
c_10_more_beyond_case:  true
c_10_less_beyond_case:  true
c_ascend_full_case:  true
c_random_full_case:  true
c_descend_full_case:  true
goos: darwin
goarch: amd64
Benchmark_FindMissA_10_empty_case-8             27475056            40.7 ns/op
Benchmark_FindMissA_10_3_case-8                 17930160            62.8 ns/op
Benchmark_FindMissA_10_5_case-8                 18361375            62.0 ns/op
Benchmark_FindMissA_10_8_case-8                 20973406            53.3 ns/op
Benchmark_FindMissA_10000_8_case-8                  9595        125819 ns/op
Benchmark_FindMissA_10000_500_case-8              113961         11520 ns/op
Benchmark_FindMissA_10000_990_case-8              120073          9646 ns/op
Benchmark_FindMissA_10000_3333_case-8             147642          7436 ns/op
Benchmark_FindMissA_10000_9527_case-8            1651844           718 ns/op
Benchmark_FindMissA_10_more_beyond_case-8       38016986            31.4 ns/op
Benchmark_FindMissA_10_less_beyond_case-8       17472870            65.2 ns/op
Benchmark_FindMissA_ascend_full_case-8          383186293            3.09 ns/op
Benchmark_FindMissA_random_full_case-8          385232196            3.08 ns/op
Benchmark_FindMissA_descend_full_case-8         377871322            3.07 ns/op
Benchmark_FindMissB_10_empty_case-8              3686882           320 ns/op
Benchmark_FindMissB_10_3_case-8                  2786142           418 ns/op
Benchmark_FindMissB_10_5_case-8                  2792865           424 ns/op
Benchmark_FindMissB_10_8_case-8                  2470468           481 ns/op
Benchmark_FindMissB_10000_8_case-8                  3446        320662 ns/op
Benchmark_FindMissB_10000_500_case-8                3464        335252 ns/op
Benchmark_FindMissB_10000_990_case-8                3481        333987 ns/op
Benchmark_FindMissB_10000_3333_case-8               2571        417444 ns/op
Benchmark_FindMissB_10000_9527_case-8               1670        663129 ns/op
Benchmark_FindMissB_10_more_beyond_case-8       35465900            32.0 ns/op
Benchmark_FindMissB_10_less_beyond_case-8        9975062           114 ns/op
Benchmark_FindMissB_ascend_full_case-8          359262752            3.26 ns/op
Benchmark_FindMissB_random_full_case-8          432639211            2.75 ns/op
Benchmark_FindMissB_descend_full_case-8         423495289            2.75 ns/op
Benchmark_FindMissC_10_empty_case-8              6010172           196 ns/op
Benchmark_FindMissC_10_3_case-8                  7565970           155 ns/op
Benchmark_FindMissC_10_5_case-8                  7372591           156 ns/op
Benchmark_FindMissC_10_8_case-8                 12011032            95.4 ns/op
Benchmark_FindMissC_10000_8_case-8                  9240        122684 ns/op
Benchmark_FindMissC_10000_500_case-8                 451       2580297 ns/op
Benchmark_FindMissC_10000_990_case-8                 246       4761021 ns/op
Benchmark_FindMissC_10000_3333_case-8                 86      14186902 ns/op
Benchmark_FindMissC_10000_9527_case-8                 45      26788040 ns/op
Benchmark_FindMissC_10_more_beyond_case-8       34603148            32.1 ns/op
Benchmark_FindMissC_10_less_beyond_case-8       35822427            32.5 ns/op
Benchmark_FindMissC_ascend_full_case-8          414983953            2.88 ns/op
Benchmark_FindMissC_random_full_case-8          415733055            2.82 ns/op
Benchmark_FindMissC_descend_full_case-8         425191416            2.82 ns/op
PASS
ok      _/Users/lavenderuni/workspace/thatyear/testlab/algorithm/00001_miss_number  57.206s
````
