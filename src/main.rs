use std::io::Write;

fn main() {
    let version: &str = "1.0.1";

    let mut accumulator: i16 = 0;
    println!("Deadfish Interpreter v{}", version);
    
    loop {
        print!(">> ");
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("read stdin");

        let mut textPrinted: bool = false;

        for c in input.chars() {

            match c {
                'i' => accumulator = accumulator.wrapping_add(1),
                'd' => accumulator = accumulator.wrapping_sub(1),
                's' => accumulator = accumulator.wrapping_pow(2),
                'o' => println!("{}", accumulator),
                'c' => {
                    let character = char::from_u32(accumulator as u32);

                    match character {
                        Some(c) => {
                            print!("{}", c);
                            textPrinted = true;
                        }
                        None => {}
                    }
                }
                ';' => accumulator = 0,
                'h' => break,
                _ => {}
            }

            if accumulator == -1 || accumulator == 256 {
                accumulator = 0;
            }
        }

        if textPrinted {
            textPrinted = false;
            println!("");
        }
    }
}
