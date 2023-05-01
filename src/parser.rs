use crate::lexer::Token;

#[derive(Debug)]
pub struct Line(pub Number, pub Statement);

#[derive(Debug)]
pub enum Statement {
    Print(Vec<ExprOrString>),
    If(Expr, Relop, Expr, Box<Statement>),
    Goto(Expr),
    Input(Vec<char>),
    Let(char, Expr),
    Gosub(Expr),
    Return,
    Clear,
    List,
    Run,
    End,
}

#[derive(Debug)]
pub enum ExprOrString {
    Expr(Expr),
    String(String),
}

pub type Expr = Vec<(AddSubtract, Term)>;

#[derive(Debug)]
pub enum AddSubtract {
    Add,
    Subtract,
}

pub type Term = Vec<(MultiplyDivide, Factor)>;

#[derive(Debug)]
pub enum MultiplyDivide {
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Factor {
    Var(char),
    Number(Number),
    Expression(Expr),
}

pub type Number = u32;

#[derive(Debug)]
pub enum Relop {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
}

#[derive(Debug)]
pub enum ParserError {
    LineNumbersNotIncrementing,

    InvalidLineNumber,
    InvalidLine,

    EmptyStatement,
    InvalidStatement,

    EmptyExpr,

    EmptyTerm,

    InvalidFactorNumber,
    MissingRightParen,
    MissingLeftParen,

    InvalidIf,
    InvalidRelop,

    InvalidInput,

