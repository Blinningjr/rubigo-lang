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


fn main() {
    let filename = "example-code/example.rbg";
    //println!("In file {}", filename);

//    let _contents = fs::read_to_string(filename)
//        .expect("Something went wrong reading the file");
    //println!("With text:\n{}", contents);
 
    let test: String = "    fn main() -> () {
        fn quad(num: i32) -> i32 {
            fn duble(num: i32) -> i32 {
                return num * 2;
            }
            let duble: i32 = duble(num);
            return duble(duble);
        }
        print(quad(3));
    }
    
    
    ".to_string();

    let _test_type_checker_fail: String = "    fn testfn(apa: i32) -> i32 {
        return false;
        fn testfn() -> i32 {
            if 1 {
                let a: i32 = 10;
                a = 20;
                return testfn(false);
            }
            a = 2;
        }
        return testfn2(1);
    }
    ".to_string();

    let _test_parser_fail: String = "fn testfn(apa: i32, te: i32) -> i32 {
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



    //println!("\nWith text:\n{}\n", test);

    let mod_body: ModualBody = Parser::parse("test".to_string(), test, true); 
    //println!("Parsed: \n{:#?}\n", statement);

    let type_checker: TypeChecker = TypeChecker::type_check(mod_body, true);
    //println!("Type Checked: \n{:#?}\n", type_checker); 

    Interpreter::interpret(type_checker.modual);
    
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

