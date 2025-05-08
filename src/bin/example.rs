use anyhow::Result;
use hopfile::{reader::HopReader, writer::HopWriter};
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut entries = HashMap::new();
    entries.insert("dog".to_string(), b"bark".to_vec());
    entries.insert("cat".to_string(), b"meow".to_vec());
    entries.insert("fish".to_string(), b"glub".to_vec());

    // write file
    HopWriter::write_to_file("example.hop", &entries)?;
    println!("Wrote example.hop");

    // read file
    let store = HopReader::open("example.hop")?;
    let keys = ["dog", "fish", "cat", "goose"];
    let results = store.get_batch(&keys);

    for (key, val) in keys.iter().zip(results.iter()) {
        match val {
            Some(bytes) => println!("{key}: {:?}", std::str::from_utf8(bytes)),
            None => println!("{key}: [not found]"),
        }
    }

    Ok(())
}
