pub mod extract_comp;

pub mod test_utils {
    use std::io::Cursor;
    /// Returns reader which implements Read trait.
    /// - `s`: Data which should be yielded by the reader upon read
    pub fn return_reader(s: &[u8]) -> Cursor<&[u8]> {
        Cursor::new(s)
    }

    /// Returns a writer which implements Write trait.
    /// `writer.get_ref()[0..].to_vec()` can be used to get the data written to the writer.
    pub fn return_writer() -> Cursor<Vec<u8>> {
        Cursor::new(Vec::<u8>::new())
    }

    pub fn get_writer_content(writer: Cursor<Vec<u8>>) -> String {
        std::str::from_utf8(&writer.get_ref()[0..])
            .unwrap()
            .to_string()
    }
}

pub mod io_utils {
    use flate2::read::GzDecoder;
    use std::fs::OpenOptions;
    use std::io::{self, BufRead, BufReader, Read, Write};
    use std::path::PathBuf;

    // Reader is a wrapper over BufRead
    // Takes in a PathBuf and open it or if no PathBuf is provided, opens up stdin
    // And provides an interface over the actual reading.
    pub fn compressed_reader<T: Read + 'static>(reader: T, compressed: bool) -> Box<dyn BufRead> {
        Box::new(BufReader::new(if compressed {
            Box::new(GzDecoder::new(reader))
        } else {
            Box::new(reader) as Box<dyn Read>
        }))
    }

    use std::io::ErrorKind;
    /// Will return writer to File if PathBuf can be opened, will panic if File unavailable
    /// And return writer to stdout if PathBuf not given
    pub fn get_writer(output: &Option<PathBuf>) -> Box<dyn Write> {
        match output {
            Some(file) => Box::new(OpenOptions::new().append(true).open(file).unwrap_or_else(
                |error| {
                    if error.kind() == ErrorKind::NotFound {
                        OpenOptions::new()
                            .create(true)
                            .write(true)
                            .open(file)
                            .expect("Problem creating the file!")
                    } else {
                        panic!("Problem opening the file: {:?}", error);
                    }
                },
            )),
            None => Box::new(io::stdout()),
        }
    }
}

use serde::{Deserialize, Serialize};

type CompCol = u64;

/// Represents a column of base compositions.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct BaseCompCol {
    pub A: CompCol,
    pub T: CompCol,
    pub G: CompCol,
    pub C: CompCol,
    pub N: CompCol,
}

type PercentageCompCol = u64;

/// Represents a percentage of base composition.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct BaseCompColBasesPercentage {
    pub A: PercentageCompCol,
    pub T: PercentageCompCol,
    pub G: PercentageCompCol,
    pub C: PercentageCompCol,
    pub N: PercentageCompCol,
}

impl BaseCompColBasesPercentage {
    pub fn as_array(self) -> [PercentageCompCol; 5] {
        [self.A, self.C, self.G, self.T, self.N]
    }
}

impl BaseCompCol {
    pub fn new() -> BaseCompCol {
        BaseCompCol {
            A: 0,
            G: 0,
            T: 0,
            C: 0,
            N: 0,
        }
    }

    pub fn as_array(&self) -> [CompCol; 5] {
        [self.A, self.C, self.G, self.T, self.N]
    }

    pub fn percentage(self) -> BaseCompColBasesPercentage {
        let sum = self.as_array().iter().sum::<u64>();
        let apply = |base: u64| (base * 100) / sum;
        
        BaseCompColBasesPercentage {
            A: apply(self.A),
            C: apply(self.C),
            G: apply(self.G),
            T: apply(self.T),
            N: apply(self.N),
        }
    }

    pub fn extract(&mut self, s: &u8) {
        match s {
            b'A' => self.A += 1,
            b'T' => self.T += 1,
            b'G' => self.G += 1,
            b'C' => self.C += 1,
            b'N' => self.N += 1,
            _ => panic!(
                "Invalid character {:?} == {:?} found in read",
                *s as char,
                s.to_ascii_lowercase()
            ),
        }
    }
}

impl Default for BaseCompCol {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the entire base composition.
/// As a Vec of `BaseCompCol`(umns), each of which hold data for a single column.
/// Also holds data on how many reads were read to produce the compositions.
#[derive(Debug)]
pub struct RawBaseComp {
    pub lib: Vec<BaseCompCol>,
    reads_read: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseComp {
    pub lib: Vec<BaseCompColBasesPercentage>,
    reads_read: u64,
}

impl RawBaseComp {
    /// Expects `seqs` to have at least one element.
    pub fn create<S: AsRef<str>, I: IntoIterator<Item = S>>(seqs: I) -> RawBaseComp {
        let mut seqs = seqs.into_iter();
        let first = seqs.next().unwrap();
        let first = first.as_ref();

        let mut base_comp = RawBaseComp {
            lib: Vec::with_capacity(first.len()),
            reads_read: 0,
        };
        for _ in 0..first.len() {
            base_comp.lib.push(BaseCompCol::new());
        }

        base_comp.extract(first);
        for seq in seqs {
            let seq = seq.as_ref();
            if seq.is_empty() {
                break;
            }
            base_comp.extract(seq);
        }

        base_comp
    }

    pub fn reads_read(&self) -> u64 {
        self.reads_read
    }

    pub fn len(&self) -> usize {
        self.lib.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn extract(&mut self, s: &str) {
        for c in s.as_bytes().iter().enumerate() {
            self.lib[c.0].extract(c.1);
        }
        self.reads_read += 1;
    }

    pub fn percentage(self) -> BaseComp {
        let mut base_comp = BaseComp {
            lib: Vec::with_capacity(self.len()),
            reads_read: self.reads_read,
        };
        for col in self.lib.iter() {
            base_comp.lib.push(col.percentage());
        }

        base_comp
    }
}

impl BaseComp {
    pub fn reads_read(&self) -> u64 {
        self.reads_read
    }

    pub fn len(&self) -> usize {
        self.lib.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod col_base_comp_tests {
    use super::*;

    #[test]
    fn test_extract() {
        let mut read = BaseCompCol::new();
        read.extract(&b'A');
        assert_eq!(read.A, 1);

        read.extract(&b'C');
        assert_eq!(read.C, 1);

        read.extract(&b'T');
        assert_eq!(read.T, 1);

        read.extract(&b'G');
        assert_eq!(read.G, 1);

        read.extract(&b'N');
        assert_eq!(read.N, 1);
    }
    #[test]
    fn test_percentage() {
        let mut read = BaseCompCol::new();
        for c in "ACTGN".as_bytes().iter() {
            read.extract(c);
        }
        let read = read.percentage();

        println!("{:?}", read);

        assert_eq!(read.A, 20, "Testing A");
        assert_eq!(read.C, 20, "Testing C");
        assert_eq!(read.T, 20, "Testing T");
        assert_eq!(read.G, 20, "Testing G");
        assert_eq!(read.N, 20, "Testing N");
    }
}
