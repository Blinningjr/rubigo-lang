/**
 * Lexer Token with generic value.
 */
struct Token<T> {
    type: TokenType,
    value: T,
    line: u32,
    start: u32,
    end: u32,
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
