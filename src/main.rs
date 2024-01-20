use std::env;
use std::fs;

#[derive(Debug)]
enum TokenType {
    // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier, String, Number,

  // Keywords.
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  EOF
}

#[derive(Debug)]
struct Token<'a> {
    kind: TokenType,
    // lexeme is the word
    lexeme: Option<&'a str>,
    // literal is the character
    literal: Option<char>,
    line: usize 
}

struct Scanner<'a> {
    start_position: usize,
    line: usize,
    current_position: usize,
    source: &'a str,
    tokens: Vec<Token<'a>>
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Scanner<'a> {
        Scanner { current_position: 0, start_position: 0, line: 1, source, tokens: Vec::new() }
    }
    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start_position = self.current_position;
            self.scan_token();
        }

        self.tokens.push(Token { kind: TokenType::EOF, lexeme: None, literal: None, line: self.line });
    }
    fn print_tokens(&mut self) {
        self.scan_tokens();
        for token in &self.tokens  {
            println!("{:?}", token);
        }
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        println!("Scanning token {:?}", c);
        match c {
            Some("(") => self.add_token(TokenType::LeftParen, None),
            Some(")") => self.add_token(TokenType::RightParen, None),
            Some("{") => self.add_token(TokenType::LeftBrace, None),
            Some("}") => self.add_token(TokenType::RightBrace, None),
            Some(",") => self.add_token(TokenType::Comma, None),
            Some(".") => self.add_token(TokenType::Dot, None),
            Some("-") => self.add_token(TokenType::Minus, None),
            Some("+") => self.add_token(TokenType::Plus, None),
            Some(";") => self.add_token(TokenType::Semicolon, None),
            Some("*") => self.add_token(TokenType::Star, None),
            
            Some("!") => {
                match self.compare_next("=") {
                    true => self.add_token(TokenType::BangEqual, None),
                    false => self.add_token(TokenType::Bang, None)
                }
            },
            Some("=") => {
                match self.compare_next("=") {
                    true => self.add_token(TokenType::EqualEqual, None),
                    false => self.add_token(TokenType::Equal, None)
                }
            },
            Some("<") => {
                match self.compare_next("=") {
                    true => self.add_token(TokenType::LessEqual, None),
                    false => self.add_token(TokenType::Less, None)
                }
            },
            Some(">") => {
                match self.compare_next("=") {
                    true => self.add_token(TokenType::GreaterEqual, None),
                    false => self.add_token(TokenType::Greater, None)
                }
            },
            Some("/") => {
                match self.compare_next("/") {
                    // it means line is a comment, only advance scanner to next line.
                    true => {
                        let mut next_option = self.peek();
                        while let Some(cha) = next_option {
                            println!("Scanning comment token {:?}", cha);
                            match cha {
                                // it should break the while loop
                                "\n" => next_option = None,
                                _ => next_option = self.advance()
                            }
                        }
                    },
                    false => self.add_token(TokenType::Slash, None)
                }
            },
            // create logger instance to display line and other specs
            Some(c) => {
                let is_numeric = c.chars().next().unwrap().is_numeric();

                if is_numeric {
                    self.number()
                }

                panic!("Unexpected character.")
            },
            _ => panic!("Unexpected character.")
        }
    }
    fn number(&mut self) {
        let mut next_value = self.peek();
        let is_numeric = next_value.unwrap().chars().next().unwrap().is_numeric();
        
        while is_numeric {
            self.advance();
        }

        next_value = self.peek();

        match next_value {
            Some(".") => {
                let mut next_val = self.advance();
                let mut is_numeric = next_val.unwrap().chars().next().unwrap().is_numeric();

                while is_numeric {
                    next_val = self.advance();
                    is_numeric = next_val.unwrap().chars().next().unwrap().is_numeric();
                }
            },
            _ => todo!()
        }
        // look information about strings and borrowing, I don't think is required since it is only used 
        // for storage purposes.
        let c = self.source.chars().collect()[self.current_position];

        // Based on rust design I should change some ways to do things, I know what values need.
        self.add_token(TokenType::Number, c);
    }
    fn peek(&self) -> Option<&str> {
        if self.is_at_end() { return Some("\0") }
        let old_current = self.current_position;
        let current = self.current_position + 1;
        return self.source.get(old_current..current);
    }
    fn pick_lexeme(&self) -> Option<&str> {
        if self.is_at_end() { return Some("\0") }
        
        return self.source.get(self.start_position..self.current_position);
    }
    fn compare_next(&mut self, expected: &str) -> bool {
        if self.is_at_end() {
            return false
        }
        let next_ch = self.advance();
        match next_ch {
            Some(exp) => {
                println!("Comparing value {:?} and expected {:?}", exp, expected);
                if exp == expected { return true } else { return false }
            },
            _ => false
        }
    }
    fn advance(&mut self) -> Option<&str> {
        let old_current = self.current_position;
        self.current_position += 1;
        return self.source.get(old_current..self.current_position);
    }
    fn add_token(&mut self, kind: TokenType, literal: Option<char>) {
        let lexeme = self.source.get(self.start_position..self.current_position);
        self.tokens.push(Token {kind, lexeme, literal: literal, line: self.line})
    }
    fn is_at_end(&self) -> bool {
        self.current_position >= self.source.len()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        panic!("Add a filename param e.g -- [filename].[ext]");
    }

    let filename = &args[1];
    
    let content = fs::read_to_string(filename)
        .expect("Should have been able to read the file");


    let mut scanner = Scanner::new(&content);

    //  move occurs because `self.tokens` has type `Vec<Token<'_>>`, which does not implement the `Copy` trait
    scanner.print_tokens();

    // println!("program -> {:?}", p);

    // println!("Filename is -> {}", filename);
    // dbg!(args);
}