extern crate rand;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    quickcheck! {
        fn quickcheck_sort_u8(data: Vec<u8>) -> bool {
            let mut correct = data.clone();
            correct.sort_unstable();
            let mut testing = data.clone();
            count_sort::sort_u8(&mut testing);
            correct == testing
        }
    }
    quickcheck! {
        fn quickcheck_sort_i8(data: Vec<i8>) -> bool {
            let mut correct = data.clone();
            correct.sort_unstable();
            let mut testing = data.clone();
            count_sort::sort_i8(&mut testing);
            correct == testing
        }
    }
    #[test]
    fn test_sort_u8() {
        for _ in 0..1000 {
            let length: u16 = rand::random();
            let mut data: Vec<u8> = Vec::with_capacity(length as usize);
            for _ in 0..(length as usize) {
                data.push(rand::random());
            }
            let mut first_clone = data.clone();
            count_sort::sort_u8(&mut first_clone);
            data.sort();
            assert_eq!(data, first_clone);
        }
    }

    #[test]
    fn test_sort_i8() {
        for _ in 0..1000 {
            let length: u16 = rand::random();
            let mut data: Vec<i8> = Vec::with_capacity(length as usize);
            for _ in 0..(length as usize) {
                data.push(rand::random());
            }
            let mut first_clone = data.clone();
            count_sort::sort_i8(&mut first_clone);
            data.sort();
            assert_eq!(data, first_clone);
        }
    }
}
