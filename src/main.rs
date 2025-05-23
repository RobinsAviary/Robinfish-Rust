use std::io::Write;

fn main() {
    let mut accumulator: i32 = 0;

    print!(">> ");
    let _ = std::io::stdout().flush();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");

    for c in input.chars() {
        if c == 'i' {
            accumulator += 1;
        }
    }

    println!("{}", input);
}
