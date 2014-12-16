#[deriving(Show)]
enum ParserError {
    ExtraOperands,
    UnrecognizedInput
}

type ParserResult = Result<f64, ParserError>;

pub fn parse(input: &str) -> ParserResult {
    let mut split = input.split_str(" ");
    let mut vec = Vec::new();

    loop {
        let i = match split.next() {
            None => break,
            Some(i) => i
        };
        
        println!("{}", i);
        match i {
            "+" => {
                let b: f64 = vec.pop().unwrap();
                let a: f64 = vec.pop().unwrap();
                vec.push(a + b);
            },
            "-" => {
                let b: f64 = vec.pop().unwrap();
                let a: f64 = vec.pop().unwrap();
                vec.push(a - b);
            },
            "*" => {
                let b: f64 = vec.pop().unwrap();
                let a: f64 = vec.pop().unwrap();
                vec.push(a * b);
            },
            "/" => {
                let b: f64 = vec.pop().unwrap();
                let a: f64 = vec.pop().unwrap();
                vec.push(a / b);
            },
            "%" => {
                let b: f64 = vec.pop().unwrap();
                let a: f64 = vec.pop().unwrap();
                vec.push(a.rem(b));
            },
            _ => {
                // Number
                let num: f64 = match from_str(i) {
                    None => return Err(ParserError::UnrecognizedInput),
                    Some(num) => num
                };
                vec.push(num);
            }
        };

        println!("Stack: {}", vec);
    }

    if vec.len() > 1 {
        Err(ParserError::ExtraOperands)
    } else {
        Ok(vec.pop().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::parse;

    #[test]
    fn number() {
        let result= parse("4");
        assert_eq!(4f64, result.unwrap());
    }

    #[test]
    fn add() {
        let result= parse("1 2 +");
        assert_eq!(3f64, result.unwrap());
    }

    #[test]
    fn subtract() {
        let result= parse("3 1 -");
        assert_eq!(2f64, result.unwrap());
    }

    #[test]
    fn multiply() {
        let result= parse("3 5 *");
        assert_eq!(15f64, result.unwrap());
    }

    #[test]
    fn divide() {
        let result= parse("5 2 /");
        assert_eq!(2.5f64, result.unwrap());
    }

    #[test]
    fn modulo() {
        let result= parse("20 8 %");
        assert_eq!(4f64, result.unwrap());
    }

    #[test]
    fn combo_a() {
        let result= parse("1.5 2 * 3 +");
        assert_eq!(6f64, result.unwrap());
    }

    #[test]
    fn combo_b() {
        let result= parse("4 2 1 2 + * -");
        assert_eq!(-2f64, result.unwrap());
    }
}
