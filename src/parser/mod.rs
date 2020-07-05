#![allow(dead_code)]

pub mod atom;
pub mod types;
pub mod literal;
pub mod expressions;
pub mod operations; pub mod ast;

pub use super::span::Span;
pub use ast::Ast;

pub use atom::{
    Atom,
    parse_atom,
    parse_atoms,
    parse_variable,
};

pub use operations::{
    Operation,
    is_operation,
    parse_operation,
};

pub use types::{
    Type,
    parse_type,
};

pub use literal::{
    Literal,
    is_literal,
    parse_literal,
    parse_float_number,
    parse_number,
    parse_char,
    parse_string,
    parse_boolean,
};

pub use expressions::{
    Expression,
    Let,
    Assigment,
    Body,
    If,
    While,
    Return,
    FunctionCall,
    Function,
};

use super::lexer::{
    Token,
    TokenHandler,
    TokenType,
};


/**
 * Parses the tokens and returns a Ast. 
 */
pub fn create_ast(token_handler: &mut TokenHandler) -> Expression {
    return check_token(token_handler);
    // while tokens.len() > 0 {
    //     // let token: Token = tokens.remove(0);
    //     check_token(tokens);
    // }
}


/**
 * Checks the first token in tokens and calls its parser.
 */
fn check_token(token_handler: &mut TokenHandler) -> Expression {
    let token: Token = token_handler.next_token(true).unwrap();
    return match token.get_type() {
        TokenType::Char => Expression::Math(
            vec!(parse_char(& token))
        ),
        TokenType::Number => Expression::Math(
            vec!(parse_number(& token))
        ),
        TokenType::FloatNumber => Expression::Math(
            vec!(parse_float_number(& token))
        ),
        TokenType::Let => parse_let(token_handler, & token),
        TokenType::If => parse_if(token_handler, & token),
        TokenType::While => parse_while(token_handler, & token),
        //TokenType::Ident => parse_assigment(token_handler, & token),
        TokenType::Return => parse_return(token_handler, & token),
        TokenType::Ident => parse_function_call(token_handler, & token),
        TokenType::Fn => parse_function(token_handler, & token),
        _ => panic!("Syntax error: Token not implemented {:?}", token),
    };
}


/**
 * Collects all parameters into a list.
 */
fn collect_parameters() {

}


/**
 * Takes a vec of tokens.
 * Returns the concatinated String that the tokens make.
 */
fn get_tokens_string(tokens: & Vec<Token>) -> String {
    let mut result: String = "".to_string();
    for token in tokens.iter() {
        result.push_str(& token.get_value()); 
    }
    return result;
}


/**
 * Returns all tokens before and including the first token of type token_type it finds.
 */
fn parse_until(token_handler: &mut TokenHandler, token_type: TokenType) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    while token_handler.hungry() {
        let token: Token = token_handler.next_token(false).unwrap();
        tokens.push(token.clone());
        if token.get_type() == token_type {
            return tokens;
        }
    }
    return tokens;
}


/**
 * Parses a expression.
 */
fn parse_expression(token_handler: &mut TokenHandler,
                    token: & Token) -> Expression {
    return match token.get_type() {
        TokenType::Let => parse_let(token_handler, token),
        TokenType::If => parse_if(token_handler, token),
        TokenType::While => parse_while(token_handler, token),
        //TokenType::Ident => parse_assigment(token_handler, token),
        TokenType::Return => parse_return(token_handler, token),
        TokenType::Ident => parse_function_call(token_handler, & token),
        _ => panic!("Syntax error: Expexted an expression."),
    };
}


/**
 * Parses token that is of type token_type.
 */
fn parse_token(token: & Token, token_type: TokenType) -> Span<String> {
    if token.get_type() == token_type {
        return Span::new(
            token.get_value(),
            token.get_line(),
            token.get_offset()
        );
    } else {
        panic!("Syntax error: expected {:?}", token_type);
    }
}


