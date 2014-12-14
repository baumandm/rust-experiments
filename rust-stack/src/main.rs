use stack::Stack;

mod stack;

fn main() {
    let mut my_stack = Stack::new();
    println!("Current Stack: {}", my_stack.to_string());
    println!("Has Next? {}", my_stack.has_next());
    
    my_stack = my_stack.push(1);
    println!("Current Stack: {}", my_stack);

    my_stack = my_stack.push(2);
    my_stack = my_stack.push(3);
    println!("Current Stack: {}", my_stack);
    
    println!("Has Next? {}", my_stack.has_next());

    while my_stack.has_next() {

        let (tail, val) = my_stack.pop();
        println!("Popped: {}", val);

        my_stack = tail;
        println!("Current Stack: {}", my_stack);
    }
    
    println!("Done!");
}
