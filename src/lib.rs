#![no_std]

#[inline]
pub fn sort_u8(array: &mut [u8]) {
    if array.len() < 2 {
    } else if array.len() < 256 {
        let mut table: [u8; 256] = [0; 256];
        let mut x = 0;
        if array.len() % 2 == 1 {
            table[array[0] as usize] += 1;
            x += 1;
        }
        while x < array.len() {
            table[array[x] as usize] += 1;
            table[array[x + 1] as usize] += 1;
            x += 2;
        }
        let mut pos = 0;
        for (i, amount) in table.iter().enumerate().filter(|(_, &amount)| amount > 0) {
            let end = *amount as usize + pos;
            if *amount % 2 == 1 {
                array[pos] = i as u8;
                pos += 1;
            }
            while pos < end {
                array[pos] = i as u8;
                array[pos + 1] = i as u8;
                pos += 2;
            }
        }
    } else if array.len() < 65536 {
        let mut table: [u16; 256] = [0; 256];
        let mut x = 0;
        if array.len() % 2 == 1 {
            table[array[0] as usize] += 1;
            x += 1;
        }
        while x < array.len() {
            table[array[x] as usize] += 1;
            table[array[x + 1] as usize] += 1;
            x += 2;
        }
        let mut pos = 0;
        for (i, amount) in table.iter().enumerate().filter(|(_, &amount)| amount > 0) {
            let end = *amount as usize + pos;
            if *amount % 2 == 1 {
                array[pos] = i as u8;
                pos += 1;
            }
            while pos < end {
                array[pos] = i as u8;
                array[pos + 1] = i as u8;
                pos += 2;
            }
        }
    } else {
        let mut table: [usize; 256] = [0; 256];
        let mut x = 0;
        if array.len() % 2 == 1 {
            table[array[0] as usize] += 1;
            x += 1;
        }
        while x < array.len() {
            table[array[x] as usize] += 1;
            table[array[x + 1] as usize] += 1;
            x += 2;
        }
        let mut pos = 0;
        for (i, amount) in table.iter().enumerate().filter(|(_, &amount)| amount > 0) {
            let end = *amount as usize + pos;
            if *amount % 2 == 1 {
                array[pos] = i as u8;
                pos += 1;
            }
            while pos < end {
                array[pos] = i as u8;
                array[pos + 1] = i as u8;
                pos += 2;
            }
        }
    }
}
