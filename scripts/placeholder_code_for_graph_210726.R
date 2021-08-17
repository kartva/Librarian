## This script reads in the base compositions of the 2019 GEO search which is filtered for the most frequent library types and limits the number of examples 
## of one library to 400.
## Base compositions are fed into UMAP to get a two dimensional representation of the data.
## Base compositions from one additional library (mimicking the library to test) get projected onto the existing UMAP.
## Currently, there is one graph, showing the compositions from the library and the test one on top.
## Eventually, there should also be a table and maybe a heatmap showing the probabilities for all library types, but this code doesn't exist yet.


## Loading packages

library(umap)
library(tidyverse)

## Getting input file path
args = commandArgs()

## Loading gathered library compositions

read_tsv("data/output_210709.tsv") -> output
read_tsv(stdin(), col_names = F) -> test.import

## header for the handling of the test library

header <- as.character(expression(A1, C1, G1, T1, N1, A2, C2, G2, T2, N2, A3, C3, G3, T3, N3, A4, C4, G4, T4, N4, A5, C5, G5, T5, N5, A6, C6, G6, T6, N6, A7, C7, G7, T7, N7, A8, C8, G8, T8, N8, A9, C9, G9, T9, N9, A10, C10, G10, T10, N10, A11, C11, G11, T11, N11, A12, C12, G12, T12, N12, A13, C13, G13, T13, N13, A14, C14, G14, T14, N14, A15, C15, G15, T15, N15, A16, C16, G16, T16, N16, A17, C17, G17, T17, N17, A18, C18, G18, T18, N18, A19, C19, G19, T19, N19, A20, C20, G20, T20, N20, A21, C21, G21, T21, N21, A22, C22, G22, T22, N22, A23, C23, G23, T23, N23, A24, C24, G24, T24, N24, A25, C25, G25, T25, N25, A26, C26, G26, T26, N26, A27, C27, G27, T27, N27, A28, C28, G28, T28, N28, A29, C29, G29, T29, N29, A30, C30, G30, T30, N30, A31, C31, G31, T31, N31, A32, C32, G32, T32, N32, A33, C33, G33, T33, N33, A34, C34, G34, T34, N34, A35, C35, G35, T35, N35, A36, C36, G36, T36, N36, A37, C37, G37, T37, N37, A38, C38, G38, T38, N38, A39, C39, G39, T39, N39, A40, C40, G40, T40, N40, A41, C41, G41, T41, N41, A42, C42, G42, T42, N42, A43, C43, G43, T43, N43, A44, C44, G44, T44, N44, A45, C45, G45, T45, N45, A46, C46, G46, T46, N46, A47, C47, G47, T47, N47, A48, C48, G48, T48, N48, A49, C49, G49, T49, N49, A50, C50, G50, T50, N50
))

## Some tidying

output %>% 
  select(-257) %>% 
  drop_na() %>%  # get rid of the ones that don't have compositions
  mutate(Nsum = rowSums(select(.,starts_with("N")))) %>% 
  filter(Nsum < 301) %>% # select the ones which have a relatively low percentage of N
  drop_na() %>% 
  select(-starts_with("N"), -Nsum) %>% # disregarding Ns
  distinct(srr_number, .keep_all = T) -> compositions # deduplicating SRR numbers (I checked, some are duplicated, they are carried through from 
                                                      # the original search from entries with very similar titles. They appear to be the same data sets.)

colnames(test.import) <- header

test.import %>% 
  drop_na() %>%  # get rid of the ones that don't have compositions
  mutate(Nsum = rowSums(select(.,starts_with("N")))) %>% 
  filter(Nsum < 301) %>% # should give a warning here instead
  drop_na() %>% 
  select(-starts_with("N"), -Nsum) -> test # disregarding Ns

## Arranging data for UMAP

compositions %>% 
  select(-species, -lib_type, -serial_num, -URL, - title) %>% 
  column_to_rownames("srr_number") -> umap.data

compositions %>% 
 select(serial_num, species, lib_type, srr_number, title) -> umap.annotations


## Arranging test data

test %>% 
  as.data.frame() -> test.data


## setting global seed which should be used by umap and the predict functions

set.seed(110)


## Running UMAP

compositions_umap <- umap(umap.data, n_neighbors = 10, min_dist = 0.4) 


## Projecting test library

predict(compositions_umap, test.data) -> test_umap


## Preparing for plotting

compositions_umap$layout %>% 
  as.data.frame() %>% 
  rownames_to_column() %>% 
  as_tibble() %>% 
  rename(SRR = rowname, UMAP1 = V1, UMAP2 = V2) %>%
  left_join(umap.annotations, by = c("SRR" = "srr_number")) -> compositions_umap_results

test_umap %>% 
  as.data.frame() %>% 
  rownames_to_column() %>% 
  as_tibble() %>% 
  rename(SRR = rowname, UMAP1 = V1, UMAP2 = V2) -> test_coordinates


## Plotting

compositions_umap_results %>%
  filter(UMAP1 > -12 & UMAP1 < 25) %>% 
  filter(UMAP2 > -12 & UMAP2 < 12) %>%           #filtering to the displayed area
  mutate(lib_type = if_else(lib_type == "miRNA-Seq", "small_RNA-Seq", lib_type)) %>% 
  mutate(lib_type = if_else(lib_type == "ncRNA-Seq", "small_RNA-Seq", lib_type)) %>% 
  mutate(lib_type = if_else(lib_type == "ATAC-seq", "ATAC-Seq", lib_type)) %>% 
  ggplot(aes(UMAP1,UMAP2, colour = lib_type, group = SRR)) +
  geom_point(size = 1.5) +
  geom_point(data = test_coordinates, size = 5, colour = "#33ff66") +
  theme_bw(base_size = 14) +
  theme(legend.title = element_text(size = 12)) +
  ylim(-12, 12) +
  xlim(-12, 25) +
  guides(colour = guide_legend(override.aes = list(size=4))) + 
  theme(legend.title = element_blank()) -> compositions_umap_results_plot

compositions_umap_results_plot

## This here is the interactive version which displays the SRR numbers
## Not sure we need this, but it could be an example to try out the interactivity

#ggplotly(compositions_umap_results_plot, tooltip = "SRR")


## Coordinates and metadata

compositions_umap_results %>%
  filter(UMAP1 > -12 & UMAP1 < 25) %>% 
  filter(UMAP2 > -12 & UMAP2 < 12) %>%           #filtering to the displayed area
  mutate(lib_type = if_else(lib_type == "miRNA-Seq", "small_RNA-Seq", lib_type)) %>% 
  mutate(lib_type = if_else(lib_type == "ncRNA-Seq", "small_RNA-Seq", lib_type)) %>% 
  mutate(lib_type = if_else(lib_type == "ATAC-seq", "ATAC-Seq", lib_type)) %>% 
  write_csv(nullfile())

ggsave(filename=args[6], path="/tmp", device="png", width = 6, height = 4, units = "in")