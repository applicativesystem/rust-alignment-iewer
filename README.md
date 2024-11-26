# rust-view-aln

- a embedded rust visualization approach for the genomic alignments and colour coded approach.
- this crate is for shorter alignments such as organelle or bacterial genome alignments.
- For longer alignments, see the other repository long read alignment, which uses a last significant bit with embedded approach to visualize them. 
- rust embedded application for visualization. 
- general note: incase of golang and RUST, please see the last commit message and if the message says final and binary released, means code completed else in development phase. 

![](https://github.com/applicativesystem/rust-view-aln/blob/master/embedded_alignment_visualization.png)

```
cargo build

```
```
λ gauravsablok rust-view-aln → λ git master* → ./target/debug/rust-view-aln -h
Usage: rust-view-aln <ALIGNMENT_ARG>

Arguments:
  <ALIGNMENT_ARG>  please provide the reads R1 file path

Options:
  -h, --help     Print help
  -V, --version  Print version

```

```
./target/debug/rust-view-aln ./sample-files/samplealignment.fasta

```

Gaurav Sablok
