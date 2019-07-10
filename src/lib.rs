#[inline]
pub fn sort_u8(array: &mut Vec<u8>) {
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
            for x in pos..(pos + *amount as usize) {
                array[x] = i as u8;
            }
            pos += *amount as usize;
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
            for x in pos..(pos + *amount as usize) {
                array[x] = i as u8;
            }
            pos += *amount as usize;
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
            for x in pos..(pos + amount) {
                array[x] = i as u8;
            }
            pos += amount;
        }
    }
}
