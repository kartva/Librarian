# Library Base Compositions (Web Ver)

[Command line version also available.](https://github.com/ChristelKrueger/Library_Base_Compositions)

## Quickstart
<small><a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/hybrid-applications-with-webpack/using-your-library.html">Reference</a></small>

Install [`npm,`](https://www.npmjs.com/get-npm) [`Rust, Cargo`](https://www.rust-lang.org/) and [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/).

```bash
npm install # Only on first use
wasm-pack build # When rust code / binding code edited
npm start
```

For producion, run: 
```bash
npm run build
# Files will be in dist/
```
### Notes on directories:

* The `static` folder contains any files that you want copied as-is into the final build. It contains an `index.html` file which loads the `index.js` file.

## Attribution:
- `favicon.ico` sourced from [favicon.io](https://favicon.io/emoji-favicons/books) sourced from [twemoji](https://twemoji.twitter.com/), licensed under [CC BY-4](https://creativecommons.org/licenses/by/4.0/).