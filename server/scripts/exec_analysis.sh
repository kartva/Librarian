#!/bin/bash
args=("$@")

Rscript -e "rmarkdown::render('scripts/Librarian_analysis.Rmd', output_dir = '${args[0]}')" --args "${args[0]}"