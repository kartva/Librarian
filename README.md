<center>
<img src="frontend/static/favicon.ico" />

# Librarian 
</center>

A quality-assurance tool to sanity check FASTQ compositions and their library types.

![image](https://user-images.githubusercontent.com/51814158/168992210-33d2dfaf-5be4-41c9-94f5-67f8328ab22b.png)
![image](https://user-images.githubusercontent.com/51814158/168992258-af672539-7d3b-440f-9f4d-c4ae62012948.png)


## Setup:

```bash
docker pull docker.io/desmondwillowbrook/librarian-server
docker run -dp 8186:8186 desmondwillowbrook/librarian-server
```

## Non-Docker setup
Install:
- [`npm`](https://www.npmjs.com/get-npm)
- [`Rust (with Cargo)`](https://www.rust-lang.org/) 
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
- R

```
apt-get install -y r-base-core r-base-dev libssl-dev libcurl4-openssl-dev libxml2-dev
Rscript -e 'install.packages(c("tidyverse", "umap")) '
```

```bash
git clone https://github.com/DesmondWillowbrook/Librarian-Server.git
cd Librarian-Server
```

```bash
./run.sh
```

## Batch query cli

##### Install
```bash
cargo install --git "https://github.com/DesmondWillowbrook/Server_Web_Library_Base_Compositions/" cli
```

##### Build from source
```bash
cd cli
cargo run --release -- ../frontend/example_inputs/*
```

Plots will be produced in the same directory as of input file.

## Folder Structure
- `frontend` contains code for the website, which consists of the webpage and WebAssembly code responsible for extracting base compositions from given files. Extracted base compositions are sent to the server for plotting.
- `server` contains code for the server, which serves the `frontend` and also responds to plotting requests.
- `cli` is a utility program to send queries to the server from the command line.

### Attribution:
- `favicon.ico` sourced from [favicon.io](https://favicon.io/emoji-favicons/books) sourced from [twemoji](https://twemoji.twitter.com/), licensed under [CC BY-4](https://creativecommons.org/licenses/by/4.0/).

### Associated repositories:
- [Library Base Compositions](https://github.com/ChristelKrueger/Library_Base_Compositions) - contains the core library for extracting base compositions from FASTQ files.
- [Web Library Base Compositions](https://github.com/DesmondWillowbrook/Web_Library_Base_Compositions)
