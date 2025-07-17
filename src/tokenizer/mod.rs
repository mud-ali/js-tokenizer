#[derive(Debug, Clone)]
pub enum Token {
    VarDeclaration(String),
    VarName(String),
    Operator(String),
    FunctionCall(String),
    Argument(String),
    String(String),
    Number(String),
    ExpressionEnd,
    // TODO: add more
    Unknown(String),
}

#[derive(Debug, Clone)]
struct ParsingStatus {
    in_string: bool,
    in_parens: bool,
    after_assignment: bool,
    token_start: usize,
    token_end: usize,
    last_token: Token,
}

/// using a String containing the code and a list of delimiters, return an Abstract Syntax Tree as a vector of Tokens
pub fn split_tokens(code: String, delimiters: &[&str]) -> Vec<Token> {
    let mut abstract_syntax_tree = Vec::<Token>::new();
    let mut parsing_status = ParsingStatus {
        in_string: false,
        in_parens: false,
        after_assignment: false,
        token_start: 0,
        token_end: 0,
        last_token: Token::Unknown(String::new()),
    };
    let chars: Vec<char> = code.chars().collect();
    for (i, character) in chars.iter().enumerate() {
        parsing_status.token_end = i;

        if *character == '\"' {
            parsing_status.in_string = !parsing_status.in_string;
            if !parsing_status.in_string {
                continue;
            }
        }

        if should_split(*character, &code, &parsing_status, delimiters) {

            let token = (&code[parsing_status.token_start..parsing_status.token_end]).trim();
            println!("{}", token);
            if token != "" {
                abstract_syntax_tree.push(get_token_type(token, &parsing_status));
                parsing_status.last_token = abstract_syntax_tree.last().unwrap().clone();
            }

            if token == "=" {
                parsing_status.after_assignment = true;
            } 

            if matches!(parsing_status.last_token, Token::ExpressionEnd) {
                parsing_status.after_assignment = false;
            }
            
            parsing_status.token_start = i;
        }
    }

    abstract_syntax_tree.push(get_token_type(&code[parsing_status.token_end..], &parsing_status));

    abstract_syntax_tree
}

/// return whether or not to split tokens after the next character
fn should_split(
    subtoken: char,
    code: &String,
    context: &ParsingStatus,
    delimiters: &[&str],
) -> bool {
    if !context.in_string && delimiters.contains(&subtoken.to_string().as_str()) {
        // println!("<---{}", subtoken);
        return true;
    }
    if context.in_string && subtoken == '\"' {
        // println!("<---{}", subtoken);
        return true;
    }
    if context.token_end > code.len() {
        // println!("<---{}", subtoken);
        return true;
    }
    // println!("--->{}", subtoken);
    return false;
}

/// return what type of token should be used based on current parsing context and the value of the token
fn get_token_type(token_val: &str, context: &ParsingStatus) -> Token {
    if token_val.chars().nth(0) == Some('"') && token_val.chars().last() == Some('"') {
        return Token::String((&token_val[1..token_val.len()-1]).to_string());
    }

    let token_val = token_val.to_string();

    if ["+", "=", "-", "*", "/"].contains(&&token_val[..]) {
        return Token::Operator(token_val);
    } else if ["let", "var", "const", "function"].contains(&&token_val[..]) {
        return Token::VarDeclaration(token_val);
    }

    match &token_val[..].parse::<f64>() {
        Ok(_val) => {
            return Token::Number(token_val);
        }
        Err(_) => (),
    }

    if matches!(context.last_token, Token::VarDeclaration(_)) {
        let Token::VarDeclaration(ref last_val) = context.last_token else {
            return Token::VarName(token_val);
        };
        if last_val == "function" {
            return Token::FunctionCall(token_val);
        }
        return Token::VarName(token_val);
    }
    
    if token_val == ";" {
        return Token::ExpressionEnd;
    }

    if context.after_assignment {
        return Token::VarName(token_val);
    } else {
        println!("before = {}",token_val);
    }

    return Token::Unknown(token_val);
}