/**
 * Parses token of type Let into Let expression.
 */
fn parse_let(token_handler: &mut TokenHandler, token: & Token) -> Expression {
    match token.get_type() {
        TokenType::Let => {
            let ident: Span<String> = parse_token(
                & token_handler.next_token(true).unwrap(),
                TokenType::Ident
            );
            let _colon: Span<String> = parse_token(
                & token_handler.next_token(true).unwrap(),
                TokenType::TypeDec
            );
            let var_type: Span<Type> = parse_type(
                & token_handler.next_token(true).unwrap()
            );
            let _equals: Span<String> = parse_token(
                & token_handler.next_token(true).unwrap(),
                TokenType::Equals
            );
            let value: Vec<Span<Atom>> =
                parse_atoms(token_handler, TokenType::EndExpression);

            return Expression::Let(Let{
                original: token_handler.get_original(),
                ident: ident,
                r#type: var_type,
                value: value,
            });
        },
        _ => panic!("Syntax error: could not parse let"),
    };
}


/**
 * Parses token of type ident into Let expression.
 */
fn parse_assigment(token_handler: &mut TokenHandler,
                   token: & Token) -> Expression {
    match token.get_type() {
        TokenType::Ident => {
            let ident: Span<String> = Span::new(
                token.get_value(),
                token.get_line(),
                token.get_offset()
            );
            let _equals: Span<String> = parse_token(
                & token_handler.next_token(true).unwrap(),
                TokenType::Equals
            );
            let value: Vec<Span<Atom>> =
                parse_atoms(token_handler, TokenType::EndExpression);

            return Expression::Assigment(Assigment{
                original: token_handler.get_original(),
                ident: ident,
                value: value,
            });
        },
        _ => panic!("Syntax error: could not parse assigment expression."),
    };
}


/**
 * Parses a body expression.
 */
fn parse_body(token_handler: &mut TokenHandler, token: & Token) -> Body {
    match token.get_type() {
        TokenType::BodyStart => {
            let mut body: Vec<Expression> = Vec::new();
            while token_handler.hungry() {
                let next_token: Token = token_handler.next_token(true).unwrap();
                match next_token.get_type() {
                    TokenType::BodyEnd => {
                        return Body{
                            raw_start: Span::new(
                                token.get_value(),
                                token.get_line(),
                                token.get_offset()
                            ),
                            raw_end: token_handler.get_original(),
                            body: body,
                        };
                    },
                    _ => body.push(parse_expression(token_handler, & next_token)),
                };
            }
            panic!("Syntax error: expected {.");
        },
        _ => panic!("Syntax error: expected {."),
    };
}


/**
 * Parses a if expresion.
 */
fn parse_if(token_handler: &mut TokenHandler, token: & Token) -> Expression {
    match token.get_type() {
        TokenType::If => {
            let condition: Vec<Span<Atom>> =
                parse_atoms(token_handler, TokenType::BodyStart);
            let original: Span<String> = token_handler.get_original(); 
            let if_body: Body =
                parse_body(token_handler,
                            & token_handler.get_last_token().unwrap());
            return Expression::If(Box::new(If{
                original: original,
                condition: condition,
                if_body: if_body,
                else_body: Option::None,
            }));
        },
        _ => panic!("Syntax error: Expected If expression."),
    };
}


/**
 * Parses a return expression.
 */
fn parse_return(token_handler: &mut TokenHandler, token: & Token) -> Expression {
    match token.get_type() {
        TokenType::Return => { 
            let value: Vec<Span<Atom>> =
                parse_atoms(token_handler, TokenType::EndExpression);

            return Expression::Return(Return{
                original: token_handler.get_original(),
                value: value,
            });
        },
        _ => panic!("Syntax error: Expected retrun expression."),
    };
}


/**
 * Parses a while expression.
 */
