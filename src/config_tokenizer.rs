use core::fmt;

// for the tokenizer
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConfigTokenType {
    StructureWord,  // anything that doesn't match the other types
    Command,        // anything word that starts with a %
    Percent,        // % with no letters after it
    LParen,         // (
    RParen,         // )
    LBrace,         // {
    RBrace,         // }
}

#[derive(Clone)]
pub struct ConfigToken {
    pub token: String,
    pub token_type: ConfigTokenType,
    pub line: usize,
    pub index: usize
}

impl fmt::Display for ConfigToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}: {:?}>", self.token, self.token_type)
    }
}
impl fmt::Debug for ConfigToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}: {:?}>", self.token, self.token_type)
    }
}

struct ConfigTokenBuilder {
    running_str: String,
    index: usize,
    line: usize,
    in_cmd: bool,
    in_num: bool,
    in_non_alphnum: bool,
    pub tokens: Vec<ConfigToken>
}
impl ConfigTokenBuilder {
    pub fn new() -> Self {
        ConfigTokenBuilder {
            running_str: String::new(),
            index: 0,
            line: 1,
            in_cmd: false,
            in_num: false,
            in_non_alphnum: false,
            tokens: Vec::<ConfigToken>::new()
        }
    }

    fn push_keyword(&mut self) {
        if self.running_str == "%" {
            self.tokens.push(ConfigToken{token: self.running_str.clone(), token_type: ConfigTokenType::Percent, line: self.line, index: self.index});
        }
        else if self.in_cmd {
            self.tokens.push(ConfigToken{token: self.running_str.clone(), token_type: ConfigTokenType::Command, line: self.line, index: self.index});
        }
        else if self.running_str.len() > 0 {
            self.tokens.push(ConfigToken{token: self.running_str.clone(), token_type: ConfigTokenType::StructureWord, line: self.line, index: self.index});
        }
        self.running_str.clear();
        self.in_cmd = false;
        self.in_num = false;
        self.in_non_alphnum = false;
    }

    pub fn add_char(&mut self, c: char) {
        let alphabet = "_qwertyuiopasdfghjklzxcvbnmQWERTYUIOOPLKJHGFDSAZXCVBNM";
        let numbers = "1234567890";
        if c == '\n' {
            self.line += 1;
            self.index = 0;
        }
        else if c == ' ' || c == '\t' {
            self.push_keyword();
        }
        else if c == '%' {
            if self.running_str.len() == 0 {
                self.running_str.push(c);
            }
            else {
                self.push_keyword();
                self.running_str.push(c);
            }
        }
        else if "(){}".contains(c){
            self.push_keyword();
            self.tokens.push(ConfigToken{token: c.to_string(), token_type: match c {
                '(' => ConfigTokenType::LParen,
                ')' => ConfigTokenType::RParen,
                '{' => ConfigTokenType::LBrace,
                '}' => ConfigTokenType::RBrace,
                _ => panic!("Unknown bracketed character")
            }, line: self.line, index: self.index});
        }
        else if alphabet.contains(c) {
            if self.in_non_alphnum || self.in_num {
                self.push_keyword();
            }
            if self.running_str == "%" {
                self.in_cmd = true;
            }
            self.running_str.push(c);
            
        }
        else if numbers.contains(c) {
            if self.in_non_alphnum {
                self.push_keyword();
            }
            if self.running_str.len() == 0 {
                self.in_num = true;
            }
            self.running_str.push(c);
        }
        else {
            if c == '.' && self.in_num && !self.running_str.contains(".") {
                self.running_str.push(c);
                return;
            }
            if self.running_str.len() == 0 {
                self.in_non_alphnum = true;
                self.running_str.push(c);
                return;
            }

            if !self.in_non_alphnum {
                self.push_keyword();
            }
            self.running_str.push(c);
        }
    }
    pub fn finish(&mut self) {
        if self.running_str.len() > 0 {
            self.push_keyword();
        }
    }
}

pub fn config_to_tokens(source: &String) -> Vec<ConfigToken>{
    let mut builder = ConfigTokenBuilder::new();
    source.chars().for_each(|c| builder.add_char(c));
    builder.finish();
    builder.tokens
}


