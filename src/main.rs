use chordprostyle as lib;
use lib::formatters::TeraFormatter;
use lib::tri_parser::TriParser;
use clap::{Arg, App};

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main(){
    let args = App::new("ChordproStyle")
        .arg(Arg::with_name("files")
            .takes_value(true)
            .multiple(true)
        )
        .arg(Arg::with_name("file with paths")
            .short("l")
            .takes_value(true)
            .conflicts_with("files")
        )
        .get_matches();

    let iter: Box<dyn Iterator<Item=String>>;
    if let Some(files) = args.values_of("files"){
        iter = Box::new(files.map(|s| s.to_owned()));
    } else if let Some(path) = args.value_of("file with paths"){
        let f = BufReader::new(File::open(path).expect("Could not open file with paths"));
        iter = Box::new(f.lines().map(|l| l.unwrap()));
    } else {
        // TODO: stdin
        panic!("no files given")
    }

    let mut parser = TriParser::new();
    let formatter = TeraFormatter::new();
    if let Err(errors) = lib::process_files(iter, &mut parser, formatter){
        eprintln!("{:?}", errors);
    }
}
