use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]

pub struct AlignmentArgs {
    /// please provide the path to the alignment file
    pub alignment_arg: String,
    /// pleasr provide the start of the clipped alignment
    pub start_alignment: Option<usize>,
    /// please provide the end of the clipped alignment
    pub end_alignment: Option<usize>,
}
