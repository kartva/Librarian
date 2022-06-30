onmessage = async function(e) {
	const wasm = await import("../pkg/index");

	console.log('Files received from main script');
	let files = e.data;
	console.debug(files);

	let result = [];
	let res;
	for (let file of files) {
		// for communication with exported function 'get_file'
		self.readFile = file;
		

		try {
			res = JSON.parse(wasm.run_json_exported("application/x-gzip" == file.type || "application/gzip" == file.type));
			res['name'] = file['name']; // retrieve the sample name
			result.push(res);
		} catch (e) {
			console.error(e);
			// our panic hook calls the exported function set_error to propogate the actual error.
			return;
		}
	}

	postMessage({out: result});
}