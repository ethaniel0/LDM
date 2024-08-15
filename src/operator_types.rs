#[derive(Clone)]
pub struct OperatorStructure {

}
impl OperatorStructure {
    pub fn binary(operator: &String) -> Self {
        OperatorStructure{

        }
    }
    pub fn unary_left(operator: &String) -> Self {
        OperatorStructure{

        }
    }
    pub fn unary_right(operator: &String) -> Self {
        OperatorStructure{

        }
    }
    pub fn custom(operator: &String) -> Self {
        OperatorStructure{

        }
    }
}

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
    pub structure: OperatorStructure,
    pub returns: ValueType
}
impl Operator {
    pub fn new() -> Self{
        let token = "".to_owned();
        let structure = OperatorStructure::binary(&token);
        Operator {
            token,
            precedence: 0,
            structure,
            returns: ValueType::left_type()
        }
    }
}
