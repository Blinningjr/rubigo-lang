/**
 * Lexer Token with generic value.
 */
struct Token<T> {
    type: TokenType,
    value: T,
    line: u32,
    start_col: u32,
    end_col: u32,
}


/**
 * All the different token types.
 */
enum TokenType {
    fn,
    while,
    if,
    else,
    let,
    ident,
    type,
    boolean,
    number,
    string,

    body_start,
    body_end,
    parenthesis_start,
    parenthesis_end,
    line_end;
}


/**
 * Handles tokens by storing and creating them. 
 */
struct TokenHandler {
    input: String,
    tokens: Vec<Token>,
    
    partial_token: String,
    partial_token_start: u32,
    line: u32,
}


/**
 * Created token handler.
 */
impl TokenHandler {
    fn new(input: String) -> TokenHandler {
        TokenHandler{
            input: input,
            tokens: Vec<Token>,
            partial_token: "",
            line: 0,
            partial_token_start: 0,
        }
    }
}


/**
 * Adds the next char in input to partial token and removes it from input.
 */
impl Consume for TokenHandler {
    fn consume(&mut self) {
        let chs = self.input.chars();
        self.partial_token.push(chs.next());
        self.input = chs.collect::<String>();
    }
}


/**
 * Creates a token from the partial token.
 */
impl NextToken for TokenHandler {
    fn nextToken(&mut self, type: TokenType) {
        let current_col = self.partial_token_start + self.partial_token.len();
        self.tokens.push(Token{
            type: type.
            value: self.partial_token,
            line: self.line,
            start_col: self.partial_token_start,
            end_col: current_col,
        });

        self.partial_token = "";
        self.partial_token_start = current_col;
    }
}


/**
 * Gets the next char of input and a lookahead.
 */
impl NextChar for TokenHandler {
    fn nextChar(& self) -> (char, char) {
        let chs = self.input.chars();
        (chs.next(), chs.next())
    }
}
