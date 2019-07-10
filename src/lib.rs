#![no_std]

#[inline]
pub fn sort_u8(array: &mut [u8]) {
    if array.len() < 2 {
    } else {
        let mut table: [usize; 256] = [0; 256];
        let mut x = 0;
        if array.len() % 2 == 1 {
            unsafe {
                *table.get_unchecked_mut(*array.get_unchecked(0) as usize) += 1;
            }
            x += 1;
        }
        while x < array.len() {
            unsafe {
                *table.get_unchecked_mut(*array.get_unchecked(x) as usize) += 1;
                *table.get_unchecked_mut(*array.get_unchecked(x + 1) as usize) += 1;
            }
            x += 2;
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
