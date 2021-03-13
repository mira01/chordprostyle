use chordprostyle as lib;
use lib::formatters::TeraFormatter;
use lib::tri_parser::TriParser;
use lib::Formatter;
use clap::{Arg, App, ArgMatches};

use std::io::BufReader;
use std::io::{BufRead, Read};
use std::io;
use std::fs::{self, File};

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
        .arg(Arg::with_name("template")
             .short("t")
             .long("template")
             .takes_value(true)
            )
        .get_matches();

    let iter = source_files(&args);
    
    let mut template_storage = String::new();
    let formatter = formatter(&mut template_storage, &args).unwrap();

    let mut parser = TriParser::new();
    if let Err(errors) = lib::process_files(iter.unwrap(), &mut parser, formatter){
        eprintln!("{:?}", errors);
        std::process::exit(1);
    }
}

fn formatter<'a>(mut template_storage: &'a mut String, args: &'a ArgMatches) -> io::Result<TeraFormatter<'a>>{
    match args.value_of("template"){
        Some(template_file) => {
            let mut f = BufReader::new(File::open(template_file)?);
            f.read_to_string(&mut template_storage);
            Ok(TeraFormatter::new(template_storage))
        },
        None => {
            Ok(TeraFormatter::default())
        }
    }
}

fn source_files<'a>(args: &'a ArgMatches) -> io::Result<Box<dyn Iterator<Item=String> +'a>>{
    let iter: io::Result<Box<dyn Iterator<Item=String>>>;
    if let Some(files) = args.values_of("files"){
        iter = Ok(Box::new(
            files.map(|s| s.to_owned())
            ));
    } else if let Some(path) = args.value_of("file with paths"){
        let f = BufReader::new(File::open(path)?);
        iter = Ok(Box::new(
            f.lines()
            .map(|l| l.unwrap())
            ));
    } else {
        // TODO: read input from stdin
        iter = Err(io::Error::new(io::ErrorKind::Other, "no files given"));
    }
    iter
}