// for the actual command objects
#[derive(Debug, Clone)]
pub struct BracketItem {
    pub inner: Vec<ConfigToken>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandItemType {
    Word,
    Bracketed,
    Command
}

#[derive(Debug, Clone)]
pub struct CommandItem {
    pub item_type: CommandItemType,
    pub word: Option<ConfigToken>,
    pub bracketed: Option<BracketItem>,
    pub command: Option<Command>
}
impl CommandItem {
    pub fn new_word(word: ConfigToken) -> Self {
        CommandItem {
            item_type: CommandItemType::Word,
            word: Some(word),
            bracketed: None,
            command: None
        }
    }

    pub fn new_bracketed(bracketed: BracketItem) -> Self {
        CommandItem {
            item_type: CommandItemType::Bracketed,
            word: None,
            bracketed: Some(bracketed),
            command: None
        }
    }

    pub fn new_command(command: Command) -> Self {
        CommandItem {
            item_type: CommandItemType::Command,
            word: None,
            bracketed: None,
            command: Some(command)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<ConfigToken>,
    pub inner: Vec<CommandItem>
}

#[derive(Debug, Clone)]
pub struct ConfigItem {
    pub class_type: ConfigToken,
    pub specifiers: Vec<ConfigToken>,
    pub commands: Vec<Command>
}

struct ConfigItemBuilder<'a> {
    tokens: &'a Vec<ConfigToken>,
    index: usize,
}
impl<'a> ConfigItemBuilder<'a> {
    pub fn new(tokens: &'a Vec<ConfigToken>) -> Self {
        ConfigItemBuilder {
            tokens,
            index: 0
        }
    }

    fn get_next_class_type(&mut self) -> ConfigToken {
        if self.index >= self.tokens.len() {
            panic!("Expected a structure word, got end of file");
        }
        if self.tokens[self.index].token_type != ConfigTokenType::StructureWord {
            panic!("Expected a structure word, got {} on line {}", self.tokens[self.index].token, self.tokens[self.index].line);
        }
        let class_type = self.tokens[self.index].clone();
        self.index += 1;
        class_type
    }

    fn get_next_specifiers(&mut self, starting_line: usize) -> Vec<ConfigToken> {
        let mut specifiers = Vec::<ConfigToken>::new();
        while self.index < self.tokens.len() && self.tokens[self.index].token_type != ConfigTokenType::LParen {
            specifiers.push(self.tokens[self.index].clone());
            self.index += 1;
        }

        if specifiers.len() == 0 {
            panic!("Expected at least one specifier, got none on line {}", self.tokens[self.index].line);
        }
        if self.index >= self.tokens.len() {
            panic!("Expected a ( on line {}, got end of file", starting_line);
        }

        self.index += 1;
        specifiers
    }

    fn search_open_close<T: PartialEq>(&self, tokens: &Vec<T>, open: T, close: T, start_index: usize) -> usize {
        let mut open_count = 1;
        let mut index = start_index;
        while open_count > 0 && index < tokens.len() {
            if tokens[index] == open {
                open_count += 1;
            }
            else if tokens[index] == close {
                open_count -= 1;
            }
            index += 1;
        }
        index-1
    }

