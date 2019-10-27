#![no_std]

macro_rules! sort {
    ($inputtype:ty, $storage_type:ty, $array:ident) => {{
    const is_unsigned: bool = <$inputtype>::min_value() == 0;
    const size: usize = ((is_unsigned as usize) * (<$inputtype>::max_value() as usize + 1))
    + ((!is_unsigned as usize) * (((<$inputtype>::max_value() as isize) - (<$inputtype>::min_value() as isize)) as usize + 1));
    let mut table: [$storage_type; size] = [0; size];
        for value in $array.iter() {
            unsafe {
                *table.get_unchecked_mut(if is_unsigned {*value as $storage_type}
                else {((*value as isize) - (<$inputtype>::min_value() as isize)) as $storage_type}) += 1;
            }
        }
        let mut pos = 0;
        for (i, amount) in table.iter().enumerate().filter(|(_, &amount)| amount > 0) {
            let end = *amount as $storage_type + pos;
            let i: $inputtype = if is_unsigned {
                i as $inputtype
            } else {
                (i as isize + (<$inputtype>::min_value() as isize)) as $inputtype
            };
            if *amount % 2 == 1 {
                unsafe {
                    *$array.get_unchecked_mut(pos) = i;
                }
                pos += 1;
            }
            while pos < end {
                unsafe {
                    *$array.get_unchecked_mut(pos) = i;
                    *$array.get_unchecked_mut(pos + 1) = i;
                }
                pos += 2;
            }
        }
    }};
}

#[inline]
pub fn sort_u8(array: &mut [u8]) {
    if array.len() < 2 {
    } else {
        let mut table = sort!(u8, usize, array);
    }
}

#[inline]
pub fn sort_i8(array: &mut [i8]) {
    if array.len() < 2 {
    } else {
        let mut table = sort!(i8, usize, array);
    }
}
