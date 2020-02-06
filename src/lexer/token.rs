/**
 * Lexer Token.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    value: String,
    line: usize,
    start_col: usize,
    end_col: usize,
}


/**
 * Returns the type of the token. 
 */
impl Token {
    pub fn new(token_type: TokenType, value: String, line: usize, start_col: usize, end_col: usize) -> Token {
        Token{
            token_type: token_type,
            value: value,
            line: line,
            start_col: start_col,
            end_col: end_col,
        }
    }
}


/**
 * Returns the type of the token. 
 */
impl Token {
    pub fn get_type(&self) -> TokenType {
        return self.token_type.clone();
    }
}


/**
 * All the different token types.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Fn,
    While,
    If,
    Else,
    Let,
    Return,
    
    Mut,
    Borrow,
    
    Ident,
    Type,
    Boolean,
    Number,
    Op,

    String,

    BodyStart,
    BodyEnd,
    ParenthesisStart,
    ParenthesisEnd,

    TypeDec,

    FnType,
}


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
 * Created token handler.
 */
impl TokenHandler {
    pub fn new(input: String) -> TokenHandler {
        TokenHandler{
            input: input,
            tokens: Vec::new(),
            partial_token: "".to_string(),
            line: 1,
            partial_token_start: 1,
        }
    }
}


/**
 * Adds the next char in input to partial token and removes it from input.
 * NOTE: Throws exception if input sting is empty
 */
impl TokenHandler {
    pub fn consume(&mut self) {
        let mut chs: std::str::Chars<'_> = self.input.chars();
        self.partial_token.push(chs.next().unwrap());
        self.input = chs.collect::<String>();
    }
}


/**
 * Creates a token from the partial token.
 */
impl TokenHandler {
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
}


/**
 * Gets the next char of input and a lookahead.
 */
impl TokenHandler {
    pub fn next_char(& self) -> (Option<char>, Option<char>) {
        let mut chs: std::str::Chars<'_> = self.input.chars();
        (chs.next(), chs.next())
    }
}


/**
 * Gets all tokens.
 */
impl TokenHandler {
    pub fn get_tokens(& self) -> Vec<Token> {
        self.tokens.clone()
    }
}


/**
 * Returns true if there are more chars in input.
 */
impl TokenHandler {
    pub fn hungry(& self) -> bool {
        self.input.chars().count() != 0
    }
}


/**
 * Discards the current char.
 */
impl TokenHandler {
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
}


/**
 * Returns the current tokens value.
 */
impl TokenHandler {
    pub fn get_token_value(& self) -> &str {
        &self.partial_token
    }
}
