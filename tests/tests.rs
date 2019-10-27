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
        fn quickcheck_sort_u16(data: Vec<u16>) -> bool {
            let mut correct = data.clone();
            correct.sort_unstable();
            let mut testing = data.clone();
            count_sort::sort_u16(&mut testing);
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
    quickcheck! {
        fn quickcheck_sort_i16(data: Vec<i16>) -> bool {
            let mut correct = data.clone();
            correct.sort_unstable();
            let mut testing = data.clone();
            count_sort::sort_i16(&mut testing);
            correct == testing
        }
    }
}
