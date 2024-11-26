use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]

pub struct AlignmentArgs {
    /// please provide the path to the alignment file
    pub alignment_arg: String,
}
