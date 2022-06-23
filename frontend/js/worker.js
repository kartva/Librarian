onmessage = async function(e) {
	const wasm = await import("../pkg/index");

	console.log('Files received from main script');
	let files = e.data;
	console.debug(files);

	let result = [];
	for (let file of files) {
		// for communication with exported function 'get_file'
		self.readFile = file;
		console.debug('Processing file: ');
		console.debug(file);

		try {
			result.push(JSON.parse(wasm.run_json_exported("application/x-gzip" == file.type)));
		} catch (e) {
			console.error(e);
			// our panic hook calls the exported function set_error to propogate the actual error.
			return;
		}
	}

	postMessage({out: result});
}