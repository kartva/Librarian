# Librarian Frontend

## Quickstart
<small><a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/hybrid-applications-with-webpack/using-your-library.html">Reference</a></small>

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