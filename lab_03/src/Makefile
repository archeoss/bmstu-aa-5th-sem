
all:
	make benchmark

benchmark:
	go test -bench=. -benchmem -cpu 4 ./... > ./benchmarks/benchmark.txt
	benchdraw  --x=size --plot=line --y=ns/op --input=./benchmarks/benchmark.txt --output=./plots/plot.svg

clean:
	rm -rf ./benchmarks/benchmark.txt
	rm -rf ./plots/plot.svg
.PHONY: all benchmark