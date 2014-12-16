use std::io;

mod parser;

fn main() {
    loop {
        print!("> ");
        let input = io::stdin().read_line().ok().expect("Failed to read line");
        let trimmed_input = input.trim_right_chars('\n');

        if trimmed_input == "q" {
            break;
        }

        match parser::parse(trimmed_input) {
            Err(why) => panic!("{}", why),
            Ok(result) => println!("Final Result: {}", result)
        }
    }

    println!("Done!");
}

