mod operators_tokenizer;
use std::fs;

use operators_tokenizer::parse_operators;

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

    let filename = "/Users/ethanhorowitz/Desktop/LDM/ldm files/operators.ldm_lib";
    let file_contents = fs::read_to_string(filename).unwrap();

    let tokenized = parse_operators(&filename.to_owned(), &file_contents);

    for operator in tokenized {
        println!("{}\t{}\t{}", operator.token, operator.precedence, operator.returns.name)
    }
    
}
