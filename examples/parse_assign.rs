use verilogy::parser::Parser;

fn main() {
    let mut parser = Parser::new();
    let src = r#"
        module simple_adder(input a, input b, output c);
            assign c = a + b;
        endmodule
    "#;
    match parser.parse(src) {
        Ok(_) => {
            println!("Parsed successfully!");
            for (_, module) in parser.modules() {
                println!("{:?}", module);
            }
        }
        Err(e) => {
            eprintln!("Error parsing: {}", e);
        }
    }
}
