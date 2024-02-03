export function get_update_threshold () {
	return 10000000n;
}

export function update_progress () {
	postMessage({type: "progress", amount: 1});
}

export function set_error (msg) {
	console.error(msg);
	postMessage({err: msg});
}