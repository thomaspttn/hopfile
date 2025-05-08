use anyhow::Result;
use hopfile::format::{HopHeader, HopIndexEntry};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom, Write};

fn main() -> Result<()> {
    let mut index = Vec::new();
    let mut value_data = Vec::new();

    let entries: HashMap<&str, &str> = [("dog", "bark"), ("cat", "meow"), ("fish", "glub")]
        .into_iter()
        .collect();

    for (k, v) in entries {
        let val_offset = value_data.len() as u64;
        let val_bytes = v.as_bytes();
        value_data.extend_from_slice(val_bytes);
        let val_len = val_bytes.len() as u32;

        index.push(HopIndexEntry {
            key_hash: fxhash::hash64(k.as_bytes()),
            key_len: k.len() as u16,
            val_offset,
            val_len,
            meta_offset: 0,
            key: k.as_bytes().to_vec(),
        });
    }

    let mut file = BufWriter::new(File::create("example.hop")?);
    file.seek(SeekFrom::Start(4096))?;

    let index_offset = file.stream_position()?;
    for entry in &index {
        entry.write_to(&mut file)?;
    }
    let index_len = file.stream_position()? - index_offset;

    let meta_offset = file.stream_position()?;
    let meta_len = 0;

    let value_offset = file.stream_position()?;
    file.write_all(&value_data)?;

    let header = HopHeader {
        index_offset,
        index_len,
        meta_offset,
        meta_len,
        value_offset,
    };
    file.seek(SeekFrom::Start(0))?;
    header.write_to(&mut file)?;

    file.flush()?;
    Ok(())
}
