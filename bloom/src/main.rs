use std::io;

use bloomfilter::BloomFilter;

mod bloomfilter;

#[allow(dead_code)]
fn main() {

    let m = read_uint("m", 32);
    let k = read_uint("k", 3);

    let mut bloom_filter = BloomFilter::new_with_options(m, k);
    println!("Initializing bloom filter (m: {}, k: {})", bloom_filter.get_m(), bloom_filter.get_k());

    loop {
        print!("> ");
        let input = io::stdin().read_line().ok().expect("Failed to read line");

        let x: &[_] = &['\n', ' '];
        let trimmed_input = input.trim_chars(x);

        let mut split = trimmed_input.split_str(" ");

        match split.next().unwrap() {
            "quit" | "exit" => break,
            "add" => {
                match split.next() {
                    None => println!("Provide a value to add"),
                    Some(i) => {
                        bloom_filter.add(i);
                        println!("Added: {}", i);
                    }
                };
            },
            "test" => {
                match split.next() {
                    None => println!("Provide a value to test"),
                    Some(i) => {
                        let result = bloom_filter.test(i);

                        println!("Test Result: {}", result);
                    }
                };
            },
            _ => println!("Available Commands: quit | add <string> | test <string>")
        };

        //println!("Current Stack: {}", my_stack);
    }

    println!("Done!");
}

fn read_uint(label: &str, default: uint) -> uint {
    loop {
        println!("Please provide a value for: {} (default: {})", label, default);
        print!("> ");
        let input = io::stdin().read_line().ok().expect("Failed to read line");

        let x: &[_] = &['\n', ' '];
        let trimmed_input = input.trim_chars(x);

        if trimmed_input.len() == 0 {
            return default;
        }

        let num = from_str(trimmed_input);
        match num {
            None => println!("Invalid input (non-numeric)"),
            Some(i) => return i
        }
    }
}
