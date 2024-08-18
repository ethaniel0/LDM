mod keywords_tokenizer;
mod operator_types;
mod simple_tokenizer;
mod config_tokenizer;
use core::panic;
use std::fs;

use config_tokenizer::CommandItem;
use keywords_tokenizer::{config_to_keyword, Keyword};
use operator_types::{config_to_operator, Operator};
use simple_tokenizer::{tokenize, LibItems};


fn main() {

    // let operators_filename = "/Users/ethanhorowitz/Desktop/LDM/ldm files/operators.ldm_lib";
    // let operators = parse_operators_from_file(&operators_filename.to_owned());

    let keywords_filename = "/Users/ethanhorowitz/Desktop/LDM/ldm files/std.ldm_lib";
    let source = fs::read_to_string(keywords_filename).unwrap();
    let config_items = config_tokenizer::parse_config(&source);

    let mut operators = Vec::<Operator>::new();
    let mut keywords = Vec::<Keyword>::new();
    let mut make_variable_structures = Vec::<Vec<CommandItem>>::new();

    for item in &config_items {
        match item.class_type.token.as_str() {
            "operator" => {
                let operator = config_to_operator(item).unwrap();
                operators.push(operator);
            },
            "keyword" => {
                let keyword = config_to_keyword(item).unwrap();
                keywords.push(keyword);
            },
            "make_variable" => {
                let mut structure = Vec::new();
                for command in &item.commands {
                    if command.name != "structure" {
                        panic!("Unknown command in make_variable: {}", command.name);
                    }
                    structure = command.inner.clone();
                }
                make_variable_structures.push(structure);
            },
            _ => {
                panic!("Unknown class type: {}", item.class_type.token);
            }
        }
    }

    let code_filename = "/Users/ethanhorowitz/Desktop/LDM/ldm files/source.ldm";
    let code_file_contents = fs::read_to_string(code_filename).unwrap();

    let lib_items = LibItems {
        keywords,
        operators
    };

    let tokens = tokenize(&code_file_contents, &lib_items);

    for item in tokens {
        println!("{} \t\t {:?}", item.token, item.token_type);
    }
}
