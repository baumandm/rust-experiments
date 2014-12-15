use std::io;

use stack::Stack;

mod stack;

fn main() {
    let mut my_stack = Stack::new();
    loop {
        print!("> ");
        let input = io::stdin().read_line().ok().expect("Failed to read line");
        let trimmed_input = input.trim_right_chars('\n');

        let mut split = trimmed_input.split_str(" ");

        match split.next().unwrap() {
            "q" => break,
            "push" => {
                match split.next() {
                    None => println!("! Provide a value to push !"),
                    Some(i) => {
                        let num = from_str(i);
                        match num {
                            None => println!("! Value must be numeric !"),
                            Some(i) => my_stack = my_stack.push(i)
                        }
                    }
                };
            },
            "pop" => {
                if my_stack.has_next() {
                    let (tail, val) = my_stack.pop();
                    println!("Popped: {}", val);
                    my_stack = tail;
                } else {
                    println!("! Can't pop any more !");
                }
            },
            _ => println!("! What?? (q/pop/push) !")
        };

        println!("Current Stack: {}", my_stack);
    }
    
    println!("Done!");
}
