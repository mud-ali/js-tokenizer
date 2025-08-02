use crate::tokenizer::Token;
use colored::{ColoredString, Colorize};

pub fn highlight(syntax_tree: Vec<Token>) -> Vec<ColoredString> {
    let mut indentation = String::new();
    let single_indent = "    ";
    syntax_tree
        .into_iter()
        .map(|token| {
            match token {
                Token::VarDeclaration(val) => format!("{}{}", indentation, val).blue(),
                Token::VarName(val) => val.purple(),
                Token::Operator(val) => val.white(),
                Token::Number(val) => val.red(),
                Token::String(val) => format!("\"{}\"", val).green(),
                Token::ExpressionEnd => ";\n".white(),
                Token::Argument(val) => val.white(),
                Token::FunctionCall(val) => format!("{}{}", indentation, val).white(),
                Token::Unknown(val) => val.white(),
                Token::Paren(val) => val.blue(),
                Token::ScopeDefiner(val) => {
                    if val == "{" {
                        let colored = format!("\n{} {{\n",indentation).green();
                        indentation += single_indent;
                        return colored;
                    }
                    indentation.truncate(indentation.len() - single_indent.len());
                    format!("{}}}\n", indentation).green()
                }
            }
        })
        .collect()
}
