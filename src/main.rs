mod lexer;
mod parser;
mod transpiler;

fn main() {
    let input_arg = std::env::args().nth(1).expect("Must specify input file");
    let input_path = std::path::Path::new(&input_arg);
    let input_source_code = std::fs::read_to_string(input_path).expect("Failed to read input file");

    let tokens = lexer::lex(&input_source_code).expect("Failed to lex");
    let ast = parser::parse(tokens).expect("Failed to parse");
    let output_source_code = transpiler::transpile(ast).expect("Failed to transpile");

    let mut output_path = input_path.to_path_buf();
    output_path.set_extension("c");
    std::fs::write(output_path, output_source_code).expect("Faile to write output file");
}
