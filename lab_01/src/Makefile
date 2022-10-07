all:
	make benchmark

benchmark:
	go test -bench=. -cpu 4 -benchmem ./... > ./benchmarks/benchmark_random.txt
	benchdraw  --x=size --plot=line --y=ns/op --input=./benchmarks/benchmark_random.txt --output=./plots/plot-time.svg
	benchdraw  --x=size --plot=line --y=B/op --input=./benchmarks/benchmark_random.txt --output=./plots/plot-mem.svg

profile:
	cd levenshtein && GOGC=off go test -gcflags '-N -l -m -m' -run TestDL_STD -benchmem -memprofile  iter_mem.out
	#go tool pprof -alloc_space levenshtein/iter_mem.out

	cd levenshtein && GOGC=off go test -gcflags '-N -l -m -m' -run TestDL_CacheRecursive -benchmem -memprofile rec_mem.out
	#go tool pprof -alloc_space levenshtein/rec_mem.out


clean:
	rm -rf ./benchmarks/benchmark.txt
	rm -rf ./plots/plot.svg

.PHONY: all benchmarkmake
