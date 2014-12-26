use std::io;
use std::iter;

mod zipper;

fn get_numbers_divisible_by(divisor: uint, max: uint) -> Vec<uint> {
    iter::count(1u, 1).
    filter(|&n| n % divisor == 0).
    take_while(|&n| n < max).
    collect()
}

fn get_factors(number: uint) -> Vec<uint> {
    iter::range(1u, number + 1).
    filter(|&n| number % n == 0).
    collect()
}

#[cfg(test)]
mod test {
    use std::num::Float;
    use std::slice::PartialEqSliceExt;

    #[test]
    fn get_numbers_divisible_by_2 () {
        let numbers = super::get_numbers_divisible_by(2, 100);
        for n in numbers.into_iter() {
            let half_n: f32 = n as f32 / 2.0;
            assert!(half_n == half_n.ceil());
        }
    }

    #[test]
    fn get_numbers_divisible_by_3 () {
        let numbers = super::get_numbers_divisible_by(3, 1000);

        let slice = numbers.as_slice();
        assert_eq!(3, slice[0]);
        assert_eq!(6, slice[1]);

        for n in numbers.iter() {
            let half_n = *n as f32 / 3.0;
            assert!(half_n == half_n.ceil());
        }
    }

    #[test]
    fn get_numbers_divisible_by_11 () {
        let numbers = super::get_numbers_divisible_by(11, 10000);

        let slice = numbers.as_slice();
        assert_eq!(11, slice[0]);
        assert!(slice.contains(&22));
        assert!(slice.contains(&33));
        assert!(!slice.contains(&34));
        assert!(slice.contains(&330));

        for n in numbers.iter() {
            let half_n = *n as f32 / 11.0;
            assert!(half_n == half_n.ceil());
        }
    }

    #[test]
    fn get_factors_of_2 () {
        let numbers = super::get_factors(8);
        let slice = numbers.as_slice();
        assert_eq!([1, 2, 4, 8], slice);
    }

    #[test]
    fn get_factors_of_18 () {
        let numbers = super::get_factors(18);
        let slice = numbers.as_slice();
        assert_eq!([1, 2, 3, 6, 9, 18], slice);
    }
}
