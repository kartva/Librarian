---
hide:
  - footer
---

Librarian can be used in one of the following three ways:

## Librarian web app

Try out the Librarian [Online tool](https://www.bioinformatics.babraham.ac.uk/librarian/)!

This is particularly useful if you only have a handful of libraries to test, only want to have a look or don't want to install anything. 

Simply select one or more fastq or fastq.gz files from your computer and view or download the Librarian plots. 

## Librarian CLI

You can install Librarian on your own system from [here](https://github.com/DesmondWillowbrook/Librarian/tree/master/server) as a Docker or non-Docker setup.

Librarian CLI can be run in the following modes:

### Web server query 
Base compositions are sent to the web server and output is sent back. This setup requires internet access but has no R dependencies. Using the remote option ensures that samples are compared to the latest reference data model.

### Local (offline) 
The reference data model is stored with Librarian, visualisations and predictions are computed locally. This setup has R depenencies, but no internet access is required.

---

Please report any bugs to [Github Issues](https://github.com/DesmondWillowbrook/Librarian/issues).
