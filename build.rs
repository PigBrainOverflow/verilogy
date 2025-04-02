fn main() {
    lalrpop::process_root().unwrap();
    println!("Parser generated successfully.");
}