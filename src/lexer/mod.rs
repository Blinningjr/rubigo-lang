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
    
    original_tracker: Vec<Span<String>>,
    original: String,
    original_line: usize,
    original_offset: usize,

    current_input_line: String,
}


/**
 * Implements Lexer functions. 
 */
impl Lexer {

    pub fn print_stack(& self) -> () {
        println!("{:#?}", self.original_tracker);    
    }
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
           
            original_tracker: Vec::new(),
            original: "".to_string(),
            original_line: 1,
            original_offset: 1,

            current_input_line: "".to_string(),
        }
    }


    /**
     * Gets the next token and removes it.
     */
    pub fn next_token(&mut self) ->
            Result<Token, &'static str> {
        let token: Result<Token, &'static str> = self.peak();
        self.next_token = None;
       
        self.current_input_line = format!("{}{}", self.current_input_line, self.original);
        let split = self.current_input_line.split("\n");
        let vec: Vec<&str> = split.collect();
        self.current_input_line = vec[vec.len() - 1].to_string();

        self.original_tracker.push(Span::new(self.original.clone(), 
                                             self.original_line,
                                             self.original_offset));
        self.original = "".to_string();
        self.original_line = self.current_line;
        self.original_offset = self.partial_token_offset;

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
     * Gets the current parsed text line.
     */
    pub fn get_line(& self) -> String {
        return self.current_input_line.clone();
    }


    /**
     * Gets the next tokens id.
     */
    pub fn get_original_start(& self) -> usize {
        return self.original_tracker.len(); 
    }


    /**
     * Gets the original input since the start token number.
     */
    pub fn get_original(&mut self, start: usize) -> Span<String> {
        let mut counter: usize = start;
        let mut original: String = "".to_string();
        
        let mut line: usize = 1;
        let mut offset: usize = 1;

        if start < self.original_tracker.len() {
            line = self.original_tracker[start].get_line();
            offset = self.original_tracker[start].get_offset();
        }
        while counter < self.original_tracker.len() { 
            original.push_str(& self.original_tracker[counter].get_fragment());
            counter += 1;
        } 

        return self.strip_begining(original, line, offset);
    }

    fn strip_begining(&mut self, mut original: String,
                      mut line: usize,
                      mut offset: usize) -> Span<String> {
        let temp: String = original.clone();
        let mut splitter = temp.splitn(2, '\n'); 
        match splitter.next() {
            Some(first) => {
                match splitter.next() {
                    Some(secound) => {
                        original = "".to_string();
                        let chs: std::str::Chars<'_> = first.chars();
                        let mut no_remove = false;
                        for ch in chs {
                            original.push(ch);
                            
                            if ch != ' ' {
                                no_remove = true;
                            }
                        }
                        let mut spacer: &str = "";
                        if no_remove {
                            spacer = "\n"; 
                        } else {
                            original = "".to_string();
                            line += 1;
                            offset = 1;
                        }
                        original = format!("{}{}{}", original, spacer, secound);
                    },
                    _ => (),
                }; 
            },
            _ => (),
        };
        return Span::new(original, line, offset); 
    }


    /**
     * Adds the next char in input to partial token and removes it from input.
     * NOTE: Throws exception if input sting is empty
     */
    fn consume(&mut self) -> () {
        let mut chs: std::str::Chars<'_> = self.input.chars();
        let ch: char = chs.next().unwrap();
        
        self.partial_token.push(ch.clone());
        self.original.push(ch.clone());

        self.input = chs.collect::<String>();
    }


    /**
     * Discard char.
     */
    fn discard(&mut self) {
        let mut chs: std::str::Chars<'_> = self.input.chars();
        let ch: char = chs.next().unwrap(); 
        
        self.original.push(ch.clone());
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
                                self.partial_token_offset += 1;
                                
                                return self.fsm_start();
                            
                            } else if ch == '\n' {
                                self.discard();
                                self.current_line += 1;
                                self.partial_token_offset = 1;
                                
                                return self.fsm_start();
                            } else if ch == '\t' {
                                self.discard();
                                self.partial_token_offset += 3;
                                
                                return self.fsm_start();
                            }

                            match check_symbol(ch) {
                                Ok(token_type) => {
                                    self.consume();
                                    return self.tokenize(token_type);
                                },
                                Err(err) => panic!("Lexer err: {} char '{}'", err, ch),
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
            let _ch1: char = chs.next().unwrap();
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

