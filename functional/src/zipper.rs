
fn zipper<T, U: Iterator<T>, V: Iterator<T>>(mut iter1: U, mut iter2: V) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    let mut iter2_finished: bool = false;

    loop {
        match iter1.next() {
            Some(i) => result.push(i),
            None => break
        };

        if iter2_finished {
            continue;
        }

        match iter2.next() {
            Some(i2) => result.push(i2),
            None => iter2_finished = true           
        };
    }

    result
}


#[cfg(test)]
mod test {
    use std::iter;

    #[test]
    fn zipper_1 () {
        let i1 = iter::range_inclusive(1i,5);
        let i2 = iter::count(100, 100).take(5);

        let numbers = super::zipper(i1, i2);
        let slice = numbers.as_slice();
        assert_eq!([1, 100, 2, 200, 3, 300, 4, 400, 5, 500], slice);
    }
}
