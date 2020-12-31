use std::cell::Cell;
use std::io;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::str;

use byteorder::{LittleEndian, ReadBytesExt};
use encoding_rs::{EUC_KR, UTF_16LE};
use failure::Error;

use crate::utils::{Color3, Color4, Quaternion, Vector2, Vector3, Vector4};

// Temporary work-around until specialization is supported in Rust
thread_local! { static WIDE_STRINGS: Cell<bool> = Cell::new(false); }

/// Custom reader that supports some additional configurable options such
/// as reading strings as wide-strings.
//
// TODO: Add tests (sample file: ai_s.stb)
pub struct RoseReader<R> {
    pub reader: BufReader<R>,
}

impl<R: Read> RoseReader<R> {
    pub fn new(inner: R) -> RoseReader<R> {
        RoseReader {
            reader: BufReader::new(inner),
        }
    }

    pub fn set_wide_strings(&self, b: bool) {
        WIDE_STRINGS.with(|v| {
            v.set(b);
        });
    }
}

impl<R: Read> Read for RoseReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Seek> Seek for RoseReader<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.reader.seek(pos)
    }
}

impl<R: Read> BufRead for RoseReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }
}

/// Extends `BufReader` with methods for reading ROSE data types
///
///# Example
/// ```rust,no_run
/// use std::fs::File;
/// use std::io::BufReader;
/// use roselib::io::ReadRoseExt;
///
/// let f = File::open("my_file.ext").unwrap();
/// let mut reader = BufReader::new(f);
/// let x = reader.read_i8().unwrap();
/// let y = reader.read_f64().unwrap();
///
/// let s = reader.read_cstring().unwrap();
///
/// println!("x is {}, y is {}, s is {}", x, y, s);
/// ```
///
/// NOTE: Strings are decoded as UTF-8. If the string is not valid UTF-8 then EUC-KR
/// is used as the fallback using replacement characters where necessary.
pub trait ReadRoseExt: Read + Seek + BufRead {
    fn read_u8(&mut self) -> Result<u8, Error>;
    fn read_u16(&mut self) -> Result<u16, Error>;
    fn read_u32(&mut self) -> Result<u32, Error>;

    fn read_i8(&mut self) -> Result<i8, Error>;
    fn read_i16(&mut self) -> Result<i16, Error>;
    fn read_i32(&mut self) -> Result<i32, Error>;

    fn read_bool(&mut self) -> Result<bool, Error>;
    fn read_bool16(&mut self) -> Result<bool, Error>;

    fn read_f32(&mut self) -> Result<f32, Error>;
    fn read_f64(&mut self) -> Result<f64, Error>;

    /// Read a null-terminated (c-style string) from the reader
    fn read_cstring(&mut self) -> Result<String, Error>;

    /// Read a string of n-bytes length from the reader
    fn read_string(&mut self, n: u64) -> Result<String, Error>;

    /// Read a string with a u8 prefixed length from the reader
    fn read_string_u8(&mut self) -> Result<String, Error>;

    /// Read a string with a u16 prefixed length from the reader
    fn read_string_u16(&mut self) -> Result<String, Error>;

    /// Read a string with a u32 prefixed length from the reader
    fn read_string_u32(&mut self) -> Result<String, Error>;

    /// Read a string with a variable-byte prefixed length from the reader
    ///
    /// If the string is less than 128 characters the the first byte holds the
    /// length. If the string is greater than or equal to 128 characters then
    /// first two bytes hold the length
    fn read_string_varbyte(&mut self) -> Result<String, Error>;

    fn read_color3(&mut self) -> Result<Color3, Error>;
    fn read_color4(&mut self) -> Result<Color4, Error>;

    fn read_vector2_f32(&mut self) -> Result<Vector2<f32>, Error>;
    fn read_vector2_u32(&mut self) -> Result<Vector2<u32>, Error>;
    fn read_vector2_i32(&mut self) -> Result<Vector2<i32>, Error>;

    fn read_vector3_f32(&mut self) -> Result<Vector3<f32>, Error>;
    fn read_vector3_i16(&mut self) -> Result<Vector3<i16>, Error>;
    fn read_vector3_u32(&mut self) -> Result<Vector3<u32>, Error>;

    fn read_vector4_f32(&mut self) -> Result<Vector4<f32>, Error>;
    fn read_vector4_i16(&mut self) -> Result<Vector4<i16>, Error>;
    fn read_vector4_u32(&mut self) -> Result<Vector4<u32>, Error>;

