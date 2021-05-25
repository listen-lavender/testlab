package test

import (
	"testing"
)

func Benchmark_FindMissA_10_empty_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissA(10, a_bench_10_empty_case)
	}
}

func Benchmark_FindMissA_10_3_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissA(10, a_bench_10_3_case)
	}
}

func Benchmark_FindMissA_10_5_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissA(10, a_bench_10_5_case)
	}
}

func Benchmark_FindMissA_10_8_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissA(10, a_bench_10_8_case)
	}
}

func Benchmark_FindMissA_10000_8_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissA(10000, a_bench_10000_8_case)
	}
}

func Benchmark_FindMissA_10000_500_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissA(10000, a_bench_10000_500_case)
	}
}

func Benchmark_FindMissA_10000_990_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissA(10000, a_bench_10000_990_case)
	}
}

func Benchmark_FindMissA_10000_3333_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissA(10000, a_bench_10000_3333_case)
	}
}

func Benchmark_FindMissA_10000_9527_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissA(10000, a_bench_10000_9527_case)
	}
}

func Benchmark_FindMissA_10_more_beyond_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissA(10, a_bench_10_more_beyond_case)
	}
}

func Benchmark_FindMissA_10_less_beyond_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissA(10, a_bench_10_less_beyond_case)
	}
}

func Benchmark_FindMissA_ascend_full_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissA(10, a_bench_ascend_full_case)
	}
}

func Benchmark_FindMissA_random_full_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissA(10, a_bench_random_full_case)
	}
}

func Benchmark_FindMissA_descend_full_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissA(10, a_bench_descend_full_case)
	}
}

func Benchmark_FindMissB_10_empty_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissB(10, b_bench_10_empty_case)
	}
}

func Benchmark_FindMissB_10_3_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissB(10, b_bench_10_3_case)
	}
}

func Benchmark_FindMissB_10_5_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissB(10, b_bench_10_5_case)
	}
}

func Benchmark_FindMissB_10_8_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissB(10, b_bench_10_8_case)
	}
}

func Benchmark_FindMissB_10000_8_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissB(10000, b_bench_10000_8_case)
	}
}

func Benchmark_FindMissB_10000_500_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissB(10000, b_bench_10000_500_case)
	}
}

func Benchmark_FindMissB_10000_990_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissB(10000, b_bench_10000_990_case)
	}
}

func Benchmark_FindMissB_10000_3333_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissB(10000, b_bench_10000_3333_case)
	}
}

func Benchmark_FindMissB_10000_9527_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissB(10000, b_bench_10000_9527_case)
	}
}

func Benchmark_FindMissB_10_more_beyond_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissB(10, b_bench_10_more_beyond_case)
	}
}

func Benchmark_FindMissB_10_less_beyond_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissB(10, b_bench_10_less_beyond_case)
	}
}

func Benchmark_FindMissB_ascend_full_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissB(10, b_bench_ascend_full_case)
	}
}

func Benchmark_FindMissB_random_full_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissB(10, b_bench_random_full_case)
	}
}

func Benchmark_FindMissB_descend_full_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissB(10, b_bench_descend_full_case)
	}
}

func Benchmark_FindMissC_10_empty_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissC(10, c_bench_10_empty_case)
	}
}

func Benchmark_FindMissC_10_3_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissC(10, c_bench_10_3_case)
	}
}

func Benchmark_FindMissC_10_5_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissC(10, c_bench_10_5_case)
	}
}

func Benchmark_FindMissC_10_8_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissC(10, c_bench_10_8_case)
	}
}

func Benchmark_FindMissC_10000_8_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissC(10000, c_bench_10000_8_case)
	}
}

func Benchmark_FindMissC_10000_500_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissC(10000, c_bench_10000_500_case)
	}
}

func Benchmark_FindMissC_10000_990_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissC(10000, c_bench_10000_990_case)
	}
}

func Benchmark_FindMissC_10000_3333_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissC(10000, c_bench_10000_3333_case)
	}
}

func Benchmark_FindMissC_10000_9527_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissC(10000, c_bench_10000_9527_case)
	}
}

func Benchmark_FindMissC_10_more_beyond_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissC(10, c_bench_10_more_beyond_case)
	}
}

func Benchmark_FindMissC_10_less_beyond_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissC(10, c_bench_10_less_beyond_case)
	}
}

func Benchmark_FindMissC_ascend_full_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissC(10, c_bench_ascend_full_case)
	}
}

func Benchmark_FindMissC_random_full_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissC(10, c_bench_random_full_case)
	}
}

func Benchmark_FindMissC_descend_full_case(b *testing.B) {
	for i := 0; i < b.N; i++ {
		findMissC(10, c_bench_descend_full_case)
	}
}
