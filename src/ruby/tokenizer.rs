use itertools::PeekNth;
use std::str::Chars;
use crate::ruby::token::Token;

pub struct Tokenizer<'text> {
    chars: PeekNth<Chars<'text>>,
    tokens: Vec<Token>,
}

impl <'text> Tokenizer<'text> {
    pub fn new(text: &'text str) -> Self {
        Self {
            chars: itertools::peek_nth(text.chars()),
            tokens: vec![],
        }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        loop {
            self.whitespace();
            match self.chars.peek() {
                None => return self.tokens,
                Some(&c) => match c {
                    c if c.is_ascii_alphabetic() => self.keyword_or_identifier(),
                    _ => panic!("Unexpected character: {:#?}", c)
                }
            }
        }
    }

    fn keyword_or_identifier(&mut self) {
        // TODO -- Only keyword right now
        assert!(!self.chars.peek().unwrap().is_numeric());
        let mut name = String::new();

        while let Some(c) = self.chars.peek() {
            if c.is_ascii_alphanumeric() {
                name.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        match name.as_str() {
            "false" => self.tokens.push(Token::False),
            "true" => self.tokens.push(Token::True),
            "nil" => self.tokens.push(Token::Nil),
            _ => todo!("Not a keyword: {}", name)
        }
    }

    fn whitespace(&mut self) {
        while self.chars.peek().is_some_and(|c| c.is_whitespace()) {
            self.chars.next();
        }
    }
}

mod tests {
    use crate::ruby::token::Token;
    use crate::ruby::tokenizer::Tokenizer;
    use Token::*;

    #[test]
    pub fn empty() {
        let text = "";
        let expected_tokens: Vec<Token> = vec![];

        let test_tokens = Tokenizer::new(text).tokenize();

        assert_eq!(expected_tokens, test_tokens);
    }

    #[test]
    pub fn keywords() {
        {
            let text = "nil";
            let expected_tokens: Vec<Token> = vec![Nil];

            let test_tokens = Tokenizer::new(text).tokenize();

            assert_eq!(expected_tokens, test_tokens);
        }
        {
            let text = "true";
            let expected_tokens: Vec<Token> = vec![True];

            let test_tokens = Tokenizer::new(text).tokenize();

            assert_eq!(expected_tokens, test_tokens);
        }
        {
            let text = "true false";
            let expected_tokens: Vec<Token> = vec![True, False];

            let test_tokens = Tokenizer::new(text).tokenize();

            assert_eq!(expected_tokens, test_tokens);
        }
    }
}