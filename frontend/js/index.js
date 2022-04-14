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
                status.innerText = 'Error encountered while processing. Press F12 for more info.';
                status.classList.add('alert', 'alert-danger', 'alert-dismissible', 'fade', 'show');
                loading(false); //remove loading part

                throw new Error("Script panic'ed.");
            
            } else {
                async function fetch_plot (compositions) {
                    //download and display plots
                    let data = await fetch ('/api/plot_comp', {
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
                        
                        for (const graph of graphs) {
                            let link = 'data:image/png;base64,' + graph;
                            let img = document.createElement('img');
                            img.src = link;
                            img.classList.add('img-fluid','w-60', 'p-3');

                            let div = document.createElement('div');
                            div.classList.add('col-md-6');
                            div.appendChild(img);
                            document.getElementById('plots').appendChild(div);
                        }

                        document.getElementById('download_plots').classList.remove('d-none'); //display the downloads button
                    } else {
                        status.innerText = 'Error from server response. Press F12 and look at console for more information.';
                        status.classList.add('alert', 'alert-danger', 'alert-dismissible', 'fade', 'show');
                        throw data;
                    }
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
        let files = ['in.fastq', 'in.fastq.gz']; //files to downloads
        for(let i = 0; i < files.length; i++){
            let filename = 'LibrarianServer_'.concat(files[i]);
            fetch(files[i]).then(function(t) {
                return t.blob().then((b)=>{
                    let a = document.createElement("a");
                    a.href = URL.createObjectURL(b);
                    a.setAttribute("download", filename);
                    a.click();
                });
            });
        }
    }
 

    // Download output plots
    function download_plots() {
        let img = document.getElementsByTagName('img');
        let filename = document.getElementById('file-selector').files[0]['name'].split('.fastq')[0];
 
        for(let i = 0; i < img.length; i++){
            let link = document.createElement('a');
            link.href = img[i].src;
            link.download = 'LibrarianServer_'.concat(filename, '_plot_', i+1);
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
        }
    }
})
