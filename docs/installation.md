---
hide:
  - footer
---

You don't need to install Librarian to use it - if you just have a few fastq files to test then you can simply vist the [Online version of Librarian](https://www.bioinformatics.babraham.ac.uk/librarian/) and select your fastq files to analyse.  No sequence data is sent to the server - only aggregated composition data, so your data stays on your machine.

If you have a larger number of files to analyse then you will want to install the command line version of the program


## Command line installation
The command line version of librarian works on linux machines. We have provided pre-compiled binaries which should work on all 64-bit versions of linux and which should just work straight away.

To install librarian go to the [latest release on github](https://github.com/DesmondWillowbrook/Librarian/releases/latest) and then download the ```librarian.tar.gz``` file. 

```
wget --quiet https://github.com/DesmondWillowbrook/Librarian/releases/latest/download/librarian.tar.gz
```

Once you've downloaded the file you can uncompress it with:

```
tar -xzvf librarian.tar.gz
```

..which will produce something like:

```
librarian_v1.1/scripts/
librarian_v1.1/scripts/Librarian_analysis.Rmd
librarian_v1.1/scripts/.DS_Store
librarian_v1.1/scripts/compositions_umap/
librarian_v1.1/scripts/compositions_umap/20220623T170807Z-a8622/
librarian_v1.1/scripts/compositions_umap/20220623T170807Z-a8622/data.txt
librarian_v1.1/scripts/compositions_umap/20220623T170807Z-a8622/compositions_umap.rds
librarian_v1.1/scripts/librarian_plotting_test_samples_server_220623.R
librarian_v1.1/scripts/Librarian/
librarian_v1.1/scripts/Librarian/test_library_composition_5.txt
librarian_v1.1/scripts/Librarian/prediction_plot.svg
librarian_v1.1/scripts/Librarian/compositions_map.svg
librarian_v1.1/scripts/Librarian/probability_maps.png
librarian_v1.1/scripts/Librarian/Librarian.Rproj
librarian_v1.1/scripts/Librarian/.DS_Store
librarian_v1.1/scripts/Librarian/.Rhistory
librarian_v1.1/scripts/Librarian/librarian_heatmap.txt
librarian_v1.1/scripts/Librarian/librarian_offline_analysis.R
librarian_v1.1/scripts/Librarian/test_library_predictions.txt
librarian_v1.1/scripts/Librarian/probability_maps.svg
librarian_v1.1/scripts/Librarian/Librarian_offline_analysis.Rmd
librarian_v1.1/scripts/Librarian/compositions_map.png
librarian_v1.1/scripts/Librarian/prediction_plot.png
librarian_v1.1/scripts/compositions_umap_results/
librarian_v1.1/scripts/compositions_umap_results/20220623T170809Z-007ce/
librarian_v1.1/scripts/compositions_umap_results/20220623T170809Z-007ce/data.txt
librarian_v1.1/scripts/compositions_umap_results/20220623T170809Z-007ce/compositions_umap_results.rds
librarian_v1.1/scripts/exec_analysis.sh
librarian_v1.1/librarian
```

The main librarian program is ```librarian_v1.1/librarian```

To test the program you can download some example data:

```wget --quiet https://www.bioinformatics.babraham.ac.uk/librarian/example_inputs/example_inputs.zip```

Which you can uncompress with:

```unzip example_inputs.zip```

..which will produce:

```
Archive:  example_inputs.zip
   creating: example_inputs/
  inflating: example_inputs/ATAC.example.fastq
  inflating: example_inputs/RNA.example.fastq
  inflating: example_inputs/bisulfite.example.fastq
```

So you now have three fastq files with which to test the program.  The example files here are uncompressed, but librarian works just fine with fastq.gz files.

### Testing a remote run
At this point you should be able to run the command line program by submitting your composition data to the librarian server for analysis

You can do this by running:

```
librarian_v1.1/librarian example_inputs/*fastq
```

Which should produce:

```
INFO [librarian] Processing "example_inputs/ATAC.example.fastq"
INFO [librarian] Processing "example_inputs/bisulfite.example.fastq"
INFO [librarian] Processing "example_inputs/RNA.example.fastq"
INFO [librarian] Sending data to server at https://www.bioinformatics.babraham.ac.uk/librarian/api/plot_comp
INFO [librarian] Requests may take up to 5 minutes to process.
INFO [librarian] Created "librarian_compositions_map.svg"
INFO [librarian] Created "librarian_compositions_map.png"
INFO [librarian] Created "librarian_probability_maps.svg"
INFO [librarian] Created "librarian_probability_maps.png"
INFO [librarian] Created "librarian_prediction_plot.svg"
INFO [librarian] Created "librarian_prediction_plot.png"
INFO [librarian] Created "librarian_librarian_heatmap.txt"
INFO [librarian] Created "librarian_Librarian_analysis.html"
```

..and you can look at the files produced to check that the analysis completed successfully.

## Running in local mode
The example above submitted the composition data to the librarian server to perform the predictions.  If you are going to be analysing a larger number of files then you will want to run the prediction locally.  This uses the same command line installation, but also requires a local version of R be available.

To run in local mode you must install a recent version of R.  There are serveral ways to do this but here is [one simple option](https://github.com/rstudio/r-builds)

You can check that R is correctly installed by running:

```Rscript --version```

If you see something like:

```
Rscript --version
R scripting front-end version 4.1.1 (2021-08-10)
```

Then you're all good.  Before you run librarian you will also need to install some additional R packages.  To do this you can open an R session and run:

```
install.packages(c("pins","tidyverse","umap","ggrastr"))
```

Once this completes successfully then you can run librarian in local mode.


### Running in local mode
To run in local mode you do the same as before but add the ```--local``` flag to your command.

```
librarian_v1.1/librarian --local example_inputs/*fastq
```

..which produces:

```
INFO [librarian] Processing "example_inputs/ATAC.example.fastq"
INFO [librarian] Processing "example_inputs/bisulfite.example.fastq"
INFO [librarian] Processing "example_inputs/RNA.example.fastq"
INFO [librarian] Created "librarian_compositions_map.svg"
INFO [librarian] Created "librarian_compositions_map.png"
INFO [librarian] Created "librarian_probability_maps.svg"
INFO [librarian] Created "librarian_probability_maps.png"
INFO [librarian] Created "librarian_prediction_plot.svg"
INFO [librarian] Created "librarian_prediction_plot.png"
INFO [librarian] Created "librarian_librarian_heatmap.txt"
INFO [librarian] Created "librarian_Librarian_analysis.html"
```
