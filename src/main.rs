mod error;
mod span;
mod lexer;
mod parser;
mod type_checker;
mod interpreter;

use std::fs;

use parser::{
    Parser,
    ModualBody,
};

use type_checker::{
    TypeChecker,
};

use interpreter::{
    Interpreter,
};

use structopt::StructOpt;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, StructOpt)]
/// A simple rust-like compiler
enum Cli {
    /// Run interpreter
    Run {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    /// Build binary file
    Build {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    /// Run type and borrow checker
    Check {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
}

fn main() {
    let args = Cli::from_args();

    match args {
        Cli::Run{path} => {
            let content: String = std::fs::read_to_string(&path)
                .expect("could not read file");
            let filename: &str = path.file_name().unwrap().to_str().unwrap();
            command_run(filename.to_string(), content);
        },
        Cli::Build{path} => {
            let content: String = std::fs::read_to_string(&path)
                .expect("could not read file");
            let filename: &str = path.file_name().unwrap().to_str().unwrap();
            command_build(filename.to_string(), content);
        },
        Cli::Check{path} => {
            let content: String = std::fs::read_to_string(&path)
                .expect("could not read file");
            let filename: &str = path.file_name().unwrap().to_str().unwrap();
            command_check(filename.to_string(), content);
        },
        _ => (),
    };
}


fn command_run(filename: String, content: String) -> () {
    let mod_body: ModualBody = Parser::parse(filename, content, true);
    let type_checker: TypeChecker = TypeChecker::type_check(mod_body, true);
    Interpreter::interpret(type_checker.modual);
}

fn command_build(filename: String, content: String) -> () {
    let pb = indicatif::ProgressBar::new(3);
    
    let mod_body: ModualBody = Parser::parse(filename.clone(), content, true);
    pb.println(format!("[+] finished parsing #{}", &filename));
    pb.inc(1);
   
    let type_checker: TypeChecker = TypeChecker::type_check(mod_body, true);
    pb.println(format!("[+] finished type checking #{}", &filename));
    pb.inc(1);
    
    // TODO llvm
    pb.println(format!("[+] finished creating binary file #{}", &filename));
    pb.inc(1);
    pb.finish_with_message("done");
}

fn command_check(filename: String, content: String) -> () {
    let pb = indicatif::ProgressBar::new(2);

    let mod_body: ModualBody = Parser::parse(filename.clone(), content, true);
    pb.println(format!("[+] finished parsing #{}",  &filename));
    pb.inc(1);

    let type_checker: TypeChecker = TypeChecker::type_check(mod_body, true);
    pb.println(format!("[+] finished type checking #{}", &filename));
    pb.inc(1);
    
    pb.finish_with_message("done");
}

