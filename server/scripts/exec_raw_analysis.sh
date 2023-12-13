#!/bin/bash

DIR="$( dirname "$0"; )";
Rscript "$DIR/Librarian_analysis_raw.R" --args "$DIR" "$1"