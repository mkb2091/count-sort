#![no_std]

macro_rules! sort {
    ($inputtype:ty, $storage_type:ty, $array:ident) => {{
        const IS_UNSIGNED: bool = <$inputtype>::min_value() == 0;
        const SIZE: usize = ((IS_UNSIGNED as usize) * (<$inputtype>::max_value() as usize + 1))
            + ((!IS_UNSIGNED as usize)
                * (((<$inputtype>::max_value() as isize) - (<$inputtype>::min_value() as isize))
                    as usize
                    + 1));
        let mut table: [$storage_type; SIZE] = [0; SIZE];
        for value in $array.iter() {
            unsafe {
                *table.get_unchecked_mut(if IS_UNSIGNED {
                    *value as $storage_type
                } else {
                    ((*value as isize) - (<$inputtype>::min_value() as isize)) as $storage_type
                }) += 1;
            }
        }
        let mut pos = 0;
        for (i, amount) in table.iter().enumerate().filter(|(_, &amount)| amount > 0) {
            let end = *amount as $storage_type + pos;
            let i: $inputtype = if IS_UNSIGNED {
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
        sort!(u8, usize, array);
    }
}

#[inline]
pub fn sort_u16(array: &mut [u16]) {
    if array.len() < 2 {
    } else {
        sort!(u16, usize, array);
    }
}

#[inline]
pub fn sort_i8(array: &mut [i8]) {
    if array.len() < 2 {
    } else {
        sort!(i8, usize, array);
    }
}

#[inline]
pub fn sort_i16(array: &mut [i16]) {
    if array.len() < 2 {
    } else {
        sort!(i16, usize, array);
    }
}
