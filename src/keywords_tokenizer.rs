use crate::config_tokenizer::{CommandItem, ConfigItem};

enum BracketedType {
    Type,
    Typed,
    Block,
    Expression,
    Name,
    Typename,

    StructureWord,
    None
}

trait Definition {
    fn matches(source: &String, start: usize) -> bool;
    fn bracketed_type() -> BracketedType;
}

pub struct Keyword {
    pub name: String,
    pub structure: Vec<CommandItem>
}

pub fn config_to_keyword(config_item: &ConfigItem) -> Option<Keyword> {
    if config_item.class_type.token != "keyword" {
        return None;
    }

    if config_item.specifiers.len() == 0 {
        panic!("Keyword must have at least one specifier (e.g. name)");
    }

    let name = config_item.specifiers.get(0).unwrap().token.clone();

    let mut structure = Vec::new();

    for command in &config_item.commands {
        if command.name != "structure" {
            panic!("Unknown command in keyword: {}", command.name);
        }
        structure = command.inner.clone();
    }

    Some(Keyword {
        name,
        structure
    })
}

