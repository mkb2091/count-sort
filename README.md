# Count Sort

A fast sorting library implementing count sort algorithm which is O(n + k). Designed for very quickly sorting large amounts of data with small range of possible values.

Currently only supports u8.

# Goals

Add support for more types including i8, u16 and i16

# Performance

Time to clone and sort randomly generated Vec<u8> of length specified below. Demonstrates for which lengths it is faster, and for which lengths the overhead is too much.

| length | .sort()   | sort_u8   |
|--------|-----------|-----------|
| 0      | 11.549 ns | 9.7494 ns |
| 1      | 24.936 ns | 23.756 ns |
| 4      | 50.282 ns | 187.28 ns |
| 16     | 227.07 ns | 335.70 ns |
| 64     | 2.1468 us | 928.32 ns |
| 256    | 11.134 us | 2.4155 us |
| 1024   | 54.347 us | 4.3068 us |
| 4096   | 246.82 us | 8.2531 us |
| 16384  | 1.0590 ms | 25.837 us |
| 65536  | 4.4587 ms | 86.281 us |
| 262144 | 18.828 ms | 331.67 us |

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
