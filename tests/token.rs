use monkey::token::Token;

#[test]
fn token_fmt() {
    assert_eq!(Token::Illegal.to_string(), "Illegal".to_string());
    assert_eq!(Token::Eof.to_string(), "Eof".to_string());
    assert_eq!(Token::Ident("aaa".to_string()).to_string(), "aaa".to_string());
    assert_eq!(Token::Assign.to_string(), "=".to_string());
    assert_eq!(Token::Plus.to_string(), "+".to_string());
    assert_eq!(Token::Comma.to_string(), ",".to_string());
    assert_eq!(Token::Semicolon.to_string(), ";".to_string());
    assert_eq!(Token::Lparen.to_string(), "(".to_string());
    assert_eq!(Token::Rparen.to_string(), ")".to_string());
    assert_eq!(Token::Lbrace.to_string(), "{".to_string());
    assert_eq!(Token::Rbrace.to_string(), "}".to_string());
    assert_eq!(Token::Function.to_string(), "function".to_string());
    assert_eq!(Token::Let.to_string(), "let".to_string());
}