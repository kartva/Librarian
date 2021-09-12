# For future reference, in case this turns out to be slow.
# https://www.lpalmieri.com/posts/fast-rust-docker-builds/
# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

# Build Rust binaries
FROM rust AS build
WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY ./src ./src

RUN cargo build --release

FROM debian:buster
# Refer to: https://cran.r-project.org/bin/linux/debian/#installation
# for the R installation process
RUN apt-get update \
	&& apt-get install -y gnupg \
	&& echo 'deb http://cloud.r-project.org/bin/linux/debian buster-cran35/' >> /etc/apt/sources.list \
	&& apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv-key 'E19F5F87128899B192B1A2C2AD5F960A256A04AF' \
    && apt-get update \
	&& apt-get install -y r-base-core r-base-dev libssl-dev libcurl4-openssl-dev libxml2-dev \
	&& Rscript -e ' install.packages(c("tidyverse", "umap")) ' \
	&& rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY ./data ./data
COPY ./frontend/dist ./static
COPY ./scripts ./scripts
COPY --from=build /app/target/release/server-web-library-base-compositions ./
EXPOSE 8186
CMD ["./server-web-library-base-compositions"]