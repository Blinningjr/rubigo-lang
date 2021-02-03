mod error;
mod span;
mod lexer;
mod parser;
mod checker;
mod borrow_checker;
mod interp;

use parser::{
    Parser,
    ModualBody,
};

use checker::{
    Checker,
};

use borrow_checker::{
    BorrowChecker,
};

use interp::{
    Interpreter,
};

use structopt::StructOpt;


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
    };
}

fn command_run(filename: String, content: String) -> () {
    let mod_body: ModualBody = Parser::parse(filename, content, true);
    Checker::check(mod_body.clone(), true);
    BorrowChecker::check(mod_body.clone(), true);
    Interpreter::interpret(mod_body);
}

fn command_build(filename: String, content: String) -> () {
    let mod_body: ModualBody = Parser::parse(filename, content, true);
    Checker::check(mod_body.clone(), true);
    BorrowChecker::check(mod_body.clone(), true);
    // TODO llvm
}

fn command_check(filename: String, content: String) -> () {
    let mod_body: ModualBody = Parser::parse(filename, content, true);
    Checker::check(mod_body.clone(), true);
    BorrowChecker::check(mod_body.clone(), true);
}

