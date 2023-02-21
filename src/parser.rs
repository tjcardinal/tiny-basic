use crate::lexer;

enum Line {
    NumStatement(Number, Statement),
    Statement(Statement),
}

enum Statement {
    Print(ExprList),
    If(Expr, Relop, Expr, Box<Statement>),
    Goto(Expr),
    Input(VarList),
    Let(String, Expr),
    Gosub(Expr),
    Return,
    Clear,
    List,
    Run,
    End,
}

type ExprList = Vec<ExprOrString>;

enum ExprOrString {
    Expr(Expr),
    String(String),
}

type VarList = Vec<String>;

struct Expr {
    first: (Option<PlusMinus>, Box<Term>),
    rest: Vec<(PlusMinus, Box<Term>)>,
}

enum PlusMinus {
    Plus,
    Minus,
}

struct Term {
    first: Factor,
    rest: Vec<(Option<TimesDivide>, Factor)>,
}

enum TimesDivide {
    Times,
    Divide,
}

enum Factor {
    Var(char),
    Number(Number),
    Expression(Expr),
}

type Number = u32;

enum Relop {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
}

enum ParserError {}

fn parse(mut input: Vec<lexer::Token>) -> Result<Vec<Line>, ParserError> {
    while !input.is_empty() {
        match input.first() {
            Some(_) => todo!(),
            None => todo!(),
        }
    }
    Ok(vec![])
}
