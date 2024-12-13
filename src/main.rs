mod args;
use args::AlignmentArgs;
use clap::Parser;
use colored::Colorize;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 Author Gaurav Sablok
 Universitat Potsdam
 Date 2024-12-13

making a segmented images for the alignments is always a daunting task. This rust crate
solve that problem by using the native rust libraries for the colour and text wrap approach.
added support for viewing all the large scale alignments.Updated the code so that it can
visualize any number of large scale alignments.
*
* */

fn main() {
    let args = AlignmentArgs::parse();
    println!("The base colour encoded information for the large scale alignments are:");
    alignment_embedded_common(&args.alignment_arg);
    println!("The spliced alignment for the given range is as follows:");
    spliced_alignment(
    &args.alignment_arg,
    args.start_alignment.unwrap(),
    args.end_alignment.unwrap(),
    );
}


fn alignment_embedded_common(path: &str) {
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    struct Embedded {
        header: String,
        sequence: String,
    }

    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(&fileopen);
    let mut embedded_hold: Vec<Embedded> = Vec::new();
    let mut hold_header: Vec<String> = Vec::new();
    let mut hold_sequence: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            hold_header.push(line);
        } else {
            hold_sequence.push(line);
        }
    }
    for i in 0..hold_header.len() {
        embedded_hold.push(Embedded {
            header: hold_header[i].clone(),
            sequence: hold_sequence[i].clone(),
        })
    }
    let mut finalholdseq_multivector = Vec::new();
    let mut finalholdid_multivector: Vec<String> = Vec::new();
    for i in 0..hold_header.len() {
        let mut intermediatehold = Vec::new();
        for j in hold_sequence[i].chars() {
            intermediatehold.push(j);
        }
        finalholdseq_multivector.push(intermediatehold);
        finalholdid_multivector.push(hold_header[i].clone());
    }

    for i in 0..finalholdseq_multivector.len() {
        for j in 0..finalholdseq_multivector[0].len() {
            if finalholdseq_multivector[i][j].to_string() == "A" {
                print!(
                    "{}",
                    finalholdseq_multivector[i][j].to_string().yellow().bold()
                )
            } else if finalholdseq_multivector[i][j].to_string() == "T" {
                print!(
                    "{}",
                    finalholdseq_multivector[i][j].to_string().red().bold()
                )
            } else if finalholdseq_multivector[i][j].to_string() == "C" {
                print!(
                    "{}",
                    finalholdseq_multivector[i][j].to_string().blue().bold()
                )
            } else if finalholdseq_multivector[i][j].to_string() == "G" {
                print!(
                    "{}",
                    finalholdseq_multivector[i][j].to_string().green().bold()
                )
            } else if finalholdseq_multivector[i][j].to_string() == "-" {
                print!(
                    "{}",
                    finalholdseq_multivector[i][j].to_string().purple().bold()
                )
            } else {
                continue;
            }
        }
        println!();
    }
}


fn spliced_alignment(path: &str, start: usize, end: usize) {
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    struct Embedded {
        header: String,
        sequence: String,
    }

    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(&fileopen);
    let mut embedded_hold: Vec<Embedded> = Vec::new();
    let mut hold_header: Vec<String> = Vec::new();
    let mut hold_sequence: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            hold_header.push(line);
        } else {
            hold_sequence.push(line);
        }
    }
    for i in 0..hold_header.len() {
        embedded_hold.push(Embedded {
            header: hold_header[i].clone(),
            sequence: hold_sequence[i].clone(),
        })
    }
    let mut finalholdseq_multivector = Vec::new();
    let mut finalholdid_multivector: Vec<String> = Vec::new();
    for i in 0..hold_header.len() {
        let mut intermediatehold = Vec::new();
        for j in hold_sequence[i].chars() {
            intermediatehold.push(j);
        }
        finalholdseq_multivector.push(intermediatehold);
        finalholdid_multivector.push(hold_header[i].clone());
    }

    let mut addclipped: Vec<_> = Vec::new();
    for i in finalholdseq_multivector.iter_mut() {
        let clipped_alignment: Vec<_> = i[start..end].to_vec();
        addclipped.push(clipped_alignment);
    }

    for i in 0..addclipped.len() {
        for j in 0..addclipped[0].len() {
            if addclipped[i][j].to_string() == "A" {
                print!("{}", addclipped[i][j].to_string().yellow().bold())
            } else if addclipped[i][j].to_string() == "T" {
                print!("{}", addclipped[i][j].to_string().red().bold())
            } else if addclipped[i][j].to_string() == "C" {
                print!("{}", addclipped[i][j].to_string().blue().bold())
            } else if addclipped[i][j].to_string() == "G" {
                print!("{}", addclipped[i][j].to_string().green().bold())
            } else if addclipped[i][j].to_string() == "-"{
                print!("{}", addclipped[i][j].to_string().purple().bold())
            } else {
                continue;
            }
        }
        println!();
    }
       }
