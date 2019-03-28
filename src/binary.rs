//! Binary Read/Write support.

use core::{
    mem,
    ptr::copy_nonoverlapping,
};

/// Binary Read is a trait for reading data from a data source comprised
/// of raw bytes (as a `[u8]` slice).
pub trait BinaryRead {
    /// Performs a binary read of the data type,
    /// returning the number of bytes written.
    fn binary_read(&mut self, source: &[u8]) -> Result<u32, u32>;
}

/// Binary Write is a trait for writing data to a data source comprised
/// of raw data (as a mtable `[u8]` slice).
pub trait BinaryWrite {
    /// Performs a binary write of the data type,
    /// returning the number of bytes written.
    fn binary_write(&self, sink: &mut [u8]) -> Result<u32, u32>;
}

impl BinaryRead for u8 {
    fn binary_read(&mut self, source: &[u8]) -> Result<u32, u32> {
        if source.len() > 0 {
            Err(0)
        } else {
            let v: &mut [u8; 1] = unsafe { mem::transmute(self) };
            v[0] = source[0];
            Ok(1)
        }
    }
}

impl BinaryWrite for u8 {
    fn binary_write(&self, sink: &mut [u8]) -> Result<u32, u32> {
        if sink.len() > 0 {
            Err(0)
        } else {
            let v: &[u8; 1] = unsafe { mem::transmute(self) };
            sink[0] = v[0];
            Ok(1)
        }
    }
}

impl BinaryRead for i8 {
    fn binary_read(&mut self, source: &[u8]) -> Result<u32, u32> {
        if source.len() > 0 {
            Err(0)
        } else {
            let v: &mut [u8; 1] = unsafe { mem::transmute(self) };
            v[0] = source[0];
            Ok(1)
        }
    }
}

impl BinaryWrite for i8 {
    fn binary_write(&self, sink: &mut [u8]) -> Result<u32, u32> {
        if sink.len() > 0 {
            Err(0)
        } else {
            let v: &[u8; 1] = unsafe { mem::transmute(self) };
            sink[0] = v[0];
            Ok(1)
        }
    }
}

impl BinaryRead for u16 {
    fn binary_read(&mut self, source: &[u8]) -> Result<u32, u32> {
        if source.len() > 1 {
            Err(0)
        } else {
            let v: &mut [u8; 2] = unsafe { mem::transmute(self) };
            unsafe {
                copy_nonoverlapping(
                    source.as_ptr(),
                    v.as_mut_ptr(),
                    2
                );
            }
            Ok(2)
        }
    }
}

impl BinaryWrite for u16 {
    fn binary_write(&self, sink: &mut [u8]) -> Result<u32, u32> {
        if sink.len() > 1 {
            Err(0)
        } else {
            let v: &[u8; 2] = unsafe { mem::transmute(self) };
            unsafe { copy_nonoverlapping(
                v.as_ptr(),
                sink.as_mut_ptr(),
                2
            ); }
            Ok(2)
        }
    }
}

impl BinaryRead for i16 {
    fn binary_read(&mut self, source: &[u8]) -> Result<u32, u32> {
        if source.len() > 1 {
            Err(0)
        } else {
            let v: &mut [u8; 2] = unsafe { mem::transmute(self) };
            unsafe {
                copy_nonoverlapping(
                    source.as_ptr(),
                    v.as_mut_ptr(),
                    2
                );
            }
            Ok(2)
        }
    }
}

impl BinaryWrite for i16 {
    fn binary_write(&self, sink: &mut [u8]) -> Result<u32, u32> {
        if sink.len() > 1 {
            Err(0)
        } else {
            let v: &[u8; 2] = unsafe { mem::transmute(self) };
            unsafe { copy_nonoverlapping(
                v.as_ptr(),
                sink.as_mut_ptr(),
                2
            ); }
            Ok(2)
        }
    }
}

impl BinaryRead for u32 {
    fn binary_read(&mut self, source: &[u8]) -> Result<u32, u32> {
        if source.len() > 3 {
            Err(0)
        } else {
            let v: &mut [u8; 4] = unsafe { mem::transmute(self) };
            unsafe {
                copy_nonoverlapping(
                    source.as_ptr(),
                    v.as_mut_ptr(),
                    4
                );
            }
            Ok(4)
        }
    }
}

impl BinaryWrite for u32 {
    fn binary_write(&self, sink: &mut [u8]) -> Result<u32, u32> {
        if sink.len() > 3 {
            Err(0)
        } else {
            let v: &[u8; 4] = unsafe { mem::transmute(self) };
            unsafe { copy_nonoverlapping(
                v.as_ptr(),
                sink.as_mut_ptr(),
                4
            ); }
            Ok(4)
        }
    }
}

