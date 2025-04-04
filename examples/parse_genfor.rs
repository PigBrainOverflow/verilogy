use verilogy::parser::Parser;

fn main() {
    let mut parser = Parser::new();
    let src = r#"
        module simple_and(
            input[3:0] a,
            input[3:0] b,
            output[3:0] c
        );
            genvar i;
            generate
                for (i = 0; i < 4; i = i + 1) begin: and_gate
                    assign c[i] = a[i] & b[i];
                end
            endgenerate
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
