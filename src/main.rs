mod error;
mod span;
mod lexer;
mod parser;
mod type_checker;

use std::fs;

use parser::{
    Parser,
    Statement,
};

use type_checker::{
    TypeChecker,
};


fn main() {
    let filename = "example-code/example.rbg";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    //println!("With text:\n{}", contents);
    

    let test: String = "    fn testfn(apa: i32) -> i32 {
        return false;
        fn testfn2() -> i32 {
            return testfn(false);
        }
        return testfn2(1);
    }
    ".to_string();

    let test_parser_fail: String = "fn testfn(apa: i32, te: i32) -> i32 {
        let test: &mut i32 = 2 * (123 - 122);
        let test: char = \" asd asd  \"
        if a == apa(123) {
            return 10;
         else {
            return apa(123);
            asdasd
        }
        while true {
            let a: f32 = 1.2;
            return 2;
        } 
        return 10;
    }".to_string();



    println!("\nWith text:\n{}\n", test);

    let statement: Statement = Parser::parse(test, true); 
    println!("Parsed: \n{:#?}\n", statement);

    println!("Type Checked: \n{:#?}\n", TypeChecker::type_check(statement)); 
    
//    let mut tokens: Vec<Token> = Vec::new();
//    let mut lexer: Lexer = Lexer::new(test); 
//    let mut hungry: bool = true;
//    while hungry {
//        match lexer.next_token() {
//            Ok(token) => tokens.push(token),
//            Err(_err) => hungry = false,
//        };
//    }
//    println!("Tokens:\n{:#?}", tokens);
}