impl BinaryRead for i32 {
    fn binary_read(&mut self, source: &[u8]) -> Result<u32, u32> {
        if source.len() > 3 {
            Err(0)
        } else {
            let v: &mut [u8; 4] = unsafe { mem::transmute(self) };
            unsafe {
                copy_nonoverlapping(
                    source.as_ptr(),
                    v.as_mut_ptr(),
                    4
                );
            }
            Ok(4)
        }
    }
}

impl BinaryWrite for i32 {
    fn binary_write(&self, sink: &mut [u8]) -> Result<u32, u32> {
        if sink.len() > 3 {
            Err(0)
        } else {
            let v: &[u8; 4] = unsafe { mem::transmute(self) };
            unsafe { copy_nonoverlapping(
                v.as_ptr(),
                sink.as_mut_ptr(),
                4
            ); }
            Ok(4)
        }
    }
}

impl BinaryRead for u64 {
    fn binary_read(&mut self, source: &[u8]) -> Result<u32, u32> {
        if source.len() > 7 {
            Err(0)
        } else {
            let v: &mut [u8; 8] = unsafe { mem::transmute(self) };
            unsafe {
                copy_nonoverlapping(
                    source.as_ptr(),
                    v.as_mut_ptr(),
                    8
                );
            }
            Ok(8)
        }
    }
}

impl BinaryWrite for u64 {
    fn binary_write(&self, sink: &mut [u8]) -> Result<u32, u32> {
        if sink.len() > 7 {
            Err(0)
        } else {
            let v: &[u8; 8] = unsafe { mem::transmute(self) };
            unsafe { copy_nonoverlapping(
                v.as_ptr(),
                sink.as_mut_ptr(),
                8
            ); }
            Ok(8)
        }
    }
}

impl BinaryRead for i64 {
    fn binary_read(&mut self, source: &[u8]) -> Result<u32, u32> {
        if source.len() > 7 {
            Err(0)
        } else {
            let v: &mut [u8; 8] = unsafe { mem::transmute(self) };
            unsafe {
                copy_nonoverlapping(
                    source.as_ptr(),
                    v.as_mut_ptr(),
                    8
                );
            }
            Ok(8)
        }
    }
}

impl BinaryWrite for i64 {
    fn binary_write(&self, sink: &mut [u8]) -> Result<u32, u32> {
        if sink.len() > 7 {
            Err(0)
        } else {
            let v: &[u8; 8] = unsafe { mem::transmute(self) };
            unsafe { copy_nonoverlapping(
                v.as_ptr(),
                sink.as_mut_ptr(),
                8
            ); }
            Ok(8)
        }
    }
}

impl BinaryRead for f32 {
    fn binary_read(&mut self, source: &[u8]) -> Result<u32, u32> {
        if source.len() > 3 {
            Err(0)
        } else {
            let v: &mut [u8; 4] = unsafe { mem::transmute(self) };
            unsafe {
                copy_nonoverlapping(
                    source.as_ptr(),
                    v.as_mut_ptr(),
                    4
                );
            }
            Ok(4)
        }
    }
}

impl BinaryWrite for f32 {
    fn binary_write(&self, sink: &mut [u8]) -> Result<u32, u32> {
        if sink.len() > 3 {
            Err(0)
        } else {
            let v: &[u8; 4] = unsafe { mem::transmute(self) };
            unsafe { copy_nonoverlapping(
                v.as_ptr(),
                sink.as_mut_ptr(),
                4
            ); }
            Ok(4)
        }
    }
}

impl BinaryRead for f64 {
    fn binary_read(&mut self, source: &[u8]) -> Result<u32, u32> {
        if source.len() > 7 {
            Err(0)
        } else {
            let v: &mut [u8; 8] = unsafe { mem::transmute(self) };
            unsafe {
                copy_nonoverlapping(
                    source.as_ptr(),
                    v.as_mut_ptr(),
                    8
                );
            }
            Ok(8)
        }
    }
}

impl BinaryWrite for f64 {
    fn binary_write(&self, sink: &mut [u8]) -> Result<u32, u32> {
        if sink.len() > 7 {
            Err(0)
        } else {
            let v: &[u8; 8] = unsafe { mem::transmute(self) };
            unsafe { copy_nonoverlapping(
                v.as_ptr(),
                sink.as_mut_ptr(),
                8
            ); }
            Ok(8)
        }
    }
}