use std::io;

enum Command {
    Add(i32, i32),
    Sub(i32, i32),
    Exit,
    Invalid,
}

fn parse(input: &str) -> Command {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.as_slice() {
        ["add", a, b] => Command::Add(a.parse().unwrap(), b.parse().unwrap()),
        ["sub", a, b] => Command::Sub(a.parse().unwrap(), b.parse().unwrap()),
        ["exit"] => Command::Exit,
        _ => Command::Invalid,
    }
}

fn execute(cmd: Command) -> bool {
    match cmd {
        Command::Add(a, b) => {
            println!("= {}", a + b);
            true
        }
        Command::Sub(a, b) => {
            println!("= {}", a - b);
            true
        }
        Command::Exit => false,
        Command::Invalid => {
            println!("Invalid command");
            true
        }
    }
}

fn main() {
    println!("Commands: add a b | sub a b | exit");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if !execute(parse(input.trim())) {
            break;
        }
    }
}
