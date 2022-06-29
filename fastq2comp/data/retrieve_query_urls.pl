#!/usr/bin/perl
use warnings;
use strict;
use FindBin qw($RealBin);
$|++;

# This script takes in a GDS search result and for each entry it
# will:
#
# 1) Identify the GSE accession number
# 2) Use this to obtain the list of samples for this series
# 3) Pick the first sample and get its SRX number
# 4) Query ENA for this number to get the SRR identifier
# 5) Construct the download URL for the first read set for this identifer

my ($gds_file) = @ARGV;


open (IN,$gds_file) or die "Can't read $gds_file: $!";

my $outdir = $gds_file;
$outdir =~ s/^.*\///;
#warn $outdir, "\n";

#$outdir = "$RealBin/$outdir";

#system("mkdir -p \"$outdir\"") == 0 or die "Failed to make results folder";

open (OUT,'>',"results_$outdir") or die "Can't write log file: $!";

while (<IN>) {
		chomp;
		next unless ($_);
		my ($id,$title) = split(/\. /,$_,2);

		my %annotation;

		while (<IN>) {
				chomp;
				last unless ($_);

				if (/Series\s+Accession:\s+(\S+)\s+/) {
						$annotation{series} = $1;
				}

				else {
					#	my ($key,$value) = split(/:\t/);
				#		$annotation{$key} = $value;
				}
		}


		$annotation{sample_id} = get_first_sample_from_series($annotation{series});
		
		next unless ($annotation{sample_id});
		# warn "The annotation is ", $annotation{series}, "\n";

		($annotation{run_id}, $annotation{species}, $annotation{type}) = get_run_from_sample($annotation{sample_id});

		next unless ($annotation{run_id});
               #  warn "The run id is ", $annotation{run_id}, "\n";

                ($annotation{run_srr}, $annotation{url}) = get_srr_from_srx($annotation{run_id});


                next unless ($annotation{run_srr});
                # warn "The run srr is ", $annotation{run_srr}, "\n";


		next unless ($annotation{url});

		#unless ($annotation{species} eq 'Mus musculus') {
			#	warn "Skipping non-mouse ($annotation{species})\n";
			#	next;
		#}

	
		foreach my $element (keys %annotation){
		    # warn ">>$element<<\t>>$annotation{$element}<<\n";
		    $annotation{$element} =~ s/[\r\n]//g;
		    # warn ">>$element<<\t>>$annotation{$element}<<\n";
		}
		
	print OUT join("\t",$id,$annotation{species},$annotation{type},$annotation{run_srr},$annotation{url},$title),"\n";

		


}

sub get_first_sample_from_series {
		my ($gse) = @_;

		my $command = "wget -qO- http://www.ncbi.nlm.nih.gov/geo/query/acc.cgi?acc=$gse\\&targ=self\\&form=text\\&view=quick";


		my $response = `$command`;
                #warn $response;
		if ($response =~ /Series_sample_id = (GSM\d+)/) {
				return $1;
		}

	 
		warn "No GSM number from $gse\n";

}

sub get_run_from_sample {
		my ($gsm) = @_;

		my $command = "wget -qO- http://www.ncbi.nlm.nih.gov/geo/query/acc.cgi?acc=$gsm\\&targ=self\\&form=text\\&view=quick";

		my $response = `$command`;
		my $run_id = "";
		my $species = "";
		my $type = "";

                #warn $response;
		if ($response =~ /(SRX\d+)/){
				$run_id = $1;
				chomp $run_id;
		}

		if ($response =~ /!Sample_organism_ch1 = (.*)/){
				$species = $1;
				chomp $species;
				#warn $species, "\n";
		}

		if ($response =~ /!Sample_library_strategy = (.*)/){
				$type = $1;
				#warn $type, "\n";
		}

		return ($run_id, $species, $type);
		# warn "No SRX number from $gsm\n";

}

sub get_srr_from_srx {

		my ($srx) = @_;

		my $command =  "wget -qO- https://www.ebi.ac.uk/ena/portal/api/filereport\\?accession=$srx\\&result=read_run\\&fields=run_accession,fastq_ftp,fastq_md5,fastq_bytes";

		my $response = `$command`;
                
                #warn "Got the following response:\n>>$response<<\n";

		my($srr, $url) =  (split(/\s+/,$response))[4,5];

		#warn "The SRR number is $srr\n";
		#warn "The URL is $url\n";
		return ($srr, $url);


}



