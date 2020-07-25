pub mod token_type;
pub mod token;
pub mod reserved;

pub use super::span::Span;
pub use token_type::TokenType;
pub use token::Token;

use reserved::{
    check_reserved_idents,
    check_symbols,
    check_symbol,
};


/**
 * Handles tokens by storing and creating them. 
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    input: String, 
    partial_token: String,
    partial_token_offset: usize,
    current_line: usize,
    next_token: Option<Token>,
}


/**
 * Implements Lexer functions. 
 */
impl Lexer {
    /**
     * Created token handler.
     */
    pub fn new(input: String) -> Lexer {
        Lexer{
            input: input,
            partial_token: "".to_string(),
            partial_token_offset: 1,
            current_line: 1,
            next_token: Option::None,
        }
    }


    /**
     * Gets the next token and removes it.
     */
    pub fn next_token(&mut self) ->
            Result<Token, &'static str> {
        let token: Result<Token, &'static str> = self.peak();
        self.next_token = None;
                
        return token;
    }


    /**
     * Gets the next token.
     */
    pub fn peak(&mut self) ->
            Result<Token, &'static str> {
        match & self.next_token {
            Some(token) => {
                return Ok(token.clone());
            },
            None => {
                
                let token: Result<Token, &'static str> = self.create_token(); 
                match & token {
                    Ok(t) => self.next_token = Some(t.clone()),
                    Err(_e) => (),
                };
                return token;
            },
        }; 
    }


    /**
     * Create next token in stream.
     */
    fn create_token(&mut self) ->
            Result<Token, &'static str> {
        if self.hungry() {
            return Ok(self.fsm_start());
        } else {
            return Err("Out of string to tokenize");
        }
    }


    /**
     * Returns true if there is more char:s to tokenize.
     */
    pub fn hungry(& self) -> bool {
        self.input.chars().count() != 0
    }


    /**
     * Adds the next char in input to partial token and removes it from input.
     * NOTE: Throws exception if input sting is empty
     */
    fn consume(&mut self) -> () {
        let mut chs: std::str::Chars<'_> = self.input.chars();
        self.partial_token.push(chs.next().unwrap());
        self.input = chs.collect::<String>();
    }


    /**
     * Discard char.
     */
    fn discard(&mut self) {
        let mut chs: std::str::Chars<'_> = self.input.chars();
        let _ch: char = chs.next().unwrap();
        self.input = chs.collect::<String>(); 
    }

    
    /**
     * Crates a token from the partial token.
     */
    fn tokenize(&mut self, token_type: TokenType) -> Token {
        let current_col: usize =
            self.partial_token_offset +
            self.partial_token.chars().count();
        let token: Token = Token::new(
            token_type.clone(), 
            self.partial_token.clone(), 
            self.current_line, 
            self.partial_token_offset);

        self.partial_token = "".to_string();
        self.partial_token_offset = current_col;

        return token;
    }


    /**
     * Starts a finite state machine for consuming and classifying a token.
     */
    fn fsm_start(&mut self) -> Token {
        let mut chs: std::str::Chars<'_> = self.input.chars();
        let ch: char = chs.next().unwrap();
        if ch.is_alphabetic() || ch == '_' {
            self.consume(); 
            return self.fsm_ident();

        } else if ch.is_numeric() {
            self.consume();
            return self.fsm_number();

        } else if ch == '"' {
           self.consume();
           return self.fsm_string();
        
        } else if ch == '\'' {
           self.consume();
           return self.fsm_char();
        
        } else {
            match chs.next() {
                Some(look_a_head) => {
                    match check_symbols(ch, look_a_head) {
                        Ok(token_type) => {
                            self.consume();
                            self.consume();
                            return self.tokenize(token_type);
                        },
                        Err(_err) => {
                            if ch == ' ' {
                                self.discard();
                                self.partial_token_offset = 1;
                                
                                return self.fsm_start();
                            
                            } else if ch == '\n' {
                                self.discard();
                                self.current_line += 1;
                                self.partial_token_offset = 1;
                                
                                return self.fsm_start();
                            }

                            match check_symbol(ch) {
                                Ok(token_type) => {
                                    self.consume();
                                    return self.tokenize(token_type);
                                },
                                Err(err) => panic!("Lexer err: {}", err),
                            };
                        },
                    };
                },
                None => {
                    match check_symbol(ch) {
                        Ok(token_type) => {
                            self.consume();
                            return self.tokenize(token_type);
                        },
                        Err(err) => panic!("Lexer err: {}", err),
                    };
                }
            };
        }
    }


    /**
     * FSM for converting string to Token of type ident.
     */
    fn fsm_ident(&mut self) -> Token {
        if self.hungry() {
            let mut chs: std::str::Chars<'_> = self.input.chars();
            let ch: char = chs.next().unwrap();
            if ch.is_alphanumeric() || ch == '_' {
                self.consume();
                return self.fsm_ident();
            } else {
                return self.fsm_ident_end();
            }
        } else {
            return self.fsm_ident_end();
        } 
    }


    /**
     * Checks if the ident is reserved and stores the correct token.
     */
    fn fsm_ident_end(&mut self) -> Token {
        let token_type: TokenType = check_reserved_idents(&self.partial_token);
        return self.tokenize(token_type);
    }


    /**
     * FSM for converting string to Token of type number. 
     */
    fn fsm_number(&mut self) -> Token {
        if self.hungry() {
            let mut chs: std::str::Chars<'_> = self.input.chars();
            let ch: char = chs.next().unwrap();

            if ch.is_numeric() {
                self.consume();
                return self.fsm_number();
            } else {
                match ch {
                    '.' => {
                        match chs.next() {
                            Some(look_a_head) => {
                                if look_a_head.is_numeric() {
                                    self.consume();
                                    self.consume();
                                    return self.fsm_float_number();
                                } else {
                                    return self.tokenize(TokenType::Number);
                                }
                            },
                            None => return self.tokenize(TokenType::Number),
                        };
                    },
                    _ => return self.tokenize(TokenType::Number),
                };
            }
        } else {
            return self.tokenize(TokenType::Number);
        }
    }

    
    /**
     * FSM for converting string to token of type float.
     */
    fn fsm_float_number(&mut self) -> Token {
        if self.hungry() {
            let mut chs: std::str::Chars<'_> = self.input.chars();
            let ch: char = chs.next().unwrap();

            if ch.is_numeric() {
                self.consume();
                return self.fsm_float_number();
            } else {
                return self.tokenize(TokenType::FloatNumber);
            }
        } else {
            return self.tokenize(TokenType::FloatNumber);
        }
    }


    /**
     * FSM for converting string to token of type String.
     */
    fn fsm_string(&mut self) -> Token {
        if self.hungry() {
            let mut chs: std::str::Chars<'_> = self.input.chars();
            let ch: char = chs.next().unwrap();

            if ch == '"' {
                self.consume();
                return self.tokenize(TokenType::String);
            
            } else {
                self.consume();
                return self.fsm_string();
            }
        } else {
           panic!("Expected String end"); 
        }
    }


    /**
     * FSM for converting string to token of type Char.
     */
    fn fsm_char(&mut self) -> Token {
        if self.hungry() {
            let mut chs: std::str::Chars<'_> = self.input.chars();
            let ch1: char = chs.next().unwrap();
            let ch2: char = chs.next().unwrap();

            if ch2 == '\'' {
                self.consume();
                self.consume();
                return self.tokenize(TokenType::Char);
            
            } else {
                panic!("Expected char"); 
            }
        } else {
           panic!("Expected char"); 
        }
    }
}

