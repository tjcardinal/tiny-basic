use crate::parser::{self, Relop};

#[derive(Debug)]
pub enum TranspileError {}

pub fn transpile(input: Vec<parser::Line>) -> Result<String, TranspileError> {
    let mut output = String::new();
    for line in input {
        output += &transpile_line(&line)?;
    }
    Ok(output)
}

fn transpile_line(input: &parser::Line) -> Result<String, TranspileError> {
    Ok("line_".to_string() + &input.0.to_string() + ": " + &transpile_statement(&input.1)? + "\n")
}
fn transpile_statement(input: &parser::Statement) -> Result<String, TranspileError> {
    match input {
        parser::Statement::Print(_) => transpile_print(input),
        parser::Statement::If(_, _, _, _) => transpile_if(input),
        parser::Statement::Goto(_) => transpile_goto(input),
        parser::Statement::Input(_) => transpile_input(input),
        parser::Statement::Let(_, _) => transpile_let(input),
        parser::Statement::Gosub(_) => transpile_gosub(input),
        parser::Statement::Return => transpile_return(input),
        parser::Statement::Clear => transpile_clear(input),
        parser::Statement::List => transpile_list(input),
        parser::Statement::Run => transpile_run(input),
        parser::Statement::End => transpile_end(input),
    }
}
fn transpile_print(input: &parser::Statement) -> Result<String, TranspileError> {}
fn transpile_expr_or_string(input: &parser::ExprOrString) -> Result<String, TranspileError> {}
fn transpile_expr(input: &parser::Expr) -> Result<String, TranspileError> {}
fn transpile_term(input: &parser::Term) -> Result<String, TranspileError> {}
fn transpile_factor(input: &parser::Factor) -> Result<String, TranspileError> {}
fn transpile_if(input: &parser::Statement) -> Result<String, TranspileError> {}
fn transpile_relop(input: &parser::Relop) -> Result<String, TranspileError> {
    Ok(match input {
        Relop::Equals => "==",
        Relop::NotEquals => "!=",
        Relop::GreaterThan => ">",
        Relop::GreaterThanEquals => ">=",
        Relop::LessThan => "<",
        Relop::LessThanEquals => "<=",
    }
    .to_string())
}
fn transpile_goto(input: &parser::Statement) -> Result<String, TranspileError> {}
fn transpile_input(input: &parser::Statement) -> Result<String, TranspileError> {}
fn transpile_let(input: &parser::Statement) -> Result<String, TranspileError> {}
fn transpile_gosub(input: &parser::Statement) -> Result<String, TranspileError> {}
fn transpile_return(input: &parser::Statement) -> Result<String, TranspileError> {}
fn transpile_clear(input: &parser::Statement) -> Result<String, TranspileError> {}
fn transpile_list(input: &parser::Statement) -> Result<String, TranspileError> {}
fn transpile_run(input: &parser::Statement) -> Result<String, TranspileError> {}
fn transpile_end(input: &parser::Statement) -> Result<String, TranspileError> {}
