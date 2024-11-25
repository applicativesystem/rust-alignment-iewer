mod args;
use args::AlignmentArgs;
use std::error::Error;
use colored::Colorize;
use colored::Color;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
 *Author Gaurav Sablok
 *Universitat Potsdam
 *Date 2024-11-24
  making a segmented images for the alignments is always a daunting task. This rust crate
  solve that problem by using the native rust libraries for the colour and text wrap approach.
*
* */

fn main() {
    let args = AlignmentArgs::parse();
    alignment_embedded(&args.alignment_arg);
}

fn alignment_embedded(path: &str) {

  use std::fs;
  use std::fs::File;
  use std::io::{BufRead, BufReader, Write};
  use std::path::Path;
  use std::process::Command;

  #[derive(Debug, Clone)]
  struct Embedded {
    header: String,
    sequence: String,
  }

  let fileopen = File::open("/home/gauravsablok/Desktop/program-system/rust-view-aln/sample-files/samplealignment.fasta").expect("file not found");
  let fileread = BufReader::new(&fileopen);
  let mut embedded_hold:Vec<Embedded> = Vec::new();
  let mut hold_header:Vec<String> = Vec::new();
  let mut hold_sequence:Vec<String> = Vec::new();
  for i in fileread.lines(){
    let line = i.expect("line not present");
    if line.starts_with(">"){
      hold_header.push(line);
    } else {
      hold_sequence.push(line);
    }
  }
  for i in 0..hold_header.len(){
    embedded_hold.push(Embedded{
      header: hold_header[i].clone(),
                       sequence: hold_sequence[i].clone(),
    })
  }
  let mut finalholdseq_multivector = Vec::new();
  let mut finalholdid_multivector:Vec<String> = Vec::new();
  for i in 0..hold_header.len(){
    let mut intermediatehold = Vec::new();
    for j in hold_sequence[i].chars(){
      intermediatehold.push(j);
    }
    finalholdseq_multivector.push(intermediatehold);
    finalholdid_multivector.push(hold_header[i].clone());
  }
  for i in 0..finalholdseq_multivector.len()-1{
    for j in 0..finalholdseq_multivector[0].len(){
       if finalholdseq_multivector[i][j] == finalholdseq_multivector[i][j]{
      println!("{}\t{}", finalholdseq_multivector[i][j].to_string().blue().bold(),
                                      finalholdseq_multivector[i+1][j].to_string().blue().bold())
    }
      if finalholdseq_multivector[i][j] !=
        finalholdseq_multivector[i+1][j]{
        println!("{}\t{}", finalholdseq_multivector[i][j].to_string().blue().bold(),
                     finalholdseq_multivector[i+1][j].to_string().red().bold())
    }
      if finalholdseq_multivector[i][j].to_string() == "-" && finalholdseq_multivector[i+1][j].to_string() == "A"
        || finalholdseq_multivector[i][j].to_string() == "-" && finalholdseq_multivector[i+1][j].to_string() == "T" ||
        finalholdseq_multivector[i][j].to_string() == "-" && finalholdseq_multivector[i+1][j].to_string() == "G" ||
        finalholdseq_multivector[i][j].to_string() == "-" && finalholdseq_multivector[i+1][j].to_string() == "C" || {
        println!("{}\t{}", finalholdseq_multivector[i][j].to_string().blue().bold(), finalholdseq_multivector[i+1][j].to_string().red().bold())
      }

    }
  }
}
