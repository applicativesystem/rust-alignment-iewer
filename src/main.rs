mod args;
use args::AlignmentArgs;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};


/*
 *Author Gaurav Sablok
 *Universitat Potsdam
 *Date 2024-11-27

  a crate to analyze the large scale alignment coming from the genome alignments
  and then writing the web rendered alignment using the native RUST form builder.
  It will write the web browser files for the genome alignments coming from the metagenome 
  and pangenome alignments. 
*
* */

fn main() {
    let args = AlignmentArgs::parse();
    alignment_match(&args.alignment_arg);
    alignment_mismatch(&args.alignment_arg);
    alignment_gapped(&args.alignment_arg);
    println!("The HTML pages for the alignment view have been written for the matched,
      gapped and mismatch regions of the given alignment")

}

fn alignment_match(path: &str) {

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

  let opentag = r#"<!DOCTYPE html>
<html>
  <head>
    <title>Genome Alignment Browser Viewer</title>
    <style>
      #genomealignment {
        content: "";
        display: table;
        clear: both;
      }
      div {
        float: left;
        height: 1px;
        width: 30%;
        padding: 0 1px;
      }
    </style>
  </head>
  <body>
    <main id="genomealignment">
      <h3>Metagenome Genome Browser Viewer </h3>"#;
let opendiv_i = r#"<div>
        <p>"#;
let closediv_i = r#"</p>      
      </div>"#;
let tagbreak = r#"<br>"#;
let closetag = r#" </body>
     </html>"#;


println!("{}", opentag);
  for i in 0..finalholdseq_multivector.len()-1{
    for j in 0..finalholdseq_multivector[0].len(){
      if finalholdseq_multivector[i][j] ==
          finalholdseq_multivector[i+1][j] && finalholdseq_multivector[i][j].to_string() != "-"
              && finalholdseq_multivector[i+1][j].to_string() != "-" {
            println!("{}{}{}", opendiv_i,finalholdseq_multivector[i][j].to_string(),closediv_i);
    } else {
      continue
    }
  }
}
println!("{}", tagbreak);
for i in 0..finalholdseq_multivector.len()-1{
  for j in 0..finalholdseq_multivector[0].len(){
    if finalholdseq_multivector[i][j] !=
        finalholdseq_multivector[i+1][j] && finalholdseq_multivector[i][j].to_string() != "-"
            && finalholdseq_multivector[i+1][j].to_string() != "-" {
          println!("{}{}{}", opendiv_i, finalholdseq_multivector[i+1][j].to_string(), closediv_i);
  } else {
    continue
  }
}
}
println!("{}", closetag)
}

fn alignment_mismatch(path: &str) {

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

  let opentag = r#"<!DOCTYPE html>
<html>
  <head>
    <title>Genome Alignment Browser Viewer</title>
    <style>
      #genomealignment {
        content: "";
        display: table;
        clear: both;
      }
      div {
        float: left;
        height: 1px;
        width: 30%;
        padding: 0 1px;
      }
    </style>
  </head>
  <body>
    <main id="genomealignment">
      <h3>Metagenome Genome Browser Viewer </h3>"#;
let opendiv_i = r#"<div>
        <p>"#;
let closediv_i = r#"</p>      
      </div>"#;
let tagbreak = r#"<br>"#;
let closetag = r#" </body>
     </html>"#;
     
println!("{}", opentag);
  for i in 0..finalholdseq_multivector.len()-1{
    for j in 0..finalholdseq_multivector[0].len(){
      if finalholdseq_multivector[i][j] !=
          finalholdseq_multivector[i+1][j] && finalholdseq_multivector[i][j].to_string() != "-"
              && finalholdseq_multivector[i+1][j].to_string() != "-" {
            println!("{}{}{}", opendiv_i,finalholdseq_multivector[i][j].to_string(),closediv_i);
    } else {
      continue
    }
  }
}
    
println!("{}", tagbreak);
for i in 0..finalholdseq_multivector.len()-1{
  for j in 0..finalholdseq_multivector[0].len(){
    if finalholdseq_multivector[i][j] !=
        finalholdseq_multivector[i+1][j] && finalholdseq_multivector[i][j].to_string() != "-"
            && finalholdseq_multivector[i+1][j].to_string() != "-" {
          println!("{}{}{}", opendiv_i, finalholdseq_multivector[i+1][j].to_string(), closediv_i);
  } else {
    continue
  }
}
}
println!("{}", closetag)
}

fn alignment_gapped(path: &str) {

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

    let opentag = r#"<!DOCTYPE html>
<html>
  <head>
    <title>Genome Alignment Browser Viewer</title>
    <style>
      #genomealignment {
        content: "";
        display: table;
        clear: both;
      }
      div {
        float: left;
        height: 1px;
        width: 30%;
        padding: 0 1px;
      }
    </style>
  </head>
  <body>
    <main id="genomealignment">
      <h3>Metagenome Genome Browser Viewer </h3>"#;
let opendiv_i = r#"<div>
        <p>"#;
let closediv_i = r#"</p>      
      </div>"#;
let tagbreak = r#"<br>"#;
let closetag = r#" </body>
     </html>"#;
println!("{}", opentag);

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
     && finalholdseq_multivector[i+1][j].to_string() == "C" ||
     finalholdseq_multivector[i][j].to_string() == "A"
     && finalholdseq_multivector[i+1][j].to_string() == "-" ||
     finalholdseq_multivector[i][j].to_string() == "T"
     && finalholdseq_multivector[i+1][j].to_string() == "-" ||
     finalholdseq_multivector[i][j].to_string() == "G"
     && finalholdseq_multivector[i+1][j].to_string() == "-" ||
     finalholdseq_multivector[i][j].to_string() == "C"
     && finalholdseq_multivector[i+1][j].to_string() == "-" {
            println!("{}{}{}", opendiv_i,finalholdseq_multivector[i][j].to_string(),closediv_i);
    } else {
      continue
    }
  }
}

println!("{}", tagbreak);

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
     && finalholdseq_multivector[i+1][j].to_string() == "C" ||
     finalholdseq_multivector[i][j].to_string() == "A"
     && finalholdseq_multivector[i+1][j].to_string() == "-" ||
     finalholdseq_multivector[i][j].to_string() == "T"
     && finalholdseq_multivector[i+1][j].to_string() == "-" ||
     finalholdseq_multivector[i][j].to_string() == "G"
     && finalholdseq_multivector[i+1][j].to_string() == "-" ||
     finalholdseq_multivector[i][j].to_string() == "C"
     && finalholdseq_multivector[i+1][j].to_string() == "-" {
          println!("{}{}{}", opendiv_i, finalholdseq_multivector[i+1][j].to_string(), closediv_i);
  } else {
    continue
  }
}
}
println!("{}", closetag)
}
