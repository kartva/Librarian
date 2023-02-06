use crate::BaseComp;
use std::io::BufRead;

#[cfg(test)]
mod test_check_read {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_check_colorspace() {
        let mut read = FASTQRead::new(6);
        let mut reader = return_reader(b"@\nAT1CGN\n+\n!!!!!!");
        read.read_fastq(&mut reader);

        assert!(read.check_colorspace("AT1CGN"))
    }

    #[test]
    fn test_count_n() {
        assert_eq!(FASTQRead::count_n("NNANNA"), 4)
    }

    #[test]
    fn test_get_average_quality() {
        assert_eq!(FASTQRead::get_average_quality("#{|}"), 68)
    }

    #[test]
    fn test_check_read() {
        let mut reader = return_reader(
            br"@
AAAAANNNNN
+
!!!!!!!!!!",
        );
        let mut f = FASTQRead::new(5);
        f.read_fastq(&mut reader);

        // case where read is trimmed
        let args = SampleArgs {
            target_read_count: 1,
            min_phred_score: 0,
            n_content: None,
            trimmed_length: 5,
        };

        assert!(f.check_read(&args));

        // case where read is too short for trim length
        let args = SampleArgs {
            target_read_count: 1,
            min_phred_score: 0,
            n_content: None,
            trimmed_length: 15,
        };

        assert!(!f.check_read(&args));

        // case where too many N's
        let args = SampleArgs {
            target_read_count: 1,
            min_phred_score: 0,
            n_content: Some(1),
            trimmed_length: 0,
        };

        assert!(!f.check_read(&args));

        // case where quality too low
        let args = SampleArgs {
            target_read_count: 1,
            min_phred_score: 50,
            n_content: Some(1),
            trimmed_length: 0,
        };

        assert!(!f.check_read(&args));
    }
}

#[cfg(test)]
mod test_runs {
    use super::*;
    use crate::{test_utils::*, BaseCompColBases};

    #[test]
    fn test_json_run() {
        let reader = return_reader(b"@\nAAA\n+\n~~~");
        let args = SampleArgs {
            target_read_count: 1u64,
            min_phred_score: 0,
            n_content: None,
            trimmed_length: 2,
        };

        let result = run_json(FASTQReader::new(args, reader));

        assert_eq!(
            result,
            std::str::from_utf8(b"{\"lib\":[{\"pos\":1,\"bases\":{\"A\":100,\"T\":0,\"G\":0,\"C\":0,\"N\":0}},{\"pos\":2,\"bases\":{\"A\":100,\"T\":0,\"G\":0,\"C\":0,\"N\":0}}],\"reads_read\":1}").unwrap()
        );
    }

    #[test]
    fn test_tsv_run() {
        let reader = return_reader(b"@\nAAA\n+\n~~~");
        let args = SampleArgs {
            target_read_count: 1u64,
            min_phred_score: 0,
            n_content: None,
            trimmed_length: 2,
        };

        let (result, seqs) = run_tsv(FASTQReader::new(args, reader));

        assert_eq!(
            result,
            std::str::from_utf8(b"100\t0\t0\t0\t0\t100\t0\t0\t0\t0").unwrap()
        );
        assert_eq!(seqs, 1);
    }

    #[test]
    fn test_run() {
        let reader = return_reader(
            br"@
AACAA
+
*****
@
AAGGA
+
!!!!!
@
AACAA
+
*****
@
TAGGA
+
*****
@
TACAA
+
*****
@
TAGGA
+
*****
@
TACAA
+
*****
@
CCCCC
+
*****
@
NNNNN
+
*****",
        );

        let args = SampleArgs {
            target_read_count: 8,
            min_phred_score: 1,
            n_content: Some(1),
            trimmed_length: 4,
        };

        let res = run(FASTQReader::new(args, reader));
        assert_eq!(res.reads_read(), 7);
        assert_eq!(
            res.lib[0].bases,
            BaseCompColBases {
                A: 28,
                T: 57,
                G: 0,
                C: 14,
                N: 0
            }
        );
    }
}

