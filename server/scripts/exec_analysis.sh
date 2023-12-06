#!/bin/bash
args=("$@")

Rscript -e "rmarkdown::render('${args[0]}', output_dir = '${args[1]}')" --args "${args[1]}"