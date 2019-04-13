use count_sort;

extern crate rand;
#[cfg(test)]
mod tests {
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