fn parse_while(token_handler: &mut TokenHandler, token: & Token) -> Expression {
    match token.get_type() {
        TokenType::While => {
            let condition: Vec<Span<Atom>> =
                parse_atoms(token_handler, TokenType::BodyStart);
            let original: Span<String> = token_handler.get_original(); 
            let body: Body =
                parse_body(token_handler,
                            & token_handler.get_last_token().unwrap());
            return Expression::While(Box::new(While{
                original: original,
                condition: condition,
                body: body,
            }));
        },
        _ => panic!("Syntax error: Expected while expression."),
    };  
}


/**
 * Parses input parameters of function call.
 */
fn parse_input_parameters(token_handler: &mut TokenHandler,
                          token: & Token) -> Vec<Vec<Span<Atom>>> {
    match token.get_type() {
        TokenType::ParenthesisStart => {
            let mut input: Vec<Vec<Span<Atom>>> = Vec::new();
            input.push(parse_atoms(token_handler, TokenType::ParenthesisEnd));
            return input;
        },
        _ => panic!("Syntax error: Expected Parameters."),
    };
}


/**
 * Parsese Function Call expression.
 */
fn parse_function_call(token_handler: &mut TokenHandler,
                       token: & Token) -> Expression {
    match token.get_type() {
        TokenType::Ident => {
            let next_token: Token = token_handler.next_token(true).unwrap();
            let parameters: Vec<Vec<Span<Atom>>> =
                parse_input_parameters(token_handler, & next_token
            );
            let _semi_colon: Span<String> =
                parse_token(& token_handler.next_token(true).unwrap(),
                TokenType::EndExpression
            ); 
            return Expression::FunctionCall(Box::new(FunctionCall{
                original: token_handler.get_original(),
                ident: token.get_value(),
                parameters: parameters,
            })); 
        },
        _ => panic!("Syntax error: Expacted function call expression."),
    };
}


/**
 * Parses function decleration.
 */
fn parse_function(token_handler: &mut TokenHandler,
                  token: & Token) -> Expression {
    match token.get_type() {
        TokenType::Fn => {
            let ident: Span<String> =
                parse_token(& token_handler.next_token(true).unwrap(),
                TokenType::Ident
            );
            let parameter_token: Token = token_handler.next_token(true).unwrap();
            let parameters: Vec<Vec<Span<Atom>>> =
                parse_input_parameters(token_handler, & parameter_token);
            let _arrow: Span<String> =
                parse_token(& token_handler.next_token(true).unwrap(),
                TokenType::FnType
            );
            let return_type: Span<Type> =
                parse_type(& token_handler.next_token(true).unwrap());
            let body_token: Token = token_handler.next_token(true).unwrap();
            let body: Body = parse_body(token_handler, & body_token);

            return Expression::Function(Box::new(Function{
                original: token_handler.get_original(),
                ident: token.get_value(),
                parameters: parameters,
                return_type: return_type,
                body: body,
            })); 
        },
        _ => panic!("Syntax error: expected function decleration,"),
    }; 
}


/**
 * Parse parameters.
 */
fn parse_parameters(token_handler: &mut TokenHandler,
                    token: & Token) -> Vec<Vec<Span<Atom>>> {
    match token.get_type() {
        TokenType::ParenthesisStart => {
            let mut parameters: Vec<Vec<Span<Atom>>> = Vec::new();
            while token_handler.hungry() {
                let next_token: Token = token_handler.next_token(true).unwrap();
                match next_token.get_type() {
                    TokenType::ParenthesisEnd => return parameters,
                    tokenType::Ident => {
                        let _colon: Span<String> =
                            parse_token(& token_handler.next_token(true).unwrap(),
                            TokenType::TypeDec);
                        let r#type: Span<Type> =
                            parse_type(& token_handler.next_token(true).unwrap());
                        parameters.push(
                    },
                    _ => panic!("Syntax error: Expected parameters."),
                };
            }
            panic!("Syntax error: expected ).");
        },
        _ => panic!("Syntax error: Expected parameters."),
    };
}

