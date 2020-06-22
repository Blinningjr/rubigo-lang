use super ::{
    Token,
    TokenType,
    TokenHandler,
    Literal,
    Operation,
    Span,

    is_literal,
    parse_literal,

    is_operation,
    parse_operation,
};


/**
 * Defines atoms in Rubigo.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Literal(Literal),
    Operation(Operation),
}


/**
 * Parses a Atom.
 */
pub fn parse_atom(token_handler: &mut TokenHandler,
                  token: & Token) -> Span<Atom> {
    if is_literal(token) {
        return parse_literal(token_handler, token);
    } else if is_operation(token) {
        return parse_operation(token);
    }
    panic!("Syntax error: Expected literal or operation");
}


/**
 * Parses atoms into a vec of atoms.
 */
pub fn parse_atoms(token_handler: &mut TokenHandler) -> Vec<Span<Atom>> {
    let mut atoms: Vec<Span<Atom>> = Vec::new();
    while token_handler.hungry() {
        let token: Token = token_handler.next_token(true).unwrap();
        match token.get_type() {
            TokenType::EndExpression => {
                if atoms.len() < 1 {
                    panic!("Syntax error: Expected one or more atoms");
                }
                return atoms;
            },
            _ => atoms.push(parse_atom(token_handler, & token)),
        };
    }
    panic!("Syntax error: Expected ;");
}

