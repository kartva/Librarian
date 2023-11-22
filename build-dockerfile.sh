#!/bin/bash
set -euxo pipefail

sudo hwclock --hctosys # to sync time between host and WSL2
podman build -t ghcr.io/desmondwillowbrook/librarian .

# try running the container
podman run \
 -v `pwd`/frontend/example_inputs/example_inputs/:/app/in \
 -v `pwd`/out:/app/out \
 -e RUST_LOG='trace' \
 -t ghcr.io/desmondwillowbrook/librarian \
 /app/in/RNA.example.fastq

# check whether the files are non-empty
test -s out/Librarian_analysis.html

# check whether the files are recent
test `find out/Librarian_analysis.html -mmin -1`

# if all good, push the image to the registry
podman push ghcr.io/desmondwillowbrook/librarian:latest

#if this fails, try logging in first
# echo $CR_PAT | podman login ghcr.io -u DesmondWillowbrook --password-stdin