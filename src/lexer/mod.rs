pub mod token_type;
pub mod token;
//pub mod token_handler;
pub mod reserved;

pub use token_type::TokenType;
pub use token::Token;
//pub use token_handler::TokenHandler;

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
    partial_token: String,
    partial_token_start: usize,
    current_line: usize,
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
            partial_token: "".to_string(),
            partial_token_start: 1,
            current_line: 1,
        }
    }


    /**
     * Creates the next token of the input.
     */
    pub fn next_token(&mut self) -> Result<Token, &'static str> {
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
    fn consume(&mut self) {
        let mut chs: std::str::Chars<'_> = self.input.chars();
        self.partial_token.push(chs.next().unwrap());
        self.input = chs.collect::<String>();
    }

    
    /**
     * Crates a token from the partial token.
     */
    fn tokenize(&mut self, token_type: TokenType) -> Token {
        let current_col: usize = self.partial_token_start + self.partial_token.chars().count();
        let token: Token = Token::new(
            token_type, 
            self.partial_token.clone(), 
            self.current_line, 
            self.partial_token_start, 
            current_col);

        self.partial_token = "".to_string();
        self.partial_token_start = current_col;

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
                                    return self.fsm_number();
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
}

