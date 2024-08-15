
pub enum SimpleTokenType {
    Identifier,
    Number,
    String,
    Special
}

pub struct SimpleToken {
    token: String,
    token_type: SimpleTokenType
}

fn get_token(substr: &String, in_str: bool, in_number: bool) {
    let mut token_type = SimpleTokenType::Identifier;
    if in_str { token_type = SimpleTokenType::String; }
    else if in_number { token_type = SimpleTokenType::Number; }
    // else if ()
    

}

pub fn tokenize(source: &String) -> Vec<SimpleToken>{
    let mut tokens = Vec::<SimpleToken>::new();

    let mut in_str = false;
    let mut in_number = false;
    let mut in_float = false;
    let mut running_str = String::new();

    let index = 0;

    let numbers = "1234567890";
    let alphabet = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOOPLKJHGFDSAZXCVBNM";
    let whitespace = " \t\n";


    for c in source.chars() {
        if in_str {
            if c == '"' {
                in_str = false;
                tokens.push(SimpleToken {
                    token: running_str.to_owned(),
                    token_type: SimpleTokenType::String
                });
                running_str.clear();
            }
            else {
                running_str.push(c);
            }
            continue;
        }
        else if c == '"' {



            in_str = true;

        }
        


        if whitespace.contains(c) && running_str.len() > 0{

        }

        if alphabet.contains(c) {
            running_str.push(c);
        }


    }

    tokens
}

