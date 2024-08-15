use crate::operator_types::*;

fn parse_operator_name(line: &str, line_count: usize, filename: &String) -> String {
    let parts: Vec<&str> = line.split(" ").collect();
    if parts.len() == 1 {
        panic!("Operator not specified on line {}, {}", line_count, filename);
    }
    if parts.len() > 2 {
        panic!("Too many arguments supplied for operator on line {}, {}", line_count, filename);
    }
    parts[1].trim().to_owned()
}

fn parse_operator_precedence(line: &str, line_count: usize, filename: &String) -> usize {
    let parts: Vec<&str> = line.split("=").collect();
    if parts.len() == 1 {
        panic!("Precedence not specified on line {}, {}", line_count, filename);
    }
    if parts.len() > 2 {
        panic!("Too many arguments supplied for precedence on line {}, {}", line_count, filename);
    }
    let precedence = usize::from_str_radix(parts[1].trim(), 10);
    if precedence.is_err() {
        panic!("Precedence is not a positive integer on line {}, {}", line_count, filename);
    }
    precedence.unwrap()
}

fn parse_operator_structure(operator_token: &String, line: &str, line_count: usize, filename: &String) -> OperatorStructure {
    let parts = line.split_once('=');
    if parts.is_none() { panic!("Structure not specified on line {}, {}", line_count, filename); }

    let structure = parts.unwrap().1.trim();
    if structure.len() == 0 { panic!("Structure not specified on line {}, {}", line_count, filename); }

    if structure == "binary" { return OperatorStructure::binary(operator_token) }
    else if structure == "unary-left" { return OperatorStructure::unary_left(operator_token) }
    else if structure == "unary-right" { return OperatorStructure::unary_right(operator_token) }
    else if structure.starts_with("custom") {
        return OperatorStructure::binary(operator_token);
    }

    panic!("Unknown operator structure on line {}, {}", line_count, filename);
}

fn parse_operator_returns(line: &str, line_count: usize, filename: &String) -> ValueType {
    let parts = line.split_once('=');
    if parts.is_none() { panic!("Operator returns not specified on line {}, {}", line_count, filename); }

    let returns = parts.unwrap().1.trim();
    if returns.len() == 0 { panic!("Operator returns not specified on line {}, {}", line_count, filename); }

    // struture:
    // left_type, right_type, or none
    // type <type name>
    // var <var name>

    let split: Vec<&str> = returns.split(" ").collect();

    if returns == "left-type" { return ValueType::left_type() }
    else if returns == "right-type" { return ValueType::right_type() }
    else if returns == "none" { return ValueType::none() }
    else if split.len() == 2 {
        if split[0] == "type" {
            return ValueType::custom(&split[1].to_owned(), true);
        }
        else if split[0] == "var" {
            return ValueType::custom(&split[1].to_owned(), false);
        }
        panic!("Unknown returns type on line {}, {}", line_count, filename);
    }
    panic!("Unknown returns type on line {}, {}", line_count, filename);
}

pub fn parse_operators(filename: &String, source: &String) -> Vec<Operator>{
    let lines = source.lines();

    let mut operators = Vec::<Operator>::new();

    let mut line_count = 0;

    let mut operator = Operator::new();

    for line in lines {
        line_count += 1;
        if line.len() == 0 { continue; }

        let line = line.trim();

        if line.starts_with("operator") {
            // if operator exists, push it
            if operator.token.len() > 0 {
                operators.push(operator);
                operator = Operator::new();
            }
            operator.token = parse_operator_name(line, line_count, filename);
        }
        else if line.starts_with("precedence") {
            operator.precedence = parse_operator_precedence(line, line_count, filename);
        }
        else if line.starts_with("structure") {
            operator.structure = parse_operator_structure(&operator.token, line, line_count, filename);
        }
        else if line.starts_with("returns") {
            operator.returns = parse_operator_returns(line, line_count, filename);
        }
        
    }

    if operator.token.len() > 0 {
        operators.push(operator);
    }

    operators
}

