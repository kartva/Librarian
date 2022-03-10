

onmessage = async function(e) {
	const wasm = await import("../pkg/index");

	console.log('Files received from main script');
	let files = e.data;
	console.debug(files);

	for (let file of files) {
		// for communication with exported function 'get_file'
		self.readFile = file;
		console.debug('Processing file: ');
		console.debug(file);

		const args = new wasm.SampleArgs (
			BigInt (100000),
			0,
			null,
			50
		);

		let result = {out: []};
		try {
			result.out.push(JSON.parse(wasm.run_json_exported(args, "application/x-gzip" == file.type)));
		} catch (e) {
			result.err = "Process panic'ed";
			break;
		}
	}

	postMessage(result);
}