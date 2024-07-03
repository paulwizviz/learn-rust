# Integers

Rust has the following built-in integer types:

| size | signed | Unsigned |
| --- | --- | --- |
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

The permitted literals for these types are:

| Number literals | Example |
| --- | --- |
| Decimal | 98_222 |
| Hex | 0xff |
| Octal | 0o77 |
| Binary | 0b1111_0000 |
| Byte (u8 only) | b'A' |

Here is are [working examples](./src/main.rs).