    fn parse_commands(&mut self, tokens: Vec<ConfigToken>) -> Vec<Command> {
        let mut commands = Vec::<Command>::new();
        let mut index = 0;
        while index < tokens.len() {
            let token = &tokens[index];
            if token.token_type != ConfigTokenType::Command {
                panic!("Unexpected token: {} on line {}", token.token, token.line);
            }
            let name = token.token[1..].to_owned();
            index += 1;

            if index >= tokens.len() {
                panic!("Expected a % at end of command on line {}", token.line);
            }

            if tokens[index].token_type == ConfigTokenType::Percent {
                // no args or inner, return command
                commands.push(Command{
                    name,
                    args: Vec::<ConfigToken>::new(),
                    inner: Vec::<CommandItem>::new()
                });
                index += 1;
                continue;
            }

            let mut args = Vec::<ConfigToken>::new();

            // get arguments
            if tokens[index].token_type == ConfigTokenType::LParen {
                index += 1;
                let next_index = self.search_open_close(
                    &tokens.iter().map(|t| t.token_type).collect::<Vec<ConfigTokenType>>(), 
                    ConfigTokenType::LParen, 
                    ConfigTokenType::RParen, 
                    index
                );
                if next_index >= tokens.len() || tokens[next_index].token_type != ConfigTokenType::RParen {
                    panic!("Expected a ), got end of file on line {}", token.line);
                }
                args = tokens[index..next_index].to_vec();
                index = next_index + 1;
            }


            // if end of command, return command
            if index >= tokens.len() || tokens[index].token_type == ConfigTokenType::Percent {
                commands.push(Command{
                    name,
                    args,
                    inner: Vec::<CommandItem>::new()
                });
                continue;
            }

            // get inner part of command
            if index >= tokens.len() {
                panic!("Expected a % at end of command on line {}", token.line);
            }

            if tokens[index].token_type == ConfigTokenType::Percent {
                // no inner, return command
                commands.push(Command{
                    name,
                    args: Vec::<ConfigToken>::new(),
                    inner: Vec::<CommandItem>::new()
                });
                index += 1;
                continue;
            }

            if index >= tokens.len() {
                panic!("Expected a % at end of command on line {}", token.line);
            }

            let mut inner = Vec::<CommandItem>::new();

            while index < tokens.len() && tokens[index].token_type != ConfigTokenType::Percent {
                let t = &tokens[index];
                if t.token_type == ConfigTokenType::Command {
                    // parse command within command
                    let new_index = self.search_open_close(
                        &tokens.iter().map(|t| t.token_type).collect::<Vec<ConfigTokenType>>(), 
                        ConfigTokenType::Command, 
                        ConfigTokenType::Percent, 
                        index+1
                    );

                    if new_index >= tokens.len() || tokens[new_index].token_type != ConfigTokenType::Percent {
                        panic!("Expected a %, got end of file on line {}", t.line);
                    }
                    let inner_tokens = tokens[index..new_index+1].to_vec();
                    index = new_index + 1;

                    let inner_commands = self.parse_commands(inner_tokens);
                    inner.push(CommandItem::new_command(inner_commands[0].clone()));
                }
                else if t.token_type == ConfigTokenType::LBrace {
                    // get inner bracketed item
                    index += 1;
                    let new_index = self.search_open_close(
                        &tokens.iter().map(|t| t.token_type).collect::<Vec<ConfigTokenType>>(), 
                        ConfigTokenType::LBrace, 
                        ConfigTokenType::RBrace, 
                        index
                    );
                    
                    if new_index >= tokens.len() || tokens[new_index].token_type != ConfigTokenType::RBrace {
                        panic!("Expected a }} on line {}, got end of file", t.line);
                    }

                    let inner_tokens = tokens[index..new_index].to_vec();
                    index = new_index + 1;

                    let inner_bracketed = BracketItem{inner: inner_tokens};
                    inner.push(CommandItem::new_bracketed(inner_bracketed));
                }
                else if t.token_type == ConfigTokenType::StructureWord {
                    inner.push(CommandItem::new_word(t.clone()));
                    index += 1;
                }
                else {
                    panic!("Unexpected token: {} on line {}:{}", t.token, t.line, t.index);
                }
            }

            if index >= tokens.len() {
                panic!("Expected a % at end of command on line {}", token.line);
            }
            index += 1;

            commands.push(Command{
                name,
                args,
                inner
            });
            
        }
        commands
    }

    // structure: <class_type> <specifiers> ( <commands> )
    pub fn next_config_item(&mut self) -> Option<ConfigItem> {
        if self.index >= self.tokens.len() {
            return None;
        }

        let class_type = self.get_next_class_type();

        let specifiers = self.get_next_specifiers(class_type.line);

        let next_index = self.search_open_close(&self.tokens.iter().map(|t| t.token_type).collect::<Vec<ConfigTokenType>>(), 
                                                        ConfigTokenType::LParen, 
                                                        ConfigTokenType::RParen, 
                                                        self.index);

        if next_index >= self.tokens.len() || self.tokens[next_index].token_type != ConfigTokenType::RParen {
            panic!("Expected a (, got end of file on line {}", class_type.line);
        }
        let inner_tokens = self.tokens[self.index..next_index].to_vec();
        self.index = next_index + 1;

        // let inner_tokens = self.get_inner_tokens(self.tokens[self.index].line);
        let commands = self.parse_commands(inner_tokens);

        Some(ConfigItem{
            class_type,
            specifiers,
            commands
        })
    }
}


pub fn parse_config(source: &String) -> Vec<ConfigItem> {
    let tokens = config_to_tokens(source);

    let mut builder = ConfigItemBuilder::new(&tokens);
    let mut items = Vec::<ConfigItem>::new();
    while let Some(item) = builder.next_config_item() {
        items.push(item);
    }

    items
}