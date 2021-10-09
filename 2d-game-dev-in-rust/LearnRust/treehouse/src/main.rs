use std::io::stdin;

fn main() {
    println!("What's your name?");
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
}
