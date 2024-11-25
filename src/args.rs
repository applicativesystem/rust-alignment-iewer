use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]

pub struct AlignmentArgs {
    /// please provide the reads R1 file path
    pub alignment_arg: String,
}
