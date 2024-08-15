mod operators_tokenizer;
mod operator_types;
mod simple_tokenizer;
use std::fs;

use operators_tokenizer::parse_operators;
use simple_tokenizer::tokenize;

enum BracketedType {
    NameWithType,
    TypedName,
    Block,
    Expression,
    Name,
    Type,

    StructureWord
}

trait Definition {
    fn matches(source: &String, start: usize) -> bool;
    fn bracketed_type() -> BracketedType;
}

struct TokenDefinition {
    token: String,
    bracketed_type: BracketedType
}

struct Keyword {
    token: String,
    structure: Vec<TokenDefinition>
}


fn main() {

    let operators_filename = "/Users/ethanhorowitz/Desktop/LDM/ldm files/operators.ldm_lib";
    let operators_file_contents = fs::read_to_string(operators_filename).unwrap();
    let operators = parse_operators(&operators_filename.to_owned(), &operators_file_contents);

    let code_filename = "/Users/ethanhorowitz/Desktop/LDM/ldm files/source.ldm";
    let code_file_contents = fs::read_to_string(code_filename).unwrap();
    let tokens = tokenize(&code_file_contents, &operators);
    
    for token in tokens {
        println!("{}\t{:?}", token.token, token.token_type)
    }
}
