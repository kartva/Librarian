#!/bin/bash

# Should be run from project root.
# Test command: cat data/results_example_gds.txt | bash -x data/download-extract/download-extract.sh

# Made with reference to the following format:
# 1	Mus musculus	RNA-Seq	SRR12478073	ftp.sra.ebi.ac.uk/vol1/fastq/SRR124/073/SRR12478073/SRR12478073.fastq.gz	Transcriptome analysis of the TA muscles from WT and Dmd Exon 51 Knockout mice
# 2	Mus musculus	RNA-Seq	SRR12926516	ftp.sra.ebi.ac.uk/vol1/fastq/SRR129/016/SRR12926516/SRR12926516_1.fastq.gz	Prenatal exposure to environmental toxins induces sexually dimorphic transcriptome changes in neonatal offspring

# i.e with a \t (TAB) character between each bit of info

# So script doesn't keep going after error
set -uxo pipefail

# Build Rust binaries incase they have not been built
if [[ ! -f "./target/release/extract_comp" ]]
then
    cargo build --release --bin extract_comp
fi

# Autopopulate TSV file headers incase it does not exist
if [[ ! -f "./data/download-extract/output.tsv" ]]
then
    echo -ne "serial_num\tspecies\tlib_type\tsrr_number\tURL\ttitle\t" >>  ./data/download-extract/output.tsv
    for i in {1..50}
    do
            echo -ne "A$i\tC$i\tG$i\tT$i\tN$i\t" >> ./data/download-extract/output.tsv
    done
    echo "" >> ./data/download-extract/output.tsv # Blank line
fi

while IFS='$\n' read -r line; do
    srr_number=`echo "$line" | awk -F '\t' '{ print $4 }'`

    output="$(python3 ./data/download-extract/sample_srr.py $srr_number 2 100 | cargo run --release --bin extract_comp -- --stdin --stdout --tsv --trim 50 100)" && \
    echo -e "$line\t$output" >> ./data/download-extract/output.tsv
done