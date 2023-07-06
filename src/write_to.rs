//taken from https://stackoverflow.com/questions/50200268/how-can-i-use-the-format-macro-in-a-no-std-environment
//adapted to add elipses if the buffer is too small
use core::cmp::min;
use core::fmt;

pub struct WriteTo<'a> {
    buffer: &'a mut [u8],
    // on write error (i.e. not enough space in buffer) this grows beyond
    // `buffer.len()`.
    used: usize,
}

impl<'a> WriteTo<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        WriteTo { buffer, used: 0 }
    }

    pub fn into_str(self) -> Option<&'a str> {
        use core::str::from_utf8_unchecked;
        Some(unsafe { from_utf8_unchecked(&self.buffer[..min(self.used, self.buffer.len())]) })
    }

    fn elipse(&mut self) {
        let offset = self.buffer.len() - 3;
        self.buffer[offset..].copy_from_slice(b"...");
    }
}

impl<'a> fmt::Write for WriteTo<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.used > self.buffer.len() {
            self.elipse();
            return Ok(());
        }
        let remaining_buf = &mut self.buffer[self.used..];
        let raw_s = s.as_bytes();
        let write_num = min(raw_s.len(), remaining_buf.len());
        remaining_buf[..write_num].copy_from_slice(&raw_s[..write_num]);
        self.used += raw_s.len();
        if write_num < raw_s.len() {
            self.elipse();
        }

        Ok(())
    }
}

pub fn show<'a>(buffer: &'a mut [u8], args: fmt::Arguments) -> Result<&'a str, fmt::Error> {
    let mut w = WriteTo::new(buffer);
    fmt::write(&mut w, args)?;
    w.into_str().ok_or(fmt::Error)
}

#[cfg(test)]
mod test {
    use super::show;
    #[test]
    fn test() {
        let mut buf = [0u8; 64];
        let _s: &str = show(
            &mut buf,
            format_args!("write some stuff {:?}: {}", "foo", 42),
        )
        .unwrap();
    }

    #[test]
    fn test_longer_string() {
        let mut buf = [0u8; 10];
        let s: &str = show(&mut buf, format_args!("123456789{}{}", "01", 2)).unwrap();
        assert_eq!(s, "1234567...")
    }

    #[test]
    fn test_exact_size_string() {
        let mut buf = [0u8; 11];
        let s: &str = show(&mut buf, format_args!("123456789{}", "01")).unwrap();
        assert_eq!(s, "12345678901")
    }
}
