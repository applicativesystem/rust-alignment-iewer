# rust-view-multi-aln

- started a webassembly approach for genomic alignments for plants and metagenomes.
- will convert this with additional svelte into a complete biowasm web assembly, so give any BAM, CRAM, SAM file and it will show the genome.
- from reads to entire genome and metagenome visualization.
- it will make the alignments specific for each file and in case of the BAM, SAM and others will also integrate the Gosling. 
- general note: incase of golang and RUST, please see the last commit message and if the message says final and binary released, means code completed else in development phase. 

![](https://github.com/applicativesystem/rust-view-multi-aln/blob/master/embedded_alignment_visualization.png)

```
cargo build

```
```
λ gauravsablok rust-view-multi-aln → λ git master* → ./target/debug/rust-view-multi-aln -h
Usage: rust-view-multi-aln <ALIGNMENT_ARG>

Arguments:
  <ALIGNMENT_ARG>  please provide the path to the alignment file

Options:
  -h, --help     Print help
  -V, --version  Print version

```

Gaurav Sablok
