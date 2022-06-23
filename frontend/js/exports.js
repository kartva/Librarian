export function get_file () {
	return self.readFile;
}

export function set_error (msg) {
	console.error(msg);
	postMessage({err: msg});
}