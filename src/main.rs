mod operators_tokenizer;
mod keywords_tokenizer;
mod operator_types;
mod simple_tokenizer;
mod config_tokenizer;
use std::fs;

use keywords_tokenizer::{config_to_keyword, Keyword};
use operators_tokenizer::parse_operators_from_file;




// struct LibItems {
//     keywords: Vec<Keyword>,
//     operators: Vec<Operator>
// }


fn main() {

    let operators_filename = "/Users/ethanhorowitz/Desktop/LDM/ldm files/operators.ldm_lib";
    let operators = parse_operators_from_file(&operators_filename.to_owned());

    let keywords_filename = "/Users/ethanhorowitz/Desktop/LDM/ldm files/keywords.ldm_lib";
    let source = fs::read_to_string(keywords_filename).unwrap();
    let config_items = config_tokenizer::parse_config(&source);
    let keywords = config_items.iter()
                                    .filter(|item| item.class_type.token == "keyword")
                                    .map(|item| config_to_keyword(item).unwrap())
                                    .collect::<Vec<Keyword>>();

    // let code_filename = "/Users/ethanhorowitz/Desktop/LDM/ldm files/source.ldm";
    // let code_file_contents = fs::read_to_string(code_filename).unwrap();
    // let tokens = tokenize(&code_file_contents, &operators);

    for item in keywords {
        println!("{:?}", item.name);
    }
}
