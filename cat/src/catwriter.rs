use super::*;

use counter::Counter;
pub(super) struct CatWriter {
    options: HashSet<Options>,
    counter: Option<Counter>,
    buffer: Box<BufWriter<dyn Write>>,
    was_empty: bool,
}

impl CatWriter {
    pub fn new(options: HashSet<Options>, buffer: Box<BufWriter<dyn Write>>) -> Self {
        let mut counter = None;
        if options.contains(&Options::Number) {
            counter = Some(Counter::new());
        }
        Self {
            options,
            counter,
            buffer,
            was_empty: false
        }
    }
}

impl Write for CatWriter {
   
    // TODO: Refactor
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf != b"\n"{
            self.was_empty = false;
        }

        if self.options.contains(&Options::SqueezeBlank) && self.was_empty{
            return Ok(0)
        }

        let mut written: usize = 0;

        
        if let Some(ref mut counter) = self.counter {
            let should_count = !(self.options.contains(&Options::NonBlank) && buf == b"\n");

            if should_count {
                let n = counter.next().unwrap();
                let prefix = format!("{:>1$}\t", n, LEFT_ALIGN).into_bytes();
                written += self.buffer.write(&prefix)?;
            }
        };

        if buf == b"\n"{
            self.was_empty = true;
        }

        let mut new_buf: Vec<u8> = Vec::with_capacity(2 * buf.len());

        for byte in buf {
            match byte {
                b'\t' => {
                    if self.options.contains(&Options::ShowTabs){
                        new_buf.push(b'^');
                        new_buf.push(b'I');
                    } else {
                        new_buf.push(*byte);
                    }
                }
                b'\n' => {
                    if self.options.contains(&Options::ShowEnds){
                        new_buf.push(b'$');
                        new_buf.push(b'\n');
                    } else {
                        new_buf.push(*byte);
                    }
                }
                0..=31 => {
                    if self.options.contains(&Options::ShowNonPrinting){
                        new_buf.push(b'^');
                        new_buf.push(*byte + 64);
                    } else {
                        new_buf.push(*byte);
                    }
                }
                32..= 126 => new_buf.push(*byte),
                127 => {
                    if self.options.contains(&Options::ShowNonPrinting){
                        new_buf.push(b'^');
                        new_buf.push(b'?');
                    } else {
                        new_buf.push(*byte);
                    }
                }
                128..=159 => {
                    if self.options.contains(&Options::ShowNonPrinting){
                        new_buf.push(b'M');
                        new_buf.push(b'-');
                        new_buf.push(b'^');
                        new_buf.push(*byte - 64);
                    } else {
                        new_buf.push(*byte);
                    }
                }
                160..=254 => {
                    if self.options.contains(&Options::ShowNonPrinting){
                        new_buf.push(b'M');
                        new_buf.push(b'-');
                        new_buf.push(*byte - 128);
                    } else {
                        new_buf.push(*byte);
                    }
                }
                255 => {
                    if self.options.contains(&Options::ShowNonPrinting){
                        new_buf.push(b'M');
                        new_buf.push(b'-');
                        new_buf.push(b'^');
                        new_buf.push(b'?');
                    } else {
                        new_buf.push(*byte);
                    }
                }
            }
        }

        written += self.buffer.write(&new_buf)?;
        
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}

mod counter {
    pub(super) struct Counter {
        value: u32,
    }

    impl Counter {
        pub fn new() -> Self {
            Self { value: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            self.value += 1;
            Some(self.value)
        }
    }
}
