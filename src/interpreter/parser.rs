use crate::interpreter::tokenizer::Token;
use crate::interpreter::tokenizer::TokenType;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Program {
    body: Vec<Statement>,
}
struct Constant {
    value: Any,
}

struct StateDef {
    name: String,
    transitions: Vec<Transition>,
}

struct Transition {
    on: String,
    write: String,
    move_head: String,
    goto: String,
}

struct Expression {}

struct Call {}

struct Name {}

fn should_add_level(token: &Token) -> bool {
    match token.token_type {
        TokenType::StateSymbol => true,
        TokenType::Colon => true,
        TokenType::ArrowSymbol => true,
        _ => false,
    }
}

fn should_close_level(token: &Token) -> bool {
    match token.token_type {
        TokenType::NewLine => true,
        _ => false,
    }
}

pub fn get_abstract_syntax_tree(tokens: Vec<Token>) -> TokenNode {
    let root = TokenNode {
        token: Token {
            token_type: TokenType::Root,
            value: String::from(""),
        },
        children: Vec::new(),
        parent: None,
    };
    let current_node = &mut root;

    for token in tokens {
        let new_node = TokenNode {
            token,
            children: Vec::new(),
            parent: Some(Rc::new(RefCell::new(current_node.clone()))),
        };
        current_node
            .borrow_mut()
            .children
            .push(Rc::clone(&new_node));

        if should_add_level(&token) {
            current_node = Rc::clone(&new_node)
        } else if should_close_level(&token) {
            current_node = Rc::clone(&current_node.borrow().parent.as_ref().unwrap());
        }
    }

    root
}
