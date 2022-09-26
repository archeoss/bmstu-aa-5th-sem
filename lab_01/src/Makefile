all:
	make benchmark

benchmark:
	go test -bench=. -cpu 4 ./... > ./benchmarks/benchmark_random.txt
	benchdraw  --x=size --plot=line --y=ns/op --input=./benchmarks/benchmark_random.txt --output=./plots/plot-random.svg

clean:
	rm -rf ./benchmarks/benchmark.txt
	rm -rf ./plots/plot.svg

.PHONY: all benchmarkmake
