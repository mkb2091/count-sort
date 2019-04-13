pub fn sort_u8(array: &mut Vec<u8>) {
    if array.len() < 256 {
        let mut table: [u8; 256] = [0; 256];
        for i in array.iter() {
            table[*i as usize] += 1;
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
        for i in array.iter() {
            table[*i as usize] += 1;
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
        for i in array.iter() {
            table[*i as usize] += 1;
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
