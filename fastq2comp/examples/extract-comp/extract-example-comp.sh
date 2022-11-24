#!/bin/bash

libs=('RNA' 'ATAC' 'bisulfite')
for lib in $libs; do
	nums=('10000' '50000' '100000' '200000' '1000000')
	for num in $nums; do
			cargo run --example extract-comp -- $num "../frontend/example_inputs/example_inputs/$lib.example.fastq"
			mv examples/extract-comp/out.json examples/extract-comp/$lib$num.json
	done
done