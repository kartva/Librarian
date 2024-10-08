# THIS DOCKERFILE RUNS THE CLI with local mode.

# For future reference, in case this turns out to be slow.
# https://www.lpalmieri.com/posts/fast-rust-docker-builds/
# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

FROM docker.io/rust AS build
WORKDIR /app

COPY ./fastq2comp/ ./fastq2comp/
COPY ./server/ ./server/
COPY ./frontend/ ./frontend/
COPY ./cli/ ./cli/

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./build-release.sh ./build-release.sh

RUN apt-get update && \
    apt-get install -y musl-tools && \
    bash -x ./build-release.sh && \
    tar -xvf release/librarian.tar.gz

# Final image

FROM docker.io/r-base:latest

# Install R packages
RUN apt-get update && \
    apt-get install -y pandoc libssl-dev libcurl4-openssl-dev libxml2-dev libfontconfig1-dev libharfbuzz-dev libfribidi-dev libfreetype-dev libpng-dev libtiff5-dev libjpeg-dev libcairo2-dev 
RUN Rscript -e 'install.packages(c("tidyverse", "umap", "ggrastr", "pins", "rmarkdown"))'

WORKDIR /app
COPY --from=build /app/librarian_v1.2 ./
RUN mkdir /app/out

LABEL org.opencontainers.image.source https://github.com/DesmondWillowbrook/Librarian

ENTRYPOINT ["./librarian", "--local", "--raw", "--output-dir", "/app/out/"]

# example command to run this image:
# docker run  \ 
# -v $(pwd):/app/in # volume mapping from host to container
# -v $(pwd)/out:/app/out # volume mapping from host to container
# -e RUST_LOG='trace' # set the log level to trace
# -t desmondwillowbrook/librarian-server # image name
# /app/in/RNA.example.fastq # path of file relative to mapped volume