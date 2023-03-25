use crate::write::IndexFileWriter;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader, SeekFrom};
use std::path::Path;

pub struct IndexFileReader {
    main: BufReader<File>,
    contents: BufReader<File>,
    next: Option<Entry>,
}

pub struct Entry {
    pub term: String,
    pub df: u32,
    pub offset: u64,
    pub nbytes: u64,
}

impl IndexFileReader {
    pub fn open_and_delete<P: AsRef<Path>>(filename: P) -> io::Result<IndexFileReader> {
        let filename = filename.as_ref();
        let mut main_raw = File::open(filename)?;

        let contents_offset = main_raw.read_u64::<LittleEndian>()?;
        println!(
            "opened {}, table of contents starts at {}",
            filename.display(),
            contents_offset
        );

        let mut contents_raw = File::open(filename)?;
        contents_raw.seek(SeekFrom::Start(contents_offset))?;
        let main = BufReader::new(main_raw);
        let mut contents = BufReader::new(contents_raw);
        let first = IndexFileReader::read_entry(&mut contents)?;

        fs::remove_file(filename)?; // YOLO

        Ok(IndexFileReader {
            main: main,
            contents: contents,
            next: first,
        })
    }

    fn read_entry(f: &mut BufReader<File>) -> io::Result<Option<Entry>> {
        let offset = match f.read_u64::<LittleEndian>() {
            Ok(value) => value,
            Err(err) => {
                if err.kind() == io::ErrorKind::UnexpectedEof {
                    return Ok(None);
                } else {
                    return Err(err);
                }
            }
        };

        let nbytes = f.read_u64::<LittleEndian>()?;
        let df = f.read_u32::<LittleEndian>()?;
        let term_len = f.read_u32::<LittleEndian>()? as usize;
        let mut bytes = Vec::with_capacity(term_len);
        bytes.resize(term_len, 0);
        f.read_exact(&mut bytes)?;
        let term = match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "unicode fail")),
        };

        Ok(Some(Entry {
            term: term,
            df: df,
            offset: offset,
            nbytes: nbytes,
        }))
    }

    pub fn peek(&self) -> Option<&Entry> {
        self.next.as_ref()
    }

    pub fn is_at(&self, term: &str) -> bool {
        match self.next {
            Some(ref e) => e.term == term,
            None => false,
        }
    }

    pub fn move_entry_to(&mut self, out: &mut IndexFileWriter) -> io::Result<()> {
        {
            let e = self.next.as_ref().expect("no entry to move");
            if e.nbytes > usize::max_value() as u64 {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "computer not big enough to hold index entry",
                ));
            }
            let mut buf = Vec::with_capacity(e.nbytes as usize);
            buf.resize(e.nbytes as usize, 0);
            self.main.read_exact(&mut buf)?;
            out.write_main(&buf)?;
        }

        self.next = Self::read_entry(&mut self.contents)?;
        Ok(())
    }
}
