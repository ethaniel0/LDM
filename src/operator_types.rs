use crate::config_tokenizer::{Command, CommandItem, CommandItemType, ConfigItem};

#[derive(Clone)]
pub struct ValueType {
    pub name: String,
    pub is_type: bool
}
impl ValueType {
    pub fn left_type() -> Self {
        ValueType {
            name: "left_type".to_owned(),
            is_type: false
        }
    }
    pub fn right_type() -> Self {
        ValueType {
            name: "right_type".to_owned(),
            is_type: false
        }
    }
    pub fn none() -> Self {
        ValueType {
            name: "none".to_owned(),
            is_type: false
        }
    }
    pub fn custom(name: &String, is_type: bool) -> Self {
        ValueType {
            name: name.to_owned(),
            is_type
        }
    }
}

#[derive(Clone)]
pub struct Operator {
    pub token: String,
    pub precedence: usize,
    pub structure: Vec<CommandItem>,
    pub returns: ValueType
}

fn parse_operator_returns(line: &Command, line_count: usize) -> ValueType {
    if line.inner.len() == 0 { panic!("Operator returns not specified on line {}", line_count); }

    // struture:
    // left_type, right_type, or none
    // type <type name>
    // var <var name>

    let mut split = Vec::<String>::new();
    for item in &line.inner {
        if item.item_type != CommandItemType::Word { panic!("Returns type is not a word on line {}", line_count); }
        let w = item.word.as_ref().unwrap();
        split.push(w.token.clone());
    }

    if split.len() == 1 && split[0] == "left_type" { return ValueType::left_type() }
    else if split.len() == 1 && split[0] == "right_type" { return ValueType::right_type() }
    else if split.len() == 1 && split[0] == "none" { return ValueType::none() }
    else if split.len() == 2 {
        if split[0] == "type" {
            return ValueType::custom(&split[1].to_owned(), true);
        }
        else if split[0] == "var" {
            return ValueType::custom(&split[1].to_owned(), false);
        }
        panic!("Unknown return type {:?} on line {}", split, line_count);
    }
    panic!("Unknown return type {:?} on line {}", split, line_count);
}

pub fn config_to_operator(config_item: &ConfigItem) -> Option<Operator> {
    if config_item.class_type.token != "operator" {
        return None;
    }

    if config_item.specifiers.len() == 0 {
        panic!("Operator must have at least one specifier (e.g. name)");
    }

    let name = config_item.specifiers.get(0).unwrap().token.clone();

    let mut structure = Vec::new();
    let mut precedence: usize = 0;
    let mut returns = ValueType::left_type();

    for command in &config_item.commands {
        if command.name == "structure" {
            structure = command.inner.clone();
        }
        else if command.name == "precedence" {
            if command.inner.len() == 0 {  panic!("Precedence not specified for operator: {}", name); }
            let c = command.inner.get(0).unwrap();
            if c.item_type != CommandItemType::Word { panic!("Precedence is not a positive integer for operator {}", name); }
            let w = c.word.as_ref().unwrap();
            let p = w.token.parse::<usize>();
            if p.is_err() { panic!("Precedence is not a positive integer at line {}", w.line); }
            precedence = p.unwrap();
        }
        else if command.name == "returns" {
            returns = parse_operator_returns(command, command.line_number);
        }
        else {
            panic!("Unknown command in operator: {}", command.name);
        }
        
    }

    Some(Operator {
        token: name,
        structure,
        precedence,
        returns
    })
}
