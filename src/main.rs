use std::{fs::File, io::Read};

use args::Args;
use clap::Parser;
use emitter::Emitter;
use lexer::Lexer;
use library::run_program;
use parser::Parserr;

mod lexer;
mod library;
mod token;
mod parser;
mod emitter;
mod args;

fn main() {

        let args = Args::parse();
        let mut file = File::open(args.filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let contents = contents.replace("\n", "\\n");
        let lexer = Lexer::init(contents);
        let emitter = Emitter::init("out.c".to_owned());
        let mut parser = Parserr::init(lexer, emitter);
        parser.program();
        let mut parsed_emitter = parser.return_emitter();
        parsed_emitter.set_path("out.c".to_owned());
        parsed_emitter.writeFile();
        run_program();
}                      