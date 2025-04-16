use regex::Regex;
use std::fs;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Root,
    NewLine,
    StateSymbol,
    Identifier,
    Colon,
    Tab,
    ArrowSymbol,
    Null,
    Comma,
    MoveSymbol,
    RightSymbol,
    LeftSymbol,
    GotoSymbol,
    LParentheses,
    RParentheses,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}
impl Token {
    fn get_token(raw_token: &str) -> Token {
        let token_type = match raw_token {
            "state" => TokenType::StateSymbol,
            "goto" => TokenType::GotoSymbol,
            "move" => TokenType::MoveSymbol,
            "right" => TokenType::RightSymbol,
            "left" => TokenType::LeftSymbol,
            "->" => TokenType::ArrowSymbol,
            "(" => TokenType::LParentheses,
            ")" => TokenType::RParentheses,
            "," => TokenType::Comma,
            "null" => TokenType::Null,
            ":" => TokenType::Colon,

            _ => TokenType::Identifier,
        };
        Token {
            token_type,
            value: raw_token.to_string(),
        }
    }
    fn tokenize_line(line: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        if line.chars().nth(0) == Some('#') {
            return tokens;
        }

        if line.starts_with("    ") || line.starts_with("\t") {
            tokens.push(Token {
                token_type: TokenType::Tab,
                value: "\t".to_string(),
            });
        }

        let function_call_signature = Regex::new(r"([a-zA-Z0-9_]+)\((.+)\)").unwrap();
        if function_call_signature.is_match(&line) {
            let captures = function_call_signature.captures(&line).unwrap();
            let function_name = captures.get(1).unwrap().as_str();
            let function_args = captures.get(2).unwrap().as_str().split(",");
            tokens.push(Token::get_token(function_name));
            tokens.push(Token::get_token("("));
            tokens.extend(function_args.map(|arg| Token::get_token(arg)));
            tokens.push(Token::get_token(")"));
            return tokens;
        }

        let raw_tokens = line.split_whitespace();
        for raw_token in raw_tokens {
            let token = Token::get_token(raw_token);
            tokens.push(token);
        }
        tokens
    }
    fn tokenize(file_contents: String) -> Vec<Token> {
        let lowercase_contents = file_contents.to_lowercase();
        let mut tokens = Vec::new();
        for line in lowercase_contents.split("\n") {
            if line.is_empty() {
                tokens.push(Token {
                    token_type: TokenType::NewLine,
                    value: "\n".to_string(),
                });
                continue;
            }
            tokens.extend(Token::tokenize_line(line.to_string()));
            tokens.push(Token {
                token_type: TokenType::NewLine,
                value: "\n".to_string(),
            });
        }
        tokens
    }
}

pub fn parse_file(file_path: &str) -> Result<Vec<Token>, &'static str> {
    match fs::read_to_string(file_path) {
        Ok(contents) => Ok(Token::tokenize(contents.to_string())),
        Err(_) => Err("Error reading file"),
    }
}
