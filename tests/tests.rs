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
    #[test]
    fn it_works() {
        for _ in 0..1000 {
            let mut data: Vec<u8> = Vec::new();
            let length: u16 = rand::random();;
            for _ in 0..(length as usize) {
                data.push(rand::random());
            }
            let mut first_clone = data.clone();
            count_sort::sort_u8(&mut first_clone);
            data.sort();
            assert_eq!(data, first_clone);
        }
    }
}
