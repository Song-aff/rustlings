#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    println!("{}", data.to_string().to_uppercase());
    string_uppercase(data);
}
