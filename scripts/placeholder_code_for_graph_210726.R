#!/usr/bin/env Rscript

## This script reads in the base compositions of the 2019 GEO search which is filtered for the most frequent library types and limits the number of examples 
## of one library to 400.
## Base compositions are fed into UMAP to get a two dimensional representation of the data.
## Base compositions from one additional library (mimicking the library to test) get projected onto the existing UMAP.
## Currently, there is one graph, showing the compositions from the library and the test one on top.
## Eventually, there should also be a table and maybe a heatmap showing the probabilities for all library types, but this code doesn't exist yet.


## Loading packages

library(umap)
library(tidyverse)


## Setting working directory

## Loading gathered library compositions

read_tsv("data/output_210709.tsv") -> output


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



## Arranging data for UMAP

compositions %>% 
  select(-species, -lib_type, -serial_num, -URL, - title) %>% 
  column_to_rownames("srr_number") -> umap.data

compositions %>% 
 select(serial_num, species, lib_type, srr_number, title) -> umap.annotations


## Making up an example to act as test data

compositions %>% 
  slice(1) %>% 
  select(-species, -lib_type, -serial_num, -URL, - title) %>% 
  column_to_rownames("srr_number") -> umap.test


## setting global seed which should be used by umap and the predict functions

set.seed(110)


## Running UMAP

compositions_umap <- umap(umap.data, n_neighbors = 10, min_dist = 0.4) 


## Projecting test library

predict(compositions_umap, umap.test) -> test_umap


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

























