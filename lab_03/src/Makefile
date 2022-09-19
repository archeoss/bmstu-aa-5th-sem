
all:
	make benchmark

benchmark:
	go test -bench=Random -cpu 4 ./... > ./benchmarks/benchmark_random.txt
	benchdraw  --x=size --plot=line --y=ns/op --input=./benchmarks/benchmark_random.txt --output=./plots/plot-random.svg

	go test -bench=Sorted -cpu 4 ./... > ./benchmarks/benchmark_sorted.txt
	benchdraw  --x=size --plot=line --y=ns/op --input=./benchmarks/benchmark_sorted.txt --output=./plots/plot-sorted.svg

	go test -bench=Reversed -cpu 4 ./... > ./benchmarks/benchmark_reversed.txt
	benchdraw  --x=size --plot=line --y=ns/op --input=./benchmarks/benchmark_reversed.txt --output=./plots/plot-reversed.svg

clean:
	rm -rf ./benchmarks/benchmark.txt
	rm -rf ./plots/plot.svg
.PHONY: all benchmarkmake