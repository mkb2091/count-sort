# Count Sort

A fast sorting library implementing count sort algorithm which is O(n + k). Designed for very quickly sorting large amounts of data with small range of possible values.

Currently only supports u8.

# Goals

Add support for more types including i8, u16 and i16

# Performance

| length | .sort()   | sort_u8   |
|--------|-----------|-----------|
| 1      | 254.34 ns | 2.4830 us |
| 4      | 311.05 ns | 2.8181 us |
| 16     | 1.0484 us | 3.4905 us |
| 64     | 14.810 us | 5.8713 us |
| 256    | 111.79 us | 11.704 us |
| 1024   | 632.43 us | 53.338 us |
| 4096   | 3.1092 ms | 86.189 us |
| 16384  | 11.990 ms | 244.76 us |
| 65536  | 52.786 ms | 968.00 us |
| 262144 | 219.08 ms | 3.8404 ms |

![Default Sort](https://github.com/mkb2091/count-sort/benches/default_sort_report.svg)
![count_sort::sort_u8](https://github.com/mkb2091/count-sort/benches/sort_u8_report.svg)

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