#[cfg(test)]
mod test_fastqreader {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_skipping() {
        // cases of:
        // read too short
        // too many N
        // too low quality
        // correct read
        let reader = return_reader(
            br"@
ACGT
+
IIII
@
ACNNN
+
IIIII
@
ACGTN
+
!!!!!
@
ACGTN
+
IIIII
",
        );

        let mut freader = FASTQReader::new(
            SampleArgs {
                target_read_count: 2,
                min_phred_score: 1,
                n_content: Some(2),
                trimmed_length: 5,
            },
            reader,
        );

        assert_eq!(freader.next(), Some("ACGTN".to_string()));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SampleArgs {
    /// Target sample count
    pub target_read_count: u64,
    /// Sets minimum average quality allowed in sampled reads.
    pub min_phred_score: usize,
    /// Sets maximum amount of N's allowed in sample reads. Set to none for no truncation.
    pub n_content: Option<usize>,
    /// Trims each sampled read to given length. Set to 0 for no trimming.
    pub trimmed_length: usize,
}

impl Default for SampleArgs {
    fn default() -> Self {
        SampleArgs {
            target_read_count: 100_000,
            min_phred_score: 0,
            n_content: None,
            trimmed_length: 50,
        }
    }
}

use regex::Regex;

use lazy_static::lazy_static;
lazy_static! {
    // Regex for checking if seq has numbers
    static ref SEQCOLORSPACECHECKER: Regex = Regex::new(r"\d").unwrap();
}

/// Abstraction for a single read of FASTQ data
#[derive(Debug)]
pub(crate) struct FASTQRead {
    pub seq: String,
    quals: String,
}

impl FASTQRead {
    /// Reads a complete FASTQ statement (composed of 4 lines) into itself
    /// - `reader`: Object implementing `std::io::BufRead` from which to read lines
    /// - Returns `None` if EOF reached.
    fn read_fastq(&mut self, reader: &mut impl BufRead) -> Option<()> {
        //Skips the 1st and 3rd line resp. in 4 lines of input
        for s in [&mut self.seq, &mut self.quals].iter_mut() {
            **s = match reader.lines().nth(1) {
                Some(n) => n.expect("Cannot read input file. The file might not be in the right format, corrupted or if compressed, may not have the correct extensions (.fastq.gz)."),
                None => return None,
            }
        }

        Some(())
    }

    fn new(len: usize) -> FASTQRead {
        FASTQRead {
            seq: String::with_capacity(len),
            quals: String::with_capacity(len),
        }
    }

    fn count_n(seq: &str) -> usize {
        seq.matches('N').count()
    }

    // Returns true if number is found in seq
    fn check_colorspace(&self, seq: &str) -> bool {
        SEQCOLORSPACECHECKER.is_match(seq)
    }

    fn get_average_quality(quals: &str) -> usize {
        let mut qual_sum: usize = 0;
        for char in quals.as_bytes() {
            qual_sum += (*char as usize) - 33;
        }

        qual_sum / quals.len()
    }

    /// Returns trimmed string.
    /// - In case len = None, returns string unchanged
    /// - In case len > str len, returns Err

    fn trim(str: &str, len: usize) -> Result<&str, ()> {
        match len {
            n if n != 0 => {
                if n > str.len() {
                    return Err(());
                }
                Ok(&str[0..n])
            }
            _ => Ok(&str[0..]),
        }
    }

