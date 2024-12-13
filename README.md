# rust-view-aln

- a embedded colour coded rust visualization approach for the multiple sequence alignment. 
- upgraded this crate to the complete version so that it now allows from the shorter alignments to large scale alignments in one crate. 
- general note: incase of golang and RUST, please see the last commit message and if the message says final and binary released, means code completed else in development phase. 

![](https://github.com/applicativesystem/rust-view-aln/blob/master/alignment_spliced_alignment.png)

```
cargo build

```
```
λ gauravsablok rust-alignment-viewer → λ git master* → ./rust-alignment-viewer -h
Usage: rust-alignment-viewer <ALIGNMENT_ARG> [START_ALIGNMENT] [END_ALIGNMENT]

Arguments:
  <ALIGNMENT_ARG>    please provide the path to the alignment file
  [START_ALIGNMENT]  pleasr provide the start of the clipped alignment
  [END_ALIGNMENT]    please provide the end of the clipped alignment

Options:
  -h, --help     Print help
  -V, --version  Print version

```

```
  ./target/debug/rust-alignment-viewer ./sample-files/sample.fasta 2 18

```

Gaurav Sablok
