#![allow(dead_code)]
use std::fmt;

/// 字句解析器で使うトークンを表すenum
pub enum Token {
    /// 未知の文字列を検出した場合のトークン
    Illegal,
    /// ファイルの終わりを表すトークン
    Eof,
    
    /// 識別子
    Ident(String),
    /// 整数リテラル
    Int(i64),
    
    /// 代入演算子"="
    Assign,
    /// 算術加算演算子"+"
    Plus,

    /// カンマ","
    Comma,
    /// セミコロン";""
    Semicolon,

    /// 左丸カッコ"("
    Lparen,
    /// 右丸カッコ")"
    Rparen,
    /// 左波カッコ"{"
    Lbrace,
    /// 右波カッコ"}"
    Rbrace,

    /// キーワード"function"
    Function,
    /// キーワード"let"
    Let,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Token::Illegal => write!(f, "Illegal"),
            Token::Eof => write!(f, "Eof"),
            Token::Ident(str) => write!(f, "{}", str),
            Token::Int(n) => write!(f, "{}", n),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
            Token::Lbrace => write!(f, "{{"),
            Token::Rbrace => write!(f, "}}"),
            Token::Function => write!(f, "function"),
            Token::Let => write!(f, "let"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Token;

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
}