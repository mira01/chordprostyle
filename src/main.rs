extern crate chordprostyle;
use chordprostyle::lexer::lex;
use chordprostyle::tri_parser::parse;
use chordprostyle::formatters::parse_formatter::ParseFormatter;

use clap::{Arg, App};

use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
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
        panic!("no files given")
    }
    for i in iter {
        println!("i: {:?}", i);
    }


}

fn doit(){
    let mut args = env::args().skip(1);

    let switch = args.next().unwrap();
    let paths : Vec<String> = if switch == "-l"{
        let f = File::open(args.next().unwrap());
        let mut file = BufReader::new(f.unwrap());
        file.lines().map(|l|{l.unwrap().to_string()}).collect()
    }
    else if switch == "-f"{
        args.map(|l| {l.to_string()}).collect()
    }
    else {
        eprintln!("invocation: -l file_with_pathts | -f files");
        panic!();
    };


    println!("<html><head><link rel='stylesheet' href='styl5.css'><meta charset='utf-8'></head><body>");
    for (i, path) in paths.iter().enumerate(){
        match process_file(&path) {
            Some(song) =>{
                let formater = ParseFormatter::new(song);
                let res = formater.format(&(i+1).to_string());
                println!("{}", res);
            },
            None =>{
                eprintln!("song {} error", path);
            }
        }
    }
    println!("</body></html>");
}

fn process_file(path: &String) -> Option<chordprostyle::model::Song>{ //better be Result
    let mut f = File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents);

    let chars = contents.chars();
    let lexresult = lex(chars);
    let lex_only = false;

    if lex_only {
        for token in lexresult{
            println!("{:?}", token);
        }
        None
    }else{
        let mut parser = parse(lexresult);
        let song = parser.parse();

        Some(song)
    }
}
