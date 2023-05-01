use crate::parser::{self, Relop};

#[derive(Debug)]
pub enum TranspileError {
    InvalidStatement,
}

pub fn transpile(input: Vec<parser::Line>) -> Result<String, TranspileError> {
    let mut output = "#include <stdio.h>\n\nint main() {\n".to_string();

    for line in input {
        output.push_str(&transpile_line(&line)?);
    }
    output.push_str("}");
    Ok(output)
}

fn transpile_line(input: &parser::Line) -> Result<String, TranspileError> {
    Ok(
        "line_".to_string()
            + &input.0.to_string()
            + ":\n"
            + &transpile_statement(&input.1)?
            + ";\n",
    )
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

fn transpile_print(input: &parser::Statement) -> Result<String, TranspileError> {
    if let parser::Statement::Print(items) = input {
        let mut output = "printf(\"".to_string();
        for item in items {
            output.push_str(match item {
                parser::ExprOrString::Expr(_) => "%d",
                parser::ExprOrString::String(_) => "%s",
            });
        }
        output.push_str("\"");
        for item in items {
            output.push_str(", ");
            output.push_str(&transpile_expr_or_string(item)?);
        }
        output.push_str(")");
        Ok(output)
    } else {
        Err(TranspileError::InvalidStatement)
    }
}

fn transpile_expr_or_string(input: &parser::ExprOrString) -> Result<String, TranspileError> {
    match input {
        parser::ExprOrString::Expr(e) => transpile_expr(e),
        parser::ExprOrString::String(s) => Ok("\"".to_string() + s + "\""),
    }
}

fn transpile_expr(input: &parser::Expr) -> Result<String, TranspileError> {
    let mut output = "(0".to_string();
    for (operation, term) in input {
        output.push_str(match operation {
            parser::AddSubtract::Add => "+",
            parser::AddSubtract::Subtract => "-",
        });
        output.push_str(&transpile_term(term)?);
    }
    output.push_str(")");
    Ok(output)
}

fn transpile_term(input: &parser::Term) -> Result<String, TranspileError> {
    let mut output = "(1".to_string();
    for (operation, factor) in input {
        output.push_str(match operation {
            parser::MultiplyDivide::Multiply => "*",
            parser::MultiplyDivide::Divide => "/",
        });
        output.push_str(&transpile_factor(factor)?);
    }
    output.push_str(")");
    Ok(output)
}

fn transpile_factor(input: &parser::Factor) -> Result<String, TranspileError> {
    Ok(match input {
        parser::Factor::Var(v) => v.to_string(),
        parser::Factor::Number(n) => n.to_string(),
        parser::Factor::Expression(e) => transpile_expr(e)?,
    })
}

fn transpile_if(input: &parser::Statement) -> Result<String, TranspileError> {
    todo!()
}

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

fn transpile_goto(input: &parser::Statement) -> Result<String, TranspileError> {
    todo!()
}

fn transpile_input(input: &parser::Statement) -> Result<String, TranspileError> {
    todo!()
}

fn transpile_let(input: &parser::Statement) -> Result<String, TranspileError> {
    todo!()
}

fn transpile_gosub(input: &parser::Statement) -> Result<String, TranspileError> {
    todo!()
}

fn transpile_return(input: &parser::Statement) -> Result<String, TranspileError> {
    todo!()
}

fn transpile_clear(input: &parser::Statement) -> Result<String, TranspileError> {
    todo!()
}

fn transpile_list(input: &parser::Statement) -> Result<String, TranspileError> {
    todo!()
}

fn transpile_run(input: &parser::Statement) -> Result<String, TranspileError> {
    todo!()
}

fn transpile_end(input: &parser::Statement) -> Result<String, TranspileError> {
    todo!()
}
