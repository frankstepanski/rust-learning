fn main() {
    let subject = "World";
    // string interpolation is not built into Rust (need to use macros)
    // println, format! and write! are macros
    let greeting = format!("Hello, {}!", subject);
    println!("{}", greeting);
}
