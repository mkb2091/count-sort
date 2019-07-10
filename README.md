# Count Sort

A fast sorting library implementing count sort algorithm which is O(n + k). Designed for very quickly sorting large amounts of data with small range of possible values. It now supports no_std

Currently only supports u8.

# Goals

Add support for more types including i8, u16 and i16

# Performance

Time to clone and sort randomly generated Vec<u8> of length specified below. Demonstrates for which lengths it is faster, and for which lengths the overhead is too much.

| length | sort_u8   | .sort()   | .sort_unstable() | dmsort    |
|--------|-----------|-----------|------------------|-----------|
| 0      | 9.3493 ns | 11.633 ns | 11.887 ns        | 11.427 ns |
| 1      | 24.791 ns | 25.148 ns | 25.700 ns        | 25.442 ns |
| 4      | 220.99 ns | 48.536 ns | 49.364 ns        | 110.13 ns |
| 16     | 373.35 ns | 223.30 ns | 238.12 ns        | 429.16 ns |
| 64     | 955.01 ns | 2.1638 us | 1.4000 us        | 1.8457 us |
| 256    | 2.1389 us | 11.445 us | 6.5242 us        | 7.3119 us |
| 1024   | 3.9392 us | 56.754 us | 27.414 us        | 30.193 us |
| 4096   | 7.4653 us | 255.55 us | 77.200 us        | 85.214 us |
| 16384  | 18.984 us | 1.0981 ms | 233.50 us        | 265.55 us |
| 65536  | 73.250 us | 4.6771 ms | 794.74 us        | 914.10 us |
| 262144 | 292.84 us | 19.938 ms | 3.1294 ms        | 3.4867 ms |

# Usage

Add dependency to Cargo.toml
```
[dependencies]
count_sort = "0.1"
```

And add the following to your code:

```rust
extern crate count_sort;

fn main () {
	let mut data: Vec<u8> = vec![252, 107, 81, 35, 185, 18, 175, 130, 37, 166];
	count_sort::sort_u8(&mut data);
	println!("{:?}", data);

}
```
