import Worker from "worker-loader!./worker.js";

const wasm = import("../pkg/index").then((wasm) => {
	function processFile (file) {
		console.info(file);

		const wasmProcess = new Worker();

		var li = document.createElement('li');
		let status = document.createElement('p');
		li.appendChild(status);
		
		status.innerText = "Processing... Should take no longer than a minute.";
		document.getElementById('output-list').appendChild(li);

		wasmProcess.onmessage = function (e) {
			let result = e.data;

			if (result.err) {
				status.innerText = "Error encountered while processing. Press F12 for more info.";
				throw new Error("Script panic'ed.");
			} else {
				async function fetch_plot (output) {
					// Download and display graph
					let data = await fetch ('/api/plot_comp', {
						headers:{
							"content-type":"application/json"
						},
						body:JSON.stringify(output),
						method:"POST"
					});
					if (data.ok) {
						let graphs = await data.json();
						console.trace(graphs);
	
						for (const graph of graphs) {
							const img = document.createElement('img');
							// w.r.t https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs
							img.src = 'data:image/png;base64,'+graph;
							li.appendChild(img);
						}
	
						status.innerText = "";
					} else {
						status.innerText = "Error from server response. Press F12 and look at console for more information.";
						console.error(data);
						console.error(data.text())
						throw data;
					}
				}
				
				status.innerText = "Waiting on server response... May take up to 5 minutes.";
				fetch_plot(result.out);
			}
		}

		wasmProcess.postMessage (file);
	}
	
	function run() {
		const fileSelector = document.getElementById('file-selector');
	
		for ( var i = 0; i < fileSelector.files.length; i++) {
			let file = fileSelector.files[i];
			processFile (file);
		}
	}
	
	document.getElementById('file-selector').onchange = run;
	console.debug("Loaded event listener to input-form");
})