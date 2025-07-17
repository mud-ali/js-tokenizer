use crate::tokenizer::Token;
use colored::{ColoredString, Colorize};

pub fn highlight(syntax_tree: Vec<Token>) -> Vec<ColoredString> {
    let mut output = Vec::new();
    for token in syntax_tree {
        if matches!(token, Token::VarDeclaration(_)) {
            let Token::VarDeclaration(ref token_val) = token else {
                continue;
            };
            output.push(token_val.blue());
        } else if matches!(token, Token::VarName(_)) {
            let Token::VarName(ref token_val) = token else {
                continue;
            };
            output.push(token_val.purple());
        } else if matches!(token, Token::Operator(_)) {
            let Token::Operator(ref token_val) = token else {
                continue;
            };
            output.push(token_val.white());
        } else if matches!(token, Token::ExpressionEnd) {
            output.push(";\n".white());
        } else if matches!(token, Token::Number(_)) {
            let Token::Number(ref token_val) = token else {
                continue;
            };
            output.push(token_val.red());
        } else if matches!(token, Token::String(_)) {
            let Token::String(ref token_val) = token else {
                continue;
            };
            output.push(format!("\"{}\"", token_val).green());
        }
    }
    return output;
}
