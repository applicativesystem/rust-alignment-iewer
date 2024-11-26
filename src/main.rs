mod args;
use args::AlignmentArgs;
use colored::Colorize;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 *Author Gaurav Sablok
 *Universitat Potsdam
 *Date 2024-11-26
  making a segmented images for the alignments is always a daunting task. This rust crate
  solve that problem by using the native rust libraries for the colour and text wrap approach.
*
* */

fn main() {
    let args = AlignmentArgs::parse();
    println!("The common sites along with the common sites information in the alignment are:");
    alignment_embedded_common(&args.alignment_arg);
    println!("The mismatch sites along with the mismatched sites information in the alignment are:");
    alignment_embedded_mismatch(&args.alignment_arg);
    println!("The gapped sites along wih the gapped sites information in the alignment are: ");
    alignment_embedded_gapped(&args.alignment_arg);

}

fn alignment_embedded_common(path: &str) {

  #[derive(Debug, Clone)]
  struct Embedded {
    header: String,
    sequence: String,
  }

  let fileopen = File::open(&path).expect("file not found");
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
       if finalholdseq_multivector[i][j].to_string() ==
        finalholdseq_multivector[i+1][j].to_string() {
        println!("{}\t{}{}", j, finalholdseq_multivector[i][j].to_string().yellow().bold(), 
                finalholdseq_multivector[i+1][j].to_string().yellow().bold())
    } else {
      continue
    }
  }
}
}

fn alignment_embedded_mismatch(path: &str) {

  #[derive(Debug, Clone)]
  struct Embedded {
    header: String,
    sequence: String,
  }

  let fileopen = File::open(&path).expect("file not found");
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
      if finalholdseq_multivector[i][j] != 
          finalholdseq_multivector[i+1][j] {
           println!("{}\t{}{}", j, finalholdseq_multivector[i][j].to_string().blue().bold(), 
                                   finalholdseq_multivector[i+1][j].to_string().red().bold())
    } else {
      continue
    }
  }
}
}

fn alignment_embedded_gapped(path: &str) {

  #[derive(Debug, Clone)]
  struct Embedded {
    header: String,
    sequence: String,
  }

  let fileopen = File::open(&path).expect("file not found");
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
       if finalholdseq_multivector[i][j].to_string() == "-"
       && finalholdseq_multivector[i+1][j].to_string() == "A"
       || finalholdseq_multivector[i][j].to_string() == "-" 
       && finalholdseq_multivector[i+1][j].to_string() == "T" ||
       finalholdseq_multivector[i][j].to_string() == "-" 
       && finalholdseq_multivector[i+1][j].to_string() == "G" ||
       finalholdseq_multivector[i][j].to_string() == "-" 
       && finalholdseq_multivector[i+1][j].to_string() == "C" {
       println!("{}\t{}{}", j, finalholdseq_multivector[i][j].to_string().white().bold(), 
       finalholdseq_multivector[i+1][j].to_string().red().bold())
    } else {
      continue
    }
  }
  }
}
