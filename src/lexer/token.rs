/**
 * Lexer Token.
 */
#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    value: String,
    line: usize,
    start_col: usize,
    end_col: usize,
}


/**
 * All the different token types.
 */
#[derive(Debug, Clone)]
enum TokenType {
    Fn,
    While,
    If,
    Else,
    Let,
    Ident,
    Type,
    Boolean,
    Number,
    Text,

    BodyStart,
    BodyEnd,
    ParenthesisStart,
    ParenthesisEnd,
    LineEnd,
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
            line: 0,
            partial_token_start: 0,
        }
    }
}


/**
 * Adds the next char in input to partial token and removes it from input.
 * NOTE: Throws exception if input sting is empty
 */
impl TokenHandler {
    pub fn consume(&mut self) {
        let mut chs = self.input.chars();
        self.partial_token.push(chs.next().unwrap());
        self.input = chs.collect::<String>();
    }
}


/**
 * Creates a token from the partial token.
 */
impl TokenHandler {
    pub fn NextToken(&mut self, token_type: TokenType) {
        let current_col = self.partial_token_start + self.partial_token.chars().count();
        self.tokens.push(Token{
            token_type: token_type,
            value: self.partial_token.clone(),
            line: self.line,
            start_col: self.partial_token_start,
            end_col: current_col,
        });

        self.partial_token = "".to_string();
        self.partial_token_start = current_col;
    }
}


/**
 * Gets the next char of input and a lookahead.
 */
impl TokenHandler {
    pub fn nextChar(& self) -> (Option<char>, Option<char>) {
        let mut chs = self.input.chars();
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