    /** Checks read according to parameters given in SampleArgs,
    return `true` if read should be included in calculation of Base Compositions,
    return `false` if not.

    Eg.
    Read "N" and SampleArgs.n_content: Some(1) will return false.
    */
    fn check_read(&mut self, args: &SampleArgs) -> bool {
        let seq = FASTQRead::trim(&self.seq, args.trimmed_length);
        let quals = FASTQRead::trim(&self.quals, args.trimmed_length);

        let (seq, quals) = match (seq, quals) {
            (Ok(s), Ok(q)) => (s, q),
            _ => return false,
        };

        // Check for numbers in reads
        if self.check_colorspace(seq) {
            panic!(
                "Found numbers in reads - this doesn't look like a fastq file\n{:?}",
                (seq, quals)
            );
        }

        // Count the N's
        if let Some(n) = args.n_content {
            if FASTQRead::count_n(seq) > n {
                return false;
            }
        }

        if FASTQRead::get_average_quality(quals) < args.min_phred_score {
            return false;
        }

        true
    }
}

use reservoir_sampling::unweighted::l as sample;

/** Takes in reader (for FASTQ lines) and SampleArgs,
returns JSONified string which includes number of reads read along with base composition.

Example output (example has whitespace to make it readable, output will not have that):
```json
{
    lib: [
        {
            pos: 1,
            bases: {
                "A":100
                "T":0
                "G":0,
                "C":0,
                "N":0
            }
        }
    ],
    reads_read: 1
}
```
Note: Reads read counts _number_ of reads read,
while pos represents the _column_ of reads whose percentage is being displayed.
*/
pub fn run_json<T>(fastq_reader: FASTQReader<T>) -> String
where
    T: BufRead,
{
    let comp = run(fastq_reader);

    serde_json::to_string(&comp).expect("Error converting base compositions to JSON")
}

/**
Takes in reader (for FASTQ lines) and SampleArgs,
returns tuple of a string (the actual base compositions) and number of reads read.
Column data is collated together, and the order in a column: `A C G T N`.
So the data looks like:
```not_rust
Output for 1 read with two columns.
(
    "100\t0\t0\t0\t0\t100\t0\t0\t0\t0",
    1
)
```
*/
pub fn run_tsv<T>(fastq_reader: FASTQReader<T>) -> (String, u64)
where
    T: BufRead,
{
    let comp = run(fastq_reader);
    let lines_read = comp.reads_read;

    (
        {
            let mut s = comp
                .lib
                .into_iter()
                .flat_map(|b| b.bases.iter())
                .fold(String::new(), |acc, curr| acc + &curr.to_string() + "\t");
            s.pop(); // remove trailing ',' to make it valid tsv
            s
        },
        lines_read,
    )
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    processed_num: u64,
    out: BaseComp,
}

/// Takes in reader (for FASTQ lines) and SampleArgs, returns [`BaseComp`]
pub fn run<T>(fastq_reader: FASTQReader<T>) -> BaseComp
where
    T: BufRead,
{
    //TODO: Convert args.target_read_count to usize or figure out how to allocate u64-sized vec
    let sampled_seqs = fastq_reader.sample_random();

    // Figure out allotment size based on line size, or provided trim len
    let mut base_comp = BaseComp::init(sampled_seqs[0].len());

    for seq in sampled_seqs {
        if seq.is_empty() {
            break;
        }
        base_comp.extract(&seq);
    }

    for r in base_comp.lib.iter_mut() {
        r.bases.percentage();
    }

    base_comp
}

pub struct FASTQReader<T: BufRead> {
    curr: FASTQRead,
    reader: T,
    sample_args: SampleArgs,
    pub target_read_count: u64,
}

impl<T: BufRead> FASTQReader<T> {
    pub fn new(args: SampleArgs, reader: T) -> FASTQReader<T> {
        let read = FASTQRead::new(args.trimmed_length);
        let target_read_count = args.target_read_count;

        FASTQReader {
            curr: read,
            reader,
            sample_args: args,
            target_read_count,
        }
    }
    pub fn sample_random(self) -> Vec<String> {
        let mut sampled_seqs = vec![String::new(); self.target_read_count as usize];

        // Randomly sample FASTQ reads
        sample(self, sampled_seqs.as_mut_slice());
        sampled_seqs
    }
}

impl<T: BufRead> Iterator for FASTQReader<T> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            self.curr.read_fastq(&mut self.reader)?;
            if FASTQRead::check_read(&mut self.curr, &self.sample_args) {
                break;
            }
        }

        Some(
            FASTQRead::trim(&self.curr.seq, self.sample_args.trimmed_length)
                .unwrap()
                .to_string(),
        )
    }
}
