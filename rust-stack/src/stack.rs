use std::fmt;

pub struct Node {
    data: int,
    next: Option<Box<Node>>
}

pub struct Stack {
    head: Option<Box<Node>>
}

impl Stack {
    pub fn new () -> Stack {
        Stack { head: None }
    }

    pub fn has_next (&self) -> bool {
        match self.head {
            None => false,
            Some(_) => true
        }
    }

    pub fn push (self, data: int) -> Stack {
        match self.head {
            None => Stack {
                head: Some(box Node {
                    data: data, 
                    next: None
                })
            }, 
            Some (tail) => Stack {
                head: Some(box Node { 
                    data: data, 
                    next: Some(tail)
                })
            }
        }
    }

    pub fn pop (self) -> (Stack, int) {
        match self.head {
            None => (Stack::new(), -1),
            Some(stack) => {
                let value = stack.data;
                (Stack { head: stack.next }, value)
            }
        }
    }
}

impl fmt::Show for Stack {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn elements_to_str(n: &Node) -> String {
            //let node = *n;
            match (*n).next {
                None => format!("{}", (*n).data),
                Some(ref tail) => {
                    let ref t = **tail;
                    format!("{} -> {}", (*n).data, elements_to_str(t))
                }
            }
        }

        match self.head {
            None => write!(f, "Null"),
            Some(ref n) => write!(f, "{}", elements_to_str(&**n))
        }
        
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn new() {
        let stack = Stack::new();
        assert_eq!(true, stack.head.is_none());
    }

    #[test]
    fn has_next() {
        let stack = Stack::new();
        assert_eq!(false, stack.has_next());
    }   

    #[test]
    fn push() {
        let mut stack = Stack::new();
        stack = stack.push(1);
        assert_eq!(true, stack.has_next());
    }   
}
