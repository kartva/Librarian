import Worker from "worker-loader!./worker.js";

const wasm = import("../pkg/index").then((wasm) => {
    function processFile (files) {
        const wasmProcess = new Worker();
 
        let status = document.getElementById('status');
        status.innerText = "Waiting on server response... May take up to 5 minutes."; //display waiting message
        status.classList.remove('alert', 'alert-danger', 'alert-dismissible', 'fade', 'show'); //remove alert status if it exists

        wasmProcess.onmessage = function (e) {
            let result = e.data;
            
            if (result.err) {
                status.innerText = `Error encountered while processing:\n${result.err}`;
                status.classList.add('alert', 'alert-danger', 'alert-dismissible', 'fade', 'show');
                loading(false); //remove loading part

                throw new Error("Script panic'ed.");
            
            } else {
                async function fetch_plot (compositions) {
                    //download and display plots
                    let data = await fetch ('api/plot_comp', {
                        headers:{
                            "content-type":"application/json"
                        },
                        body:JSON.stringify(compositions),
                        method:"POST"
                    });

                    loading(false); //remove loading part

                    if (data.ok) {
                        //remove status to display results
                        status.innerText = '';
                        status.classList.add('d-none');
                        let graphs = await data.json();
                        
                        //display plots
                        for (const graph of graphs) {
                            const filename = graph.filename;
                            const plot = graph.plot;

                            let enc_data = btoa(plot);
                            let link = 'data:image/svg+xml;base64,' + enc_data;
                            let img = document.createElement('img');
                            img.src = link;
                            img.id = filename; //set id as filename
                            img.classList.add('img-fluid','w-60', 'p-3', 'plot');
                            img.style.height = '400px';

                            let p = document.createElement('p');
                            let label;
                            if (filename == 'Compositions_map.svg') {
                                label = 'UMAP representation of compositions of published sequencing data. Different library types are indicated by colours. Compositions of test libraries are projected onto the same manifold and indicated by light green circles.';
                            }else if (filename == 'Probability_maps.svg') {
                                label = 'This collection of maps shows the probability of a particular region of the map to correspond to a certain library type. The darker the colour, the more dominated the region is by the indicated library type. The location of test libraries is indicated by a light blue circle.';
                            }else if (filename == 'Prediction_plot.svg') {
                                label = 'For each projected test library, the location on the Compositions/Probability Map is determined. This plot shows how published library types are represented at the same location.';
                            }

                            let textNode = document.createTextNode(label);
                            p.appendChild(textNode);

                            let div = document.createElement('div');
                            div.classList.add('col-md-6');
                            div.appendChild(img);
                            div.appendChild(p);
                            document.getElementById('plots').appendChild(div);
                        }

                        document.getElementById('download_plots').classList.remove('d-none'); //display the downloads button


                        // Fill the samples table
                        let res = result['out'];
                        let table = document.getElementById("samples_table");

                        let row = table.insertRow();
                        row.insertCell(0).innerHTML = 'Sample name';
                        row.insertCell(1).innerHTML = 'Sample number';

                        for (let i = 0; i < res.length; i++){
                            row = table.insertRow();
                            let number = res[i]['idx'];
                            if(number < 10){
                                number = '0' + number;
                            }
                            row.insertCell(0).innerHTML = res[i]['name'];
                            row.insertCell(1).innerHTML = number;
                        }

                        document.getElementById('interpretation').classList.remove('d-none');

                    } else {
                        status.innerText = 'Error from server response. Press F12 and look at console for more information.';
                        status.classList.add('alert', 'alert-danger', 'alert-dismissible', 'fade', 'show');
                        throw data;
                    }
                }
                
                let smaller_reads = result.out.map((e, i) => {e.idx = i + 1; return e;}).filter(e => e.reads_read < 100000);

                if (smaller_reads.length != 0) {
                    status.innerText = `Fewer valid reads (${smaller_reads.map(e => ` ${e.reads_read} in sample ${e.idx}`)}) than recommended (100000)
                    (this may be due to reads being filtered out due to being shorter than 50 bases)`;
                    status.classList.add('alert', 'alert-danger', 'alert-dismissible', 'fade', 'show');
                    loading(false); //remove loading part

                    throw new Error("Reads too short");
                }

                fetch_plot(result.out);
            }
        }
 
        wasmProcess.postMessage (files);
    }

    // Prepare the page before running a file
    function setupPage(){
        loading(true);

        //display the result part and the status information
        document.getElementById('result').classList.remove('d-none');
        document.getElementById('status').classList.remove('d-none');

        document.getElementById('download_plots').classList.add('d-none'); //hide the downloads plots button

        //remove previous results plots if exists
        let plots = document.getElementById('plots');
        while (plots.firstChild) {
            plots.removeChild(plots.firstChild);
        }

        //remove previous table
        let table = document.getElementById("samples_table");
        table.innerHTML = '';
    }


    // Hide/display some element after/before loading
    function loading(disabled){
        if(disabled){
            document.getElementById('spinner').classList.remove('d-none'); //display the spinner
        }else{
            document.getElementById('spinner').classList.add('d-none');
        }

        //disabled the run button to avoid multiple launches
        document.getElementById('run').disabled = disabled;
        document.getElementById('file-selector').disabled = disabled;
    }
   

    //run the tool
    function run() {
        setupPage() //prepare the page
        const fileSelector = document.getElementById('file-selector');
        processFile(fileSelector.files);
    }
   
    let runBtn = document.getElementById('run');
   
    document.getElementById('file-selector').addEventListener('change', (e) => {
        runBtn.disabled = false;
    }) //on file change
   
    runBtn.addEventListener('click', run); //on run button
 
    //add event listener on downloads buttons (files and plots)
    document.getElementById('download_files').addEventListener('click', download_files);
    document.getElementById('download_plots').addEventListener('click', download_plots);


    // Download input files
    function download_files() {
        fetch("example_inputs/example-inputs.tar.gz").then(function(t) {
            return t.blob().then((b)=>{
                let a = document.createElement("a");
                a.href = URL.createObjectURL(b);
                a.setAttribute("download", "example-inputs.tar.gz");
                a.click();
            });
        });
    }
 

    // Download output plots
    function download_plots() {
        let imgs = document.getElementsByClassName('plot');
 
        for (let img of imgs){
            let link = document.createElement('a');
            link.href = img.src;
            link.download = img.id; //set filename as id
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
        }
    }
})
