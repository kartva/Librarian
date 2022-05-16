onmessage = async function(e) {
	console.log('Files received from main script');
	let files = e.data.files;
	let run_json_exported = e.data.run_json_exported;
	
	console.debug(files);

	let result = [];
	for (let file of files) {
		// for communication with exported function 'get_file'
		self.readFile = file;
		console.debug('Processing file: ');
		console.debug(file);

		try {
			result.push(JSON.parse(run_json_exported("application/x-gzip" == file.type)));
		} catch (e) {
			console.error(e);
			postMessage({err: file.name});
			return;
		}
	}

	postMessage({out: result});
}