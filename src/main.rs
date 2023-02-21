mod interpreter;
mod lexer;
mod parser;
mod transpiler;

fn main() {
    let asdf = lexer::lex("PRINT \t123\n LET \"asdf123\" IF A 123Z");
    println!("{asdf:?}");
}
