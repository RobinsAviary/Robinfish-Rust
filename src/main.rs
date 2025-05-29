use std::io::Write;

fn main() {
    let mut accumulator: i32 = 0;
    loop {
        print!(">> ");
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");

        for c in input.chars() {
            if c == 'i' {
                accumulator += 1;
            }

            if c == 'd' {
                accumulator -= 1;
            }

            if c == 's' {
                accumulator = accumulator.pow(2);
            }

            if c == 'o' {
                println!("{}", accumulator);
            }

            if c == ';' {
                accumulator = 0;
            }

            if accumulator == -1 || accumulator == 256 {
                accumulator = 0;
            }
        }

        println!("{}", input);
    }
}
