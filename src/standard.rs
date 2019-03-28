//! Standard

use std::io::{self, Read, Write};
use std::mem;

/// Standard Read is a trait implementing a fast-read for a given type
/// wrapping `std::io::Read`.
pub trait StandardRead {
    /// Reads instance from a given source.
    fn standard_read<R: Read>(&mut self, source: &mut R) -> io::Result<()>;
}

/// Standard WRite is a trait implementing a fast-write for a given type
/// wrapping `std::io::Write`.
pub trait StandardWrite {
    /// Writes instance to a given sink.
    fn standard_write<W: Write>(&self, sink: &mut W) -> io::Result<()>;
}

impl StandardRead for u8 {
    fn standard_read<R: Read>(&mut self, source: &mut R) -> io::Result<()> {
        let v: &mut [u8; 1] = unsafe { mem::transmute(self) };
        source.read_exact(&mut v[..])?;
        Ok(())
    }
}

impl StandardWrite for u8 {
    fn standard_write<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        let v: &[u8; 1] = unsafe { mem::transmute(self) };
        sink.write_all(&v[..])?;
        Ok(())
    }
}

impl StandardRead for i8 {
    fn standard_read<R: Read>(&mut self, source: &mut R) -> io::Result<()> {
        let v: &mut [u8; 1] = unsafe { mem::transmute(self) };
        source.read_exact(&mut v[..])?;
        Ok(())
    }
}

impl StandardWrite for i8 {
    fn standard_write<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        let v: &[u8; 1] = unsafe { mem::transmute(self) };
        sink.write_all(&v[..])?;
        Ok(())
    }
}

impl StandardRead for bool {
    fn standard_read<R: Read>(&mut self, source: &mut R) -> io::Result<()> {
        let mut b: u8 = 0;
        b.standard_read(source)?;
        *self = b == 1;
        Ok(())
    }
}

impl StandardWrite for bool {
    fn standard_write<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        let mut b: u8 = 0;
        if *self == true {
            b = 1;
        }
        b.standard_write(sink)?;
        Ok(())
    }
}

impl StandardRead for u16 {
    fn standard_read<R: Read>(&mut self, source: &mut R) -> io::Result<()> {
        let v: &mut [u8; 2] = unsafe { mem::transmute(self) };
        source.read_exact(&mut v[..])?;
        Ok(())
    }
}

impl StandardWrite for u16 {
    fn standard_write<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        let v: &[u8; 2] = unsafe { mem::transmute(self) };
        sink.write_all(&v[..])?;
        Ok(())
    }
}

impl StandardRead for i16 {
    fn standard_read<R: Read>(&mut self, source: &mut R) -> io::Result<()> {
        let v: &mut [u8; 2] = unsafe { mem::transmute(self) };
        source.read_exact(&mut v[..])?;
        Ok(())
    }
}

impl StandardWrite for i16 {
    fn standard_write<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        let v: &[u8; 2] = unsafe { mem::transmute(self) };
        sink.write_all(&v[..])?;
        Ok(())
    }
}

impl StandardRead for u32 {
    fn standard_read<R: Read>(&mut self, source: &mut R) -> io::Result<()> {
        let v: &mut [u8; 4] = unsafe { mem::transmute(self) };
        source.read_exact(&mut v[..])?;
        Ok(())
    }
}

impl StandardWrite for u32 {
    fn standard_write<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        let v: &[u8; 4] = unsafe { mem::transmute(self) };
        sink.write_all(&v[..])?;
        Ok(())
    }
}

impl StandardRead for i32 {
    fn standard_read<R: Read>(&mut self, source: &mut R) -> io::Result<()> {
        let v: &mut [u8; 4] = unsafe { mem::transmute(self) };
        source.read_exact(&mut v[..])?;
        Ok(())
    }
}

impl StandardWrite for i32 {
    fn standard_write<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        let v: &[u8; 4] = unsafe { mem::transmute(self) };
        sink.write_all(&v[..])?;
        Ok(())
    }
}

impl StandardRead for f32 {
    fn standard_read<R: Read>(&mut self, source: &mut R) -> io::Result<()> {
        let v: &mut [u8; 4] = unsafe { mem::transmute(self) };
        source.read_exact(&mut v[..])?;
        Ok(())
    }
}

impl StandardWrite for f32 {
    fn standard_write<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        let v: &[u8; 4] = unsafe { mem::transmute(self) };
        sink.write_all(&v[..])?;
        Ok(())
    }
}

impl StandardRead for u64 {
    fn standard_read<R: Read>(&mut self, source: &mut R) -> io::Result<()> {
        let v: &mut [u8; 8] = unsafe { mem::transmute(self) };
        source.read_exact(&mut v[..])?;
        Ok(())
    }
}

impl StandardWrite for u64 {
    fn standard_write<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        let v: &[u8; 8] = unsafe { mem::transmute(self) };
        sink.write_all(&v[..])?;
        Ok(())
    }
}

impl StandardRead for i64 {
    fn standard_read<R: Read>(&mut self, source: &mut R) -> io::Result<()> {
        let v: &mut [u8; 8] = unsafe { mem::transmute(self) };
        source.read_exact(&mut v[..])?;
        Ok(())
    }
}

impl StandardWrite for i64 {
    fn standard_write<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        let v: &[u8; 8] = unsafe { mem::transmute(self) };
        sink.write_all(&v[..])?;
        Ok(())
    }
}

impl StandardRead for f64 {
    fn standard_read<R: Read>(&mut self, source: &mut R) -> io::Result<()> {
        let v: &mut [u8; 8] = unsafe { mem::transmute(self) };
        source.read_exact(&mut v[..])?;
        Ok(())
    }
}

impl StandardWrite for f64 {
    fn standard_write<W: Write>(&self, sink: &mut W) -> io::Result<()> {
        let v: &[u8; 8] = unsafe { mem::transmute(self) };
        sink.write_all(&v[..])?;
        Ok(())
    }
}