    InvalidLet,
}

pub fn parse(input: Vec<Token>) -> Result<Vec<Line>, ParserError> {
    let mut output = vec![];
    let mut line;
    let mut rest = input.as_slice();
    loop {
        match rest {
            [] => return Ok(output),
            [Token::CarrigeReturn, tokens @ ..] => rest = tokens,
            [tokens @ ..] => {
                (line, rest) = parse_line(tokens)?;
                if let Some(previous) = output.last() {
                    if line.0 <= previous.0 {
                        return Err(ParserError::LineNumbersNotIncrementing);
                    }
                }
                output.push(line);
            }
        }
    }
}

fn parse_line(tokens: &[Token]) -> Result<(Line, &[Token]), ParserError> {
    match tokens {
        [Token::Number(n), tokens @ ..] => {
            let number = str::parse(n).map_err(|_| ParserError::InvalidLineNumber)?;
            let (statement, rest) = parse_statement(tokens)?;
            Ok((Line(number, statement), rest))
        }
        _ => Err(ParserError::InvalidLine),
    }
}

fn parse_statement(tokens: &[Token]) -> Result<(Statement, &[Token]), ParserError> {
    let (head, rest) = tokens.split_first().ok_or(ParserError::EmptyStatement)?;
    let (statement, rest) = match head {
        Token::Print => parse_print(rest)?,
        Token::If => parse_if(rest)?,
        Token::Goto => parse_goto(rest)?,
        Token::Input => parse_input(rest)?,
        Token::Let => parse_let(rest)?,
        Token::Gosub => parse_gosub(rest)?,
        Token::Return => (Statement::Return, rest),
        Token::Clear => (Statement::Clear, rest),
        Token::List => (Statement::List, rest),
        Token::Run => (Statement::Run, rest),
        Token::End => (Statement::End, rest),
        _ => return Err(ParserError::InvalidStatement),
    };
    Ok((statement, rest))
}

fn parse_print(tokens: &[Token]) -> Result<(Statement, &[Token]), ParserError> {
    let mut output = vec![];
    let mut expr_or_string;
    let mut rest = tokens;
    loop {
        match rest {
            [] => return Ok((Statement::Print(output), rest)),
            [Token::CarrigeReturn, tokens @ ..] => return Ok((Statement::Print(output), tokens)),
            [tokens @ ..] => {
                (expr_or_string, rest) = parse_expr_or_string(tokens)?;
                output.push(expr_or_string);
            }
        }
    }
}

fn parse_expr_or_string(tokens: &[Token]) -> Result<(ExprOrString, &[Token]), ParserError> {
    match tokens {
        [Token::String(s), tokens @ ..] => Ok((ExprOrString::String(s.to_string()), tokens)),
        [tokens @ ..] => {
            let (expr, rest) = parse_expr(tokens)?;
            Ok((ExprOrString::Expr(expr), rest))
        }
    }
}

fn parse_expr(tokens: &[Token]) -> Result<(Expr, &[Token]), ParserError> {
    let mut output = vec![];
    let mut term;
    let mut rest = tokens;

    // First term has optional AddSubtract
    match rest {
        [] | [Token::CarrigeReturn, ..] => return Err(ParserError::EmptyExpr),
        [Token::Subtract, tokens @ ..] => {
            (term, rest) = parse_term(tokens)?;
            output.push((AddSubtract::Subtract, term));
        }
        [Token::Add, tokens @ ..] | [tokens @ ..] => {
            (term, rest) = parse_term(tokens)?;
            output.push((AddSubtract::Add, term));
        }
    }

    // All other terms requires AddSubtract
    loop {
        match rest {
            [Token::Subtract, tokens @ ..] => {
                (term, rest) = parse_term(tokens)?;
                output.push((AddSubtract::Subtract, term));
            }
            [Token::Add, tokens @ ..] => {
                (term, rest) = parse_term(tokens)?;
                output.push((AddSubtract::Add, term));
            }
            _ => return Ok((output, rest)),
        }
    }
}

fn parse_term(tokens: &[Token]) -> Result<(Term, &[Token]), ParserError> {
    let mut output = vec![];
    let mut factor;
    let mut rest = tokens;

    // First factor requires no MultiplyDivide
    match rest {
        [] | [Token::CarrigeReturn, ..] => return Err(ParserError::EmptyTerm),
        [tokens @ ..] => {
            (factor, rest) = parse_factor(tokens)?;
            output.push((MultiplyDivide::Multiply, factor));
        }
    }

    // All other factors requires MultiplyDivide
    loop {
        match rest {
            [Token::Multiply, tokens @ ..] => {
                (factor, rest) = parse_factor(tokens)?;
                output.push((MultiplyDivide::Multiply, factor));
            }
            [Token::Divide, tokens @ ..] => {
                (factor, rest) = parse_factor(tokens)?;
                output.push((MultiplyDivide::Divide, factor));
            }
            _ => return Ok((output, rest)),
        }
    }
}

fn parse_factor(tokens: &[Token]) -> Result<(Factor, &[Token]), ParserError> {
    match tokens {
        [Token::Var(v), tokens @ ..] => Ok((Factor::Var(*v), tokens)),
        [Token::Number(n), tokens @ ..] => {
            let number = str::parse(n).map_err(|_| ParserError::InvalidFactorNumber)?;
            Ok((Factor::Number(number), tokens))
        }
        [Token::LeftParen, tokens @ ..] => {
            let (expr, rest) = parse_expr(tokens)?;
            match rest {
                [Token::RightParen, tokens @ ..] => Ok((Factor::Expression(expr), tokens)),
                _ => Err(ParserError::MissingRightParen),
            }
        }
        _ => Err(ParserError::MissingLeftParen),
    }
}

fn parse_if(tokens: &[Token]) -> Result<(Statement, &[Token]), ParserError> {
    let (left_expr, rest) = parse_expr(tokens)?;
    let (relop, rest) = parse_relop(rest)?;
    let (right_expr, rest) = parse_expr(rest)?;
    if let [Token::Then, rest @ ..] = rest {
        let (statement, rest) = parse_statement(rest)?;
        let statement = Box::new(statement);
        Ok((Statement::If(left_expr, relop, right_expr, statement), rest))
    } else {
        Err(ParserError::InvalidIf)
    }
}

fn parse_relop(tokens: &[Token]) -> Result<(Relop, &[Token]), ParserError> {
    match tokens {
        [Token::Equals, tokens @ ..] => Ok((Relop::Equals, tokens)),
        [Token::NotEquals, tokens @ ..] => Ok((Relop::NotEquals, tokens)),
        [Token::GreaterThan, tokens @ ..] => Ok((Relop::GreaterThan, tokens)),
        [Token::GreaterThanEquals, tokens @ ..] => Ok((Relop::GreaterThanEquals, tokens)),
        [Token::LessThan, tokens @ ..] => Ok((Relop::LessThan, tokens)),
        [Token::LessThanEquals, tokens @ ..] => Ok((Relop::LessThanEquals, tokens)),
        _ => Err(ParserError::InvalidRelop),
    }
}

fn parse_goto(tokens: &[Token]) -> Result<(Statement, &[Token]), ParserError> {
    let (expr, rest) = parse_expr(tokens)?;
    Ok((Statement::Goto(expr), rest))
}

fn parse_input(tokens: &[Token]) -> Result<(Statement, &[Token]), ParserError> {
    let mut output = vec![];
    let mut rest = tokens;
    loop {
        match rest {
            [] => return Ok((Statement::Input(output), rest)),
            [Token::CarrigeReturn, tokens @ ..] => return Ok((Statement::Input(output), tokens)),
            [Token::Var(v), tokens @ ..] => {
                output.push(*v);
                rest = tokens
            }
            _ => return Err(ParserError::InvalidInput),
        }
    }
}

fn parse_let(tokens: &[Token]) -> Result<(Statement, &[Token]), ParserError> {
    if let [Token::Var(v), Token::Equals, tokens @ ..] = tokens {
        let (expr, rest) = parse_expr(tokens)?;
        Ok((Statement::Let(*v, expr), rest))
    } else {
        Err(ParserError::InvalidLet)
    }
}

fn parse_gosub(tokens: &[Token]) -> Result<(Statement, &[Token]), ParserError> {
    let (expr, rest) = parse_expr(tokens)?;
    Ok((Statement::Gosub(expr), rest))
}
