#[derive(Debug)]
pub enum Token {
    // separators
    CarrigeReturn,
    Comma,
    LeftParen,
    RightParen,

    // arithmetic ops
    Add,
    Subtract,
    Multiply,
    Divide,

    // relational ops
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,

    // identifiers
    Print,
    If,
    Then,
    Goto,
    Input,
    Let,
    Gosub,
    Return,
    Clear,
    List,
    Run,
    End,
    Var(char),

    // literals
    Number(String),
    String(String),
}

#[derive(Debug)]
pub enum LexerError {
    UnknownToken(String),
    InvalidString,
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut output = vec![];
    let mut first;
    let mut rest = input;
    while !rest.is_empty() {
        // ignored whitespace
        if rest.starts_with([' ', '\t']) {
            (_, rest) = rest.split_at(1);

        // separators
        } else if rest.starts_with('\n') {
            (_, rest) = rest.split_at(1);
            output.push(Token::CarrigeReturn);
        } else if rest.starts_with(',') {
            (_, rest) = rest.split_at(1);
            output.push(Token::Comma);
        } else if rest.starts_with('(') {
            (_, rest) = rest.split_at(1);
            output.push(Token::LeftParen);
        } else if rest.starts_with(')') {
            (_, rest) = rest.split_at(1);
            output.push(Token::RightParen);

        // arithmetic ops
        } else if rest.starts_with('+') {
            (_, rest) = rest.split_at(1);
            output.push(Token::Add);
        } else if rest.starts_with('-') {
            (_, rest) = rest.split_at(1);
            output.push(Token::Subtract);
        } else if rest.starts_with('*') {
            (_, rest) = rest.split_at(1);
            output.push(Token::Multiply);
        } else if rest.starts_with('/') {
            (_, rest) = rest.split_at(1);
            output.push(Token::Divide);

        // relational ops
        } else if rest.starts_with('=') {
            (_, rest) = rest.split_at(1);
            output.push(Token::Equals);
        } else if rest.starts_with("<>") {
            (_, rest) = rest.split_at(2);
            output.push(Token::NotEquals);
        } else if rest.starts_with('>') {
            (_, rest) = rest.split_at(1);
            output.push(Token::GreaterThan);
        } else if rest.starts_with(">=") {
            (_, rest) = rest.split_at(2);
            output.push(Token::GreaterThanEquals);
        } else if rest.starts_with('<') {
            (_, rest) = rest.split_at(1);
            output.push(Token::LessThan);
        } else if rest.starts_with("<=") {
            (_, rest) = rest.split_at(2);
            output.push(Token::LessThanEquals);

        // identifiers
        } else if rest.starts_with(|c: char| c.is_ascii_alphabetic()) {
            (first, rest) = split_identifier(rest);
            let token = match first {
                "PRINT" => Token::Print,
                "IF" => Token::If,
                "THEN" => Token::Then,
                "GOTO" => Token::Goto,
                "INPUT" => Token::Input,
                "LET" => Token::Let,
                "GOSUB" => Token::Gosub,
                "RETURN" => Token::Return,
                "CLEAR" => Token::Clear,
                "LIST" => Token::List,
                "RUN" => Token::Run,
                "END" => Token::End,
                s if (s.len() == 1) => Token::Var(s.chars().next().unwrap()),
                s => return Err(LexerError::UnknownToken(s.to_owned())),
            };
            output.push(token);

        // literals
        } else if rest.starts_with(|c: char| c.is_ascii_digit()) {
            (first, rest) = split_number(rest);
            output.push(Token::Number(first.to_owned()));
        } else if rest.starts_with('\"') {
            (first, rest) = split_string(rest).ok_or(LexerError::InvalidString)?;
            output.push(Token::String(first.to_owned()));

        // Unknown
        } else {
            // todo: does this crash on non ascii characters?
            return Err(LexerError::UnknownToken(rest.split_at(1).0.to_owned()));
        }
    }
    Ok(output)
}

fn split_identifier(input: &str) -> (&str, &str) {
    assert!(input.starts_with(|c: char| c.is_ascii_alphabetic()));
    match input.find(|c: char| !c.is_ascii_alphabetic()) {
        Some(i) => input.split_at(i),
        None => (input, ""),
    }
}

fn split_number(input: &str) -> (&str, &str) {
    assert!(input.starts_with(|c: char| c.is_ascii_digit()));
    match input.find(|c: char| !c.is_ascii_digit()) {
        Some(i) => input.split_at(i),
        None => (input, ""),
    }
}

fn split_string(input: &str) -> Option<(&str, &str)> {
    assert!(input.starts_with('\"'));
    input.split_at(1).1.split_once('\"')
}
