use super::{
    Token,
    TokenType,
};


/**
 * Handles tokens by storing and creating them. 
 */
pub struct TokenHandler {
    input: String,
    tokens: Vec<Token>,
    
    partial_token: String,
    partial_token_start: usize,
    line: usize,
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
            tokens: Vec::new(),
            partial_token: "".to_string(),
            line: 1,
            partial_token_start: 1,
        }
    }


    /**
     * Adds the next char in input to partial token and removes it from input.
     * NOTE: Throws exception if input sting is empty
     */
    pub fn consume(&mut self) {
        let mut chs: std::str::Chars<'_> = self.input.chars();
        self.partial_token.push(chs.next().unwrap());
        self.input = chs.collect::<String>();
    }


    /**
     * Creates a token from the partial token.
     */
    pub fn next_token(&mut self, token_type: TokenType) {
        let current_col: usize = self.partial_token_start + self.partial_token.chars().count();
        self.tokens.push(Token::new(
            token_type, 
            self.partial_token.clone(), 
            self.line, 
            self.partial_token_start, 
            current_col)
        );

        self.partial_token = "".to_string();
        self.partial_token_start = current_col;
    }


    /**
     * Gets the next char of input and a lookahead.
     */
    pub fn next_char(& self) -> (Option<char>, Option<char>) {
        let mut chs: std::str::Chars<'_> = self.input.chars();
        (chs.next(), chs.next())
    }


    /**
     * Gets all tokens.
     */
    pub fn get_tokens(& self) -> Vec<Token> {
        self.tokens.clone()
    }


    /**
     * Returns true if there are more chars in input.
     */
    pub fn hungry(& self) -> bool {
        self.input.chars().count() != 0
    }


    /**
     * Discards the current char.
     */
    pub fn discard(&mut self) {
        let mut chs: std::str::Chars<'_> = self.input.chars();
        self.partial_token_start += 1;

        match chs.next() {
            Some(ch) => {
                match ch {
                    '\n' => {
                        self.line += 1;
                        self.partial_token_start = 1;
                    },
                    _ => (),
                };
            },
            None => (),
        };
        
        self.input = chs.collect::<String>();
    }


    /**
     * Returns the current tokens value.
     */
    pub fn get_token_value(& self) -> &str {
        &self.partial_token
    }
}

