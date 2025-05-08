use anyhow::Result;
use hopfile::{format::HopHeader, reader::HopReader};
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<()> {
    let file = File::open("example.hop")?;
    let mut reader = BufReader::new(file);

    let header = HopHeader::read_from(&mut reader)?;
    println!("=== HOP HEADER ===");
    println!("Index Offset:      {}", header.index_offset);
    println!("Index Length:      {}", header.index_len);
    println!("Metadata Offset:   {}", header.meta_offset);
    println!("Metadata Length:   {}", header.meta_len);
    println!("Value Offset:      {}", header.value_offset);
    println!();

    let store = HopReader::open("example.hop")?;
    println!("Index entries loaded: {}", store.entry_count());

    println!("--- Keys ---");
    for entry in store.entries() {
        let key = String::from_utf8_lossy(&entry.key);
        println!("{key} ({} bytes)", entry.val_len);
    }

    Ok(())
}
