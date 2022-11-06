//! 「Go言語でつくるインタプリッタ」に登場する言語「Monkey」のRust実装におけるlibクレート

/// 字句解析器のトークンを扱うモジュール
pub mod token;

use token::Token;

/// binクレートから呼び出される関数
pub fn run() {
    println!("Hello, world!");
    println!("Int : {}", Token::Int(112));
    println!("Ident : {}", Token::Ident("xyz".to_string()));
    println!("Eof : {}", Token::Eof);
    println!("Pllus : {}", Token::Plus);
}