    fn read_quaternion(&mut self) -> Result<Quaternion, Error>;
    fn read_quaternion_wxyz(&mut self) -> Result<Quaternion, Error>;

    /// Get the position of the stream
    fn position(&mut self) -> Result<u64, Error>;

    // Read strings as wide strings (2-bytes)
    fn wide_strings(&self) -> bool;
}

impl<R> ReadRoseExt for R
where
    R: Read + Seek + BufRead + ReadBytesExt,
{
    fn read_u8(&mut self) -> Result<u8, Error> {
        Ok(ReadBytesExt::read_u8(self)?)
    }

    fn read_u16(&mut self) -> Result<u16, Error> {
        Ok(ReadBytesExt::read_u16::<LittleEndian>(self)?)
    }

    fn read_u32(&mut self) -> Result<u32, Error> {
        Ok(ReadBytesExt::read_u32::<LittleEndian>(self)?)
    }

    fn read_i8(&mut self) -> Result<i8, Error> {
        Ok(ReadBytesExt::read_i8(self)?)
    }

    fn read_i16(&mut self) -> Result<i16, Error> {
        Ok(ReadBytesExt::read_i16::<LittleEndian>(self)?)
    }

    fn read_i32(&mut self) -> Result<i32, Error> {
        Ok(ReadBytesExt::read_i32::<LittleEndian>(self)?)
    }

    fn read_bool(&mut self) -> Result<bool, Error> {
        Ok(ReadRoseExt::read_u8(self)? != 0)
    }

    fn read_bool16(&mut self) -> Result<bool, Error> {
        Ok(ReadRoseExt::read_u16(self)? != 0)
    }

    fn read_f32(&mut self) -> Result<f32, Error> {
        Ok(ReadBytesExt::read_f32::<LittleEndian>(self)?)
    }

    fn read_f64(&mut self) -> Result<f64, Error> {
        Ok(ReadBytesExt::read_f64::<LittleEndian>(self)?)
    }

    fn read_cstring(&mut self) -> Result<String, Error> {
        let mut buffer: Vec<u8> = Vec::new();
        self.read_until(0x00, &mut buffer)?;
        let _ = buffer.pop();
        Ok(decode_string(buffer, self.wide_strings()))
    }

    fn read_string(&mut self, n: u64) -> Result<String, Error> {
        if n == 0 {
            return Ok(String::new());
        }
        let mut buffer = Vec::new();
        let mut bytes = self.take(n as u64);
        bytes.read_to_end(&mut buffer)?;

        // Remove terminating null bytes
        if let Some(&0x00) = buffer.last() {
            let _ = buffer.pop();
        }

        Ok(decode_string(buffer, self.wide_strings()))
    }

    fn read_string_u8(&mut self) -> Result<String, Error> {
        let length = ReadRoseExt::read_u8(self)?;
        self.read_string(u64::from(length))
    }

    fn read_string_u16(&mut self) -> Result<String, Error> {
        let length = ReadRoseExt::read_u16(self)?;
        self.read_string(u64::from(length))
    }

    fn read_string_u32(&mut self) -> Result<String, Error> {
        let length = ReadRoseExt::read_u32(self)?;
        self.read_string(u64::from(length))
    }

    fn read_string_varbyte(&mut self) -> Result<String, Error> {
        let first_byte = ReadRoseExt::read_u8(self)?;
        if (first_byte & 128) == 0 {
            return self.read_string(first_byte as u64);
        }

        let second_byte = ReadRoseExt::read_u8(self)?;
        let length: u16 = ((second_byte as u16) << 7) | ((first_byte as u16) - 128);
        self.read_string(length as u64)
    }

    fn read_color3(&mut self) -> Result<Color3, Error> {
        let mut c = Color3::new();
        c.r = ReadRoseExt::read_f32(self)?;
        c.g = ReadRoseExt::read_f32(self)?;
        c.b = ReadRoseExt::read_f32(self)?;
        Ok(c)
    }

    fn read_color4(&mut self) -> Result<Color4, Error> {
        let mut c = Color4::new();
        c.r = ReadRoseExt::read_f32(self)?;
        c.g = ReadRoseExt::read_f32(self)?;
        c.b = ReadRoseExt::read_f32(self)?;
        c.a = ReadRoseExt::read_f32(self)?;
        Ok(c)
    }

    fn read_vector2_f32(&mut self) -> Result<Vector2<f32>, Error> {
        let mut v = Vector2::<f32>::new();
        v.x = ReadRoseExt::read_f32(self)?;
        v.y = ReadRoseExt::read_f32(self)?;
        Ok(v)
    }

    fn read_vector2_u32(&mut self) -> Result<Vector2<u32>, Error> {
        let mut v = Vector2::<u32>::new();
        v.x = ReadRoseExt::read_u32(self)?;
        v.y = ReadRoseExt::read_u32(self)?;
        Ok(v)
    }

    fn read_vector2_i32(&mut self) -> Result<Vector2<i32>, Error> {
        let mut v = Vector2::<i32>::new();
        v.x = ReadRoseExt::read_i32(self)?;
        v.y = ReadRoseExt::read_i32(self)?;
        Ok(v)
    }

    fn read_vector3_f32(&mut self) -> Result<Vector3<f32>, Error> {
        let mut v = Vector3::<f32>::new();
        v.x = ReadRoseExt::read_f32(self)?;
        v.y = ReadRoseExt::read_f32(self)?;
        v.z = ReadRoseExt::read_f32(self)?;
        Ok(v)
    }

    fn read_vector3_i16(&mut self) -> Result<Vector3<i16>, Error> {
        let mut v = Vector3::<i16>::new();
        v.x = ReadRoseExt::read_i16(self)?;
        v.y = ReadRoseExt::read_i16(self)?;
        v.z = ReadRoseExt::read_i16(self)?;
        Ok(v)
    }

    fn read_vector3_u32(&mut self) -> Result<Vector3<u32>, Error> {
        let mut v = Vector3::<u32>::default();
        v.x = ReadRoseExt::read_u32(self)?;
        v.y = ReadRoseExt::read_u32(self)?;
        v.z = ReadRoseExt::read_u32(self)?;
        Ok(v)
    }

    fn read_vector4_f32(&mut self) -> Result<Vector4<f32>, Error> {
        let mut v = Vector4::<f32>::new();
        v.w = ReadRoseExt::read_f32(self)?;
        v.x = ReadRoseExt::read_f32(self)?;
        v.y = ReadRoseExt::read_f32(self)?;
        v.z = ReadRoseExt::read_f32(self)?;
        Ok(v)
    }

    fn read_vector4_i16(&mut self) -> Result<Vector4<i16>, Error> {
        let mut v = Vector4::<i16>::new();
        v.w = ReadRoseExt::read_i16(self)?;
        v.x = ReadRoseExt::read_i16(self)?;
        v.y = ReadRoseExt::read_i16(self)?;
        v.z = ReadRoseExt::read_i16(self)?;
        Ok(v)
    }

    fn read_vector4_u32(&mut self) -> Result<Vector4<u32>, Error> {
        let mut v = Vector4::<u32>::default();
        v.w = ReadRoseExt::read_u32(self)?;
        v.x = ReadRoseExt::read_u32(self)?;
        v.y = ReadRoseExt::read_u32(self)?;
        v.z = ReadRoseExt::read_u32(self)?;
        Ok(v)
    }

    fn read_quaternion(&mut self) -> Result<Quaternion, Error> {
        let mut q = Quaternion::new();
        q.x = ReadRoseExt::read_f32(self)?;
        q.y = ReadRoseExt::read_f32(self)?;
        q.z = ReadRoseExt::read_f32(self)?;
        q.w = ReadRoseExt::read_f32(self)?;
        Ok(q)
    }

    fn read_quaternion_wxyz(&mut self) -> Result<Quaternion, Error> {
        let mut q = Quaternion::new();
        q.w = ReadRoseExt::read_f32(self)?;
        q.x = ReadRoseExt::read_f32(self)?;
        q.y = ReadRoseExt::read_f32(self)?;
        q.z = ReadRoseExt::read_f32(self)?;
        Ok(q)
    }

    fn position(&mut self) -> Result<u64, Error> {
        Ok(self.seek(SeekFrom::Current(0))?)
    }

    fn wide_strings(&self) -> bool {
        WIDE_STRINGS.with(|b| b.get())
    }
}

/// Decodes a string by first trying to read as UTF-8, otherwise falls back
/// to EUC-KR encoding using replacement characters where necessary. If the
/// wide argument is set then it will only try to decode the string as UTF-16LE
fn decode_string(b: Vec<u8>, wide: bool) -> String {
    if wide {
        let (decoded, _encoding, _valid) = UTF_16LE.decode(&b);
        return String::from(decoded.trim_end_matches('\u{fffd}'));
    }

    match str::from_utf8(&b) {
        Ok(s) => String::from(s),
        Err(_) => {
            let (decoded, _encoding, _valid) = EUC_KR.decode(&b);
            String::from(decoded)
        }
    }
}
