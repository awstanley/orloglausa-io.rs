# orloglausa input/output code (`orloglausa-io`)

Basic crate for handling input and output.  This is largely extracted and reworked from other code of mine, as I got sick of re-writing, re-working, and re-tuning it every time I needed it -- why not use crates if you can't use crates?

Note: `standard-rw` requires `std`, and leans on `std::io::Read` and `std::io::Write`.  This requires `std` (via `extern crate std`).

The basic aim here is to make reading and writing binary data as painless as possible from standard sources.

Endianness is not considered here; platform is treated as the same as the source data; reordering should happen before/after (if/as necessary).

## Traits

 - `StandardRead`/`StandardWrite`: `std::io` wrapping `Read` and `Write` (respectively) to read from data.  The outputs here are just `io::Result<()>`
 - `BinaryRead`/`BinaryWrite`: reading and writing from `&[u8]`, returning a `Result<u32, u32>` for read and write (where the `u32` values always indicate the number of bytes read or written).

## Features

 - `default`: Includes `binary-rw` and nothing else at this time;
 - `standard-rw`: `StandardRead` and `StandardWrite`, implementing them for the standard numeric types (`u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`);
 - `binary-rw`: `BinaryRead` and `BinaryWrite`, implementing them for the standard numeric types (`u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`).

## Licence

Released under the zlib/png licence (see `LICENCE`).