# Skip to line 76 for actual setup, this is setting up both Rust and NPM.

FROM buildpack-deps:buster as frontend

# https://github.com/rust-lang/docker-rust/blob/878a3bd2f3d92e51b9984dba8f8fd8881367a063/1.55.0/buster/Dockerfile
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.55.0

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='3dc5ef50861ee18657f9db2eeb7392f9c2a6c95c90ab41e45ab4ca71476b4338' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='67777ac3bc17277102f2ed73fd5f14c51f4ca5963adadf7f174adf4ebc38747b' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='32a1532f7cef072a667bac53f1a5542c99666c4071af0c9549795bbdb2069ec1' ;; \
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='e50d1deb99048bc5782a0200aa33e4eea70747d49dffdc9d06812fd22a372515' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/1.24.3/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

# https://github.com/nodejs/docker-node/blob/e886c2c1d3a109ccfe169419f3b30f9794bacf61/16/buster/Dockerfile
RUN groupadd --gid 1000 node \
  && useradd --uid 1000 --gid node --shell /bin/bash --create-home node

ENV NODE_VERSION 16.10.0

RUN ARCH= && dpkgArch="$(dpkg --print-architecture)" \
  && case "${dpkgArch##*-}" in \
    amd64) ARCH='x64';; \
    ppc64el) ARCH='ppc64le';; \
    s390x) ARCH='s390x';; \
    arm64) ARCH='arm64';; \
    armhf) ARCH='armv7l';; \
    i386) ARCH='x86';; \
    *) echo "unsupported architecture"; exit 1 ;; \
  esac \
  # gpg keys listed at https://github.com/nodejs/node#release-keys
  && set -ex \
  && for key in \
    4ED778F539E3634C779C87C6D7062848A1AB005C \
    94AE36675C464D64BAFA68DD7434390BDBE9B9C5 \
    74F12602B6F1C4E913FAA37AD3A89613643B6201 \
    71DCFD284A79C3B38668286BC97EC7A07EDE3FC1 \
    8FCCA13FEF1D0C2E91008E09770F7A9A5AE15600 \
    C4F0DFFF4E8C1A8236409D08E73BC641CC11F4C8 \
    C82FA3AE1CBEDC6BE46B9360C43CEC45C17AB93C \
    DD8F2338BAE7501E3DD5AC78C273792F7D83545D \
    A48C2BEE680E841632CD4E44F07496B3EB3C1762 \
    108F52B48DB57BB0CC439B2997B01419BD92F80A \
    B9E2F5981AA6E0CD28160D9FF13993A75599653C \
  ; do \
      gpg --batch --keyserver hkps://keys.openpgp.org:80 --recv-keys "$key" || \
      gpg --batch --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys "$key" ; \
  done \
  && curl -fsSLO --compressed "https://nodejs.org/dist/v$NODE_VERSION/node-v$NODE_VERSION-linux-$ARCH.tar.xz" \
  && curl -fsSLO --compressed "https://nodejs.org/dist/v$NODE_VERSION/SHASUMS256.txt.asc" \
  && gpg --batch --decrypt --output SHASUMS256.txt SHASUMS256.txt.asc \
  && grep " node-v$NODE_VERSION-linux-$ARCH.tar.xz\$" SHASUMS256.txt | sha256sum -c - \
  && tar -xJf "node-v$NODE_VERSION-linux-$ARCH.tar.xz" -C /usr/local --strip-components=1 --no-same-owner \
  && rm "node-v$NODE_VERSION-linux-$ARCH.tar.xz" SHASUMS256.txt.asc SHASUMS256.txt \
  && ln -s /usr/local/bin/node /usr/local/bin/nodejs \
  # smoke tests
  && node --version \
  && npm --version

WORKDIR /app
COPY ./frontend/ ./
RUN cargo install wasm-pack
RUN npm install
RUN wasm-pack build
RUN npm run build

# For future reference, in case this turns out to be slow.
# https://www.lpalmieri.com/posts/fast-rust-docker-builds/
# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

# Build Rust binaries
FROM rust AS build
WORKDIR /app

RUN cargo init --bin
COPY ./server/Cargo.toml ./Cargo.toml
COPY ./server/Cargo.lock ./Cargo.lock
RUN cargo build --release
RUN rm src/*.rs

COPY ./server/src ./src
RUN ls ./target/release/deps/ && rm ./target/release/deps/server_web_library_base_compositions*

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
COPY ./server/data/ ./data
COPY ./server/scripts ./scripts
COPY --from=frontend /app/dist/ ./frontend/dist/
COPY --from=build /app/target/release/server-web-library-base-compositions ./

ENV LIBRARIAN_INDEX_PATH "./frontend/dist"
ENV LIBRARIAN_PORT "8186"

EXPOSE 8186
CMD ["./server-web-library-base-compositions"]