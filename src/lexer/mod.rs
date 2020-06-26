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
pub struct TokenHandler {
    input: String, 
    partial_expression: String,
    partial_expression_offset: usize,
    partial_expression_line: usize,
    partial_token: String,
    partial_token_offset: usize,
    current_line: usize,
    last_token: Option<Token>,
}


/**
 * Implements TokenHandler functions. 
 */
impl TokenHandler {
    /**
     * Created token handler.
     */
    pub fn new(input: String) -> TokenHandler {
        TokenHandler{
            input: input,
            partial_expression: "".to_string(),
            partial_expression_offset: 1,
            partial_expression_line: 1,
            partial_token: "".to_string(),
            partial_token_offset: 1,
            current_line: 1,
            last_token: Option::None,
        }
    }


    /**
     * Creates the next token of the input.
     */
    pub fn next_token(&mut self, no_whitespace: bool) ->
            Result<Token, &'static str> {
        if no_whitespace {
            while self.hungry() {
                let token: Token = self.fsm_start();
                match token.get_type() {
                    TokenType::Space => (),
                    TokenType::NewLine => (),
                    _ => {
                        self.last_token = Some(token.clone());
                        return Ok(token);
                    },
                };
            }
            return Err("Out of string to tokenize");
        } else {
            if self.hungry() {
                self.last_token = Some(self.fsm_start());
                return Ok(self.last_token.clone().unwrap());
            } else {
                return Err("Out of string to tokenize");
            }
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
    fn consume(&mut self) {
        let mut chs: std::str::Chars<'_> = self.input.chars();
        self.partial_token.push(chs.next().unwrap());
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

        match token_type {
            TokenType::NewLine => {
                self.current_line += 1;
                self.partial_token_offset = 1;
            },
            _ => (),
        };

        self.add_partial_expression(& token);

        return token;
    }


    /**
     * Gets the original string of all the latest tokens.
     */
    pub fn get_original(&mut self) -> Span<String> {
        let original: Span<String> = Span::new(
            self.partial_expression.clone(),
            self.partial_expression_line,
            self.partial_expression_offset
        );

        self.partial_expression = "".to_string();
        return original;
    }


    /**
     * Gets the last token.
     */
    pub fn get_last_token(&self) -> Option<Token> {
        return self.last_token.clone();
    }


    /**
     * Adds to the partial expression or starts one.
     */
    fn add_partial_expression(&mut self, token: & Token) -> () {
        if self.partial_expression == "" {
            self.partial_expression_offset = token.get_offset();
            self.partial_expression_line = token.get_line();
        }
        self.partial_expression.push_str(& token.get_value());
    }


    /**
     * Starts a finite state machine for consuming and classifying a token.
     */
    fn fsm_start(&mut self) -> Token {
        let mut chs: std::str::Chars<'_> = self.input.chars();
        let ch: char = chs.next().unwrap();
        if ch.is_alphabetic() || ch == '_' {
            self.consume();
            // Make it call reserved FSM:s.
            return self.fsm_ident();
        } else if ch.is_numeric() {
            self.consume();
            return self.fsm_number();
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
}

