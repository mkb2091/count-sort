# Count Sort

A fast sorting library implementing count sort algorithm which is O(n + k). Designed for very quickly sorting large amounts of data with small range of possible values.

Currently only supports u8.

# Goals

Add support for more types including i8, u16 and i16

# Performance

Time to clone and sort 10 randomly generated Vec<u8> of length specified below

| length | .sort()   | sort_u8   |
|--------|-----------|-----------|
| 0      | 77.827 ns | 2.2363 us |
| 1      | 254.34 ns | 2.4040 us |
| 4      | 311.05 ns | 2.6972 us |
| 16     | 1.0484 us | 3.2623 us |
| 64     | 14.810 us | 5.2870 us |
| 256    | 111.79 us | 11.182 us |
| 1024   | 632.43 us | 51.417 us |
| 4096   | 3.1092 ms | 84.417 us |
| 16384  | 11.990 ms | 251.44 us |
| 65536  | 52.786 ms | 906.54 us |
| 262144 | 219.08 ms | 3.6449 ms |

![Default Sort](benches/default_sort_report.svg)
![count_sort::sort_u8](benches/sort_u8_report.svg)

# Usage

Add dependency to Cargo.toml
```
[dependencies]
count_sort = "0.1.0"
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
