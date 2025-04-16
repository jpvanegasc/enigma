use std::string::String;

#[derive(Debug)]
pub enum TopLevelEntities {
    StateDef(StateDef),
    Expression(Expression),
}

#[derive(Debug)]
pub enum ConstantTypes {
    I(i32),
    S(&'static String),
}

#[derive(Debug)]
pub struct Program {
    pub body: Vec<TopLevelEntities>,
}
#[derive(Debug)]
pub struct Constant {
    pub value: ConstantTypes,
}

#[derive(Debug)]
pub struct StateDef {
    pub name: String,
    pub transitions: Vec<Transition>,
}

#[derive(Debug)]
pub struct Transition {
    pub on: String,
    pub write: String,
    pub move_head: String,
    pub goto: String,
}

#[derive(Debug)]
pub struct Expression {}

#[derive(Debug)]
pub struct Call {}

#[derive(Debug)]
pub struct Name {}
