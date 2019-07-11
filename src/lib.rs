#![no_std]

#[inline]
pub fn sort_u8(array: &mut [u8]) {
    if array.len() < 2 {
    } else {
        let mut table: [usize; 256] = [0; 256];
        for value in array.iter() {
            unsafe {
                *table.get_unchecked_mut(*value as usize) += 1;
            }
        }
        let mut pos = 0;
        for (i, amount) in table.iter().enumerate().filter(|(_, &amount)| amount > 0) {
            let end = *amount as usize + pos;
            if *amount % 2 == 1 {
                unsafe {
                    *array.get_unchecked_mut(pos) = i as u8;
                }
                pos += 1;
            }
            while pos < end {
                unsafe {
                    *array.get_unchecked_mut(pos) = i as u8;
                    *array.get_unchecked_mut(pos + 1) = i as u8;
                }
                pos += 2;
            }
        }
    }
}
