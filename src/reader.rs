use crate::common::hash_key;
use crate::format::{HopHeader, HopIndexEntry};
use anyhow::Result;
use memmap2::Mmap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

pub struct HopReader {
    mmap: Mmap,
    index: HashMap<u64, HopIndexEntry>,
}

impl HopReader {
    pub fn open(path: &str) -> Result<Self> {
        let mut file = BufReader::new(File::open(path)?);

        let header = HopHeader::read_from(&mut file)?;

        file.seek(SeekFrom::Start(header.index_offset))?;
        let mut index = HashMap::new();
        let mut read = 0;
        while read < header.index_len {
            let entry = HopIndexEntry::read_from(&mut file)?;
            read += entry.size() as u64;
            index.insert(entry.key_hash, entry);
        }

        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        Ok(Self { mmap, index })
    }

    pub fn get(&self, key: &str) -> Option<&[u8]> {
        let hash = hash_key(key.as_bytes());
        self.index.get(&hash).map(|entry| {
            let start = self.value_offset() as usize + entry.val_offset as usize;
            let end = start + entry.val_len as usize;
            &self.mmap[start..end]
        })
    }

    fn value_offset(&self) -> u64 {
        // you can cache it later, but for now read from the mmap header:
        let mut cursor = std::io::Cursor::new(&self.mmap[..4096]);
        HopHeader::read_from(&mut cursor).unwrap().value_offset
    }

    pub fn get_batch<'a>(&'a self, keys: &[&str]) -> Vec<Option<&'a [u8]>> {
        keys.iter().map(|&k| self.get(k)).collect()
    }

    pub fn entries(&self) -> impl Iterator<Item = &crate::format::HopIndexEntry> {
        self.index.values()
    }

    pub fn entry_count(&self) -> usize {
        self.index.len()
    }
}
