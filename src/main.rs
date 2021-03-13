use chordprostyle as lib;
use lib::formatters::TeraFormatter;
use lib::tri_parser::TriParser;
use lib::Formatter;
use lib::LibError;
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

    let result = go(&args);
    
    match result {
        Ok(processed_files) =>{
            match processed_files {
                Ok(no_error) => {;}
                Err(some_errors) => {
                    eprintln!("Following errors occured: {:?}", some_errors);
                    std::process::exit(1);
                }
            }
        }
        Err(invocation_error) => {
            eprintln!("error invoking program {:?}", invocation_error);
            std::process::exit(100);
        }
    }

}

fn go(args: &ArgMatches) -> Result<Result<(), Vec<(String, LibError)>>, LibError>{
    let mut template_storage = String::new();
    let formatter = formatter(&mut template_storage, &args)?;
    let mut parser = TriParser::new();
    let iter = source_files(&args)?;
    Ok(lib::process_files(iter, &mut parser, formatter))
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
