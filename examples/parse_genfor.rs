use verilogy::parser::Parser;

fn main() {
    let mut parser = Parser::new();
    let src = r#"
        module simple_and #(
            parameter WIDTH = 8
        )
        (
            input[WIDTH-1:0] a,
            input[WIDTH-1:0] b,
            output[WIDTH-1:0] c
        );
            genvar i;
            generate
                for (i = 0; i < WIDTH; i = i + 1) begin: and_gate
                    assign c[i] = a[i] & b[i];
                end
            endgenerate
        endmodule
    "#;
    match parser.parse(src) {
        Ok(_) => {
            println!("Parsed successfully!");
            for (_, module) in parser.modules() {
                println!("{}", serde_json::to_string_pretty(&module).unwrap());
            }
        }
        Err(e) => {
            eprintln!("Error parsing: {}", e);
        }
    }
}
