pub mod token;

use token::Token;

pub fn run() {
    println!("Hello, world!");
    println!("Int : {}", Token::Int(112));
    println!("Ident : {}", Token::Ident("xyz".to_string()));
    println!("Eof : {}", Token::Eof);
    println!("Pllus : {}", Token::Plus);
}
