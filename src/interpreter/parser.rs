use crate::interpreter::ast;
use crate::interpreter::tokenizer::Token;
use crate::interpreter::tokenizer::TokenType;

pub fn get_abstract_syntax_tree(tokens: Vec<Token>) -> ast::Program {
    let mut program = ast::Program { body: Vec::new() };
    for token in tokens {
        if token.token_type == TokenType::StateSymbol {
            program
                .body
                .push(ast::TopLevelEntities::StateDef(ast::StateDef {
                    name: "".to_string(),
                    transitions: Vec::new(),
                }));
        }
    }
    program
}
