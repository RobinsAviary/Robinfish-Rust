use std::io::Write;

fn main() {
    let mut accumulator: i32 = 0;
    loop {
        print!(">> ");
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("read stdin");

        for c in input.chars() {

            match c {
                'i' => accumulator += 1,
                'd' => accumulator -= 1,
                's' => accumulator = accumulator.pow(2),
                'o' => println!("{}", accumulator),
                ';' => accumulator = 0,
                _ => {}
            }

            if accumulator == -1 || accumulator == 256 {
                accumulator = 0;
            }
        }
    }
}
