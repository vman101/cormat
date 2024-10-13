use std::fs::File;
use std::io::prelude::*;

enum ControlType {
    If,
    ElIf,
    Else,
    For,
    While,
    Do,
    Switch,
    Case,
    Default,
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Preprocessor,
    Signature,
    Assignment,
    Symbol,
    String,
    ControlUnit,
    Character,
    Number,
    Type,
    Semicolon,
    CurlyOpen,
    CurlyClosing,
    ParanthesisOpen,
    ParanthesisClosed,
    SingleQuote,
    DoubleQuote,
    Tab,
    Newline,
    Space,
}

#[derive(Debug)]
struct Token<'a> {
    value: &'a str,
    kind: TokenType,
}

impl<'a> Token<'a> {
    fn determine_token_kind(value: &'a str) -> TokenType {
        if value.chars().all(char::is_numeric) {
            TokenType::Number
        } else if value.starts_with('#') {
            TokenType::Preprocessor
        } else if value == "\'" {
            TokenType::SingleQuote
        } else if value == "\"" {
            TokenType::DoubleQuote
        } else if value == "{" {
            TokenType::CurlyOpen
        } else if value == "}" {
            TokenType::CurlyClosing
        } else if value == "(" {
            TokenType::ParanthesisOpen
        } else if value == ")" {
            TokenType::ParanthesisClosed
        } else if value == "\t" {
            TokenType::Tab
        } else if value == "\n" {
            TokenType::Newline
        } else if value == " " {
            TokenType::Space
        } else if value == ";" {
            TokenType::Semicolon
        } else if value == "=" {
            TokenType::Assignment
        } else if C_TYPES.contains(&value) {
            TokenType::Type
        } else {
            TokenType::Symbol
        }
    }
    pub fn new(value: &'a str) -> Token<'a> {
        let kind = Token::determine_token_kind(value);
        Token { value, kind }
    }
}

fn hanlde_funciton_declaration(tokens: &mut [Token], i: &mut usize) {
    println!("Unimplemented");
}

fn handle_type_signed_unsigned(tokens: &mut [Token], i: &mut usize) {
    if tokens[*i + 1].kind == TokenType::Type
        && tokens[*i + 1].value != "struct"
        && tokens[*i + 1].value != "union"
        && tokens[*i + 1].value != "enum"
        && tokens[*i + 1].value != tokens[*i].value
    {
        if tokens[*i + 2].kind == TokenType::Symbol {
            if tokens[*i + 3].kind == TokenType::Semicolon {
                println!(
                    "Variable {} declared as type {}",
                    tokens[*i + 2].value,
                    tokens[*i].value
                );
            } else if tokens[*i + 3].kind == TokenType::ParanthesisOpen {
                hanlde_funciton_declaration(&mut tokens[*i..], i);
            } else if tokens[*i + 3].value == "=" {
                if tokens[*i + 4].kind == TokenType::Number {
                    if tokens[*i + 5].kind == TokenType::Semicolon {
                        println!(
                            "Variable {} declared as type {} and assigned value {}",
                            tokens[*i + 2].value,
                            tokens[*i].value,
                            tokens[*i + 4].value
                        );
                    } else {
                        eprintln!("Expected semicolon after variable assignment");
                    }
                } else {
                    eprintln!("Expected number after variable assignment");
                }
            }
        } else {
            eprintln!("Expected variable name after type declaration");
        }
    } else {
        eprintln!("Expected variable name after type declaration");
    }
}

fn handle_type(tokens: &mut [Token], i: &mut usize) {
    println!("In hanlde_type");
    if tokens[*i + 1].kind == TokenType::Tab {
    	println!("next was tab");
        if tokens[*i + 2].kind == TokenType::Symbol {
        	println!("then symbol");
            if tokens[*i + 3].kind == TokenType::Semicolon {
	           	println!("then semicolon");
                if tokens[*i + 4].kind == TokenType::Newline {
                    println!("variable {} correctly declared", tokens[*i + 2].value);
                    *i += 4;
                    return;
                }
            }
        }
    } else if tokens[*i + 1].kind == TokenType::Type {
        if tokens[*i].value == "unsigned" || tokens[*i].value == "signed" {
            handle_type_signed_unsigned(tokens, i);
        } else {
            eprintln!("Expected tab after Variable Declaration");
        }
    }
}

fn parse_tokens(tokens: &mut Vec<Token>) -> std::io::Result<()> {
	let mut i: usize = 0;

	while i < tokens.len() {
		match &mut tokens[i].kind {
			TokenType::Type => {
				println!("{:?}", tokens[i].kind);
				handle_type(&mut tokens[i..], &mut i)
			}
			_ => {
			    println!("{:?}", tokens[i].kind);
			}
		}
		i += 1;
	}
	Ok(())
}

static C_TYPES: &[&str] = &[
    "int", "char", "float", "double", "void", "short", "long", "unsigned", "signed", "struct",
    "union", "enum", "typedef",
];

static KEY_WORDS: &[&str] = &[
    "if", "else", "for", "while", "do", "switch", "case", "default",
];

fn main() -> std::io::Result<()> {
    let mut file = File::open("./test.c")?;
    let mut contents = String::new();
    let delimiter = "[]{}();\n\t \"\'";
    let mut current_chunk = String::new();

    file.read_to_string(&mut contents)?;
    let mut tokens: Vec<Token> = Vec::new();
    let mut strings_token: Vec<String> = Vec::new();
    for c in contents.chars() {
        if delimiter.contains(c) {
            if !current_chunk.is_empty() {
                strings_token.push(current_chunk.clone());
                current_chunk.clear();
            }
            strings_token.push(c.to_string());
        } else {
            current_chunk.push(c);
        }
    }

    let strings_token = &strings_token;
    for string in strings_token {
        tokens.push(Token::new(&string));
    }
    let mut i: usize = 0;
    while i < tokens.len() {
    	println!("{:?}", tokens[i].value.to_ascii_lowercase());
   	i += 1;
    }
    let _ = parse_tokens(&mut tokens);
    Ok(())
}