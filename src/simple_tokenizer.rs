use crate::operator_types::Operator;

#[derive(Debug)]
pub enum SimpleTokenType {
    Identifier,
    Number,
    String,
    Operator,
    Keyword,

    RBRACKET,
    LBRACKET,

    RPAREN,
    LPAREN
}

pub struct SimpleToken {
    pub token: String,
    pub token_type: SimpleTokenType
}

fn get_token(substr: &String, in_str: bool, in_number: bool) {
    let mut token_type = SimpleTokenType::Identifier;
    if in_str { token_type = SimpleTokenType::String; }
    else if in_number { token_type = SimpleTokenType::Number; }
    // else if ()
}

pub fn tokenize(source: &String, operators: &Vec<Operator>) -> Vec<SimpleToken>{
    let mut tokens = Vec::<SimpleToken>::new();

    let mut in_str = false;
    let mut in_number = false;
    let mut in_float = false;
    let mut in_identifier = false;
    let mut in_operator = false;
    let mut running_str = String::new();

    let mut index = 0;
    let mut line = 1;

    let numbers = "1234567890";
    let alphabet = "_qwertyuiopasdfghjklzxcvbnmQWERTYUIOOPLKJHGFDSAZXCVBNM";
    let whitespace = " \t\n";


    let mut possible_operators = operators.clone();

    let chars: Vec<char> = source.chars().collect();
    let mut char_ind = 0;

    while char_ind < chars.len(){
        let c = chars[char_ind];
        char_ind += 1;
        if c == '\n' {
            line += 1;
            index = 0;
        }
        index += 1;

        
        if running_str.len() == 0 {
            if whitespace.contains(c) { continue; }
            if c == '"' {
                in_str = true;
                continue;
            }
            if numbers.contains(c) {
                running_str.push(c);
                in_number = true;
                continue;
            }
            if alphabet.contains(c) {
                running_str.push(c);
                in_identifier = true;
                continue;
            }
            if "{}()".contains(c){
                tokens.push(SimpleToken {
                    token: c.to_string(),
                    token_type: match c {
                        '{' => SimpleTokenType::LBRACKET,
                        '}' => SimpleTokenType::RBRACKET,
                        '(' => SimpleTokenType::LPAREN,
                        ')' => SimpleTokenType::RPAREN,
                        _ => panic!("Unknown bracket at line {}, {}", line, index)
                    }
                });
                continue;
            }
            // must be an operator
            running_str.push(c);
            in_operator = true;


            // filter operators
            possible_operators.retain(|x| x.token.starts_with(&running_str));
            if possible_operators.len() == 0 {
                panic!("Unknown operator {} at line {}, {}", running_str, line, index);
            }
            continue;
        }
        
        if in_str {
            if c == '"' {
                tokens.push(SimpleToken {
                    token: running_str.to_owned(),
                    token_type: SimpleTokenType::String
                });
                running_str.clear();
                in_str = false;
                in_identifier = false;
                in_float = false;
                in_number = false;
                in_operator = false;
                possible_operators = operators.clone();
            }
            else {
                running_str.push(c);
            }
            continue;
        }

        else if in_number {
            if numbers.contains(c) {
                running_str.push(c);
            }
            else if c == '.' {
                if in_float {
                    panic!("Invalid number at line {}, {}", line, index);
                }
                running_str.push(c);
                in_float = true;
            }
            else if alphabet.contains(c) {
                panic!("Invalid number at line {}, {}", line, index);
            }
            else {
                tokens.push(SimpleToken {
                    token: running_str.to_owned(),
                    token_type: SimpleTokenType::Number
                });
                running_str.clear();
                in_str = false;
                in_identifier = false;
                in_float = false;
                in_number = false;
                in_operator = false;
                possible_operators = operators.clone();
                char_ind -= 1;
            }
            continue;
        }

        else if in_identifier {
            if alphabet.contains(c) || numbers.contains(c) {
                running_str.push(c);
            }
            else {
                tokens.push(SimpleToken {
                    token: running_str.to_owned(),
                    token_type: SimpleTokenType::Identifier
                });
                running_str.clear();
                in_str = false;
                in_identifier = false;
                in_float = false;
                in_number = false;
                in_operator = false;
                possible_operators = operators.clone();
                char_ind -= 1;
            }
            continue;
        }

        else if in_operator {
            // another operator character
            if !numbers.contains(c) && !alphabet.contains(c) && !whitespace.contains(c) {
                running_str.push(c);
                possible_operators.retain(|x| x.token.starts_with(&running_str));
                if possible_operators.len() == 0 {
                    panic!("Unknown operator {} at line {}, {}", running_str, line, index);
                }
            }
            else {
                // make sure operator is an exact match for an operator
                let mut found = false;
                for operator in &possible_operators {
                    if operator.token == running_str {
                        found = true;
                        break;
                    }
                }
                if !found {
                    panic!("Unknown operator {} at line {}, {}", running_str, line, index);
                }

                tokens.push(SimpleToken {
                    token: running_str.to_owned(),
                    token_type: SimpleTokenType::Operator
                });
                running_str.clear();
                in_str = false;
                in_identifier = false;
                in_float = false;
                in_number = false;
                in_operator = false;
                possible_operators = operators.clone();
                char_ind -= 1;
            }
        }
    }

    tokens
}

