<center>
<img src="frontend/static/favicon.ico" />

# Librarian 
</center>

A quality-assurance tool to sanity check FASTQ compositions and their library types.

![image](https://user-images.githubusercontent.com/51814158/168992210-33d2dfaf-5be4-41c9-94f5-67f8328ab22b.png)
![image](https://user-images.githubusercontent.com/51814158/168992258-af672539-7d3b-440f-9f4d-c4ae62012948.png)



## Batch query cli

Look at the [batch query cli](cli/README.md) for an alternative to the website.

Plots will be produced in the same directory as of input file.

## Folder Structure
- `frontend` contains code for the website, which consists of the webpage and WebAssembly code responsible for extracting base compositions from given files. Extracted base compositions are sent to the server for plotting.
- `server` contains code for the server, which serves the `frontend` and also responds to plotting requests.
- `cli` is a utility program to send queries to the server from the command line.

### Attribution:
- `favicon.ico` sourced from [favicon.io](https://favicon.io/emoji-favicons/books) sourced from [twemoji](https://twemoji.twitter.com/), licensed under [CC BY-4](https://creativecommons.org/licenses/by/4.0/).

### Associated repositories:
- [Library Base Compositions](https://github.com/ChristelKrueger/Library_Base_Compositions) - contains the core library for extracting base compositions from FASTQ files.
- [Web Library Base Compositions](https://github.com/DesmondWillowbrook/Web_Library_Base_Compositions)
