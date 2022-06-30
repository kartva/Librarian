<center>
<img src="frontend/static/favicon.ico" />

# Librarian 
</center>

> A tool to predict the sequencing library type from the base composition of a supplied FastQ file.

Reads from high throughput sequencing experiments show base compositions that are characteristic for their library type. For example, data from RNA-seq and WGBS-seq libraries show markedly different distributions of G, A, C and T across the reads. Librarian makes use of different composition signatures for library quality control: Test library compositions are extracted and compared against previously published data sets.

To that end, Librarian produces several plots to help identify library types. For example, it produces the following given the [bisulfite and RNA example files](frontend/example_inputs/):

- Compositions Map: UMAP representation of compositions of published sequencing data. Different library types are indicated by colours. Compositions of test libraries are projected onto the same manifold and indicated by light green circles.

![image](https://user-images.githubusercontent.com/51814158/176667487-9b08e975-d629-40ae-b441-7a6a8ca469c6.png)


- Probability Maps: This collection of maps shows the probability of a particular region of the map to correspond to a certain library type. The darker the colour, the more dominated the region is by the indicated library type. The location of test libraries is indicated by a light blue circle.

![image](https://user-images.githubusercontent.com/51814158/176667516-5bc13283-46a5-4d8e-ac79-78539409b53b.png)


- Prediction Plot: For each projected test library, the location on the Compositions/Probability Map is determined. This plot shows how published library types are represented at the same location.

![image](https://user-images.githubusercontent.com/51814158/176667561-0d5cd83d-bf7f-4df5-ac55-5cc4af0c99ce.png)

> How to interpret: Some regions on the map are very specific to a certain library type, others are more mixed. Therefore, for some test libraries the results will be much clearer than for others. The different plots are intended to provide a good overview of how similar the test library is to published data. The cause of any deviations should be inspected; the interpretation will be different depending on how characteristic the composition signature of the library type and how far off the projection of the test sample is.

You can try Librarian at the [Babraham Institute website](https://www.bioinformatics.babraham.ac.uk/librarian/), run a [tool to query samples from the command-line](cli/README.md), or [set up the server yourself](server/README.md).

## Folder Structure
- `frontend` contains code for the website, which consists of the webpage and WebAssembly code responsible for extracting base compositions from given files. Extracted base compositions are sent to the server for plotting.
- `server` contains code for the server, which serves the `frontend` and also responds to plotting requests.
- `cli` is a utility program to send queries to the server from the command line.

### Attribution:
- `favicon.ico` sourced from [favicon.io](https://favicon.io/emoji-favicons/books) sourced from [twemoji](https://twemoji.twitter.com/), licensed under [CC BY-4](https://creativecommons.org/licenses/by/4.0/).

### Associated repositories:
- [Library Base Compositions](https://github.com/ChristelKrueger/Library_Base_Compositions) - contains the core library for extracting base compositions from FASTQ files.
- [Web Library Base Compositions](https://github.com/DesmondWillowbrook/Web_Library_Base_Compositions)
