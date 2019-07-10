#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate count_sort;

fuzz_target!(|data: &[u8]| {
	let mut mutable_clone = data.to_vec();
    count_sort::sort_u8(&mut mutable_clone);
});
