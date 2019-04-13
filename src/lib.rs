pub fn sort_u8(array: &mut Vec<u8>) {
    let mut table: [usize; 256] = [0; 256];
    for i in array.iter() {
        table[*i as usize] += 1;
    }
    let mut pos = 0;
    for (i, amount) in table.iter().enumerate() {
        for x in pos..(pos + amount) {
            array[x] = i as u8;
        }
        pos += amount;
    }
}
