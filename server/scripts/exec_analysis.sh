#!/bin/bash

DIR="$( dirname "$0"; )";
Rscript -e "rmarkdown::render('${DIR}/Librarian_analysis.Rmd', output_dir = '$1')" --args "$1"