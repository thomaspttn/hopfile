use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Seek, SeekFrom, Write};

pub const HOP_MAGIC: &[u8; 4] = b"HOP1";
pub const HOP_HEADER_SIZE: usize = 4096;

#[derive(Debug)]
pub struct HopHeader {
    pub index_offset: u64,
    pub index_len: u64,
    pub meta_offset: u64,
    pub meta_len: u64,
    pub value_offset: u64,
}

impl HopHeader {
    pub fn write_to<W: Write + Seek>(&self, mut w: W) -> anyhow::Result<()> {
        w.seek(SeekFrom::Start(0))?;
        w.write_all(HOP_MAGIC)?;
        w.write_u64::<LittleEndian>(self.index_offset)?;
        w.write_u64::<LittleEndian>(self.index_len)?;
        w.write_u64::<LittleEndian>(self.meta_offset)?;
        w.write_u64::<LittleEndian>(self.meta_len)?;
        w.write_u64::<LittleEndian>(self.value_offset)?;

        // pad remainder to HOP_HEADER_SIZE
        let written = 4 + 8 * 5;
        let padding = HOP_HEADER_SIZE - written;
        w.write_all(&vec![0u8; padding])?;
        Ok(())
    }

    pub fn read_from<R: Read + Seek>(mut r: R) -> anyhow::Result<Self> {
        r.seek(SeekFrom::Start(0))?;
        let mut magic = [0u8; 4];
        r.read_exact(&mut magic)?;
        if &magic != HOP_MAGIC {
            anyhow::bail!("Invalid file magic");
        }

        Ok(Self {
            index_offset: r.read_u64::<LittleEndian>()?,
            index_len: r.read_u64::<LittleEndian>()?,
            meta_offset: r.read_u64::<LittleEndian>()?,
            meta_len: r.read_u64::<LittleEndian>()?,
            value_offset: r.read_u64::<LittleEndian>()?,
        })
    }
}

#[derive(Debug)]
pub struct HopIndexEntry {
    pub key_hash: u64,
    pub key_len: u16,
    pub val_offset: u64,
    pub val_len: u32,
    pub meta_offset: u64, // 0 means no metadata
    pub key: Vec<u8>,
}

impl HopIndexEntry {
    pub fn write_to<W: Write>(&self, mut w: W) -> anyhow::Result<()> {
        w.write_u64::<LittleEndian>(self.key_hash)?;
        w.write_u16::<LittleEndian>(self.key_len)?;
        w.write_u64::<LittleEndian>(self.val_offset)?;
        w.write_u32::<LittleEndian>(self.val_len)?;
        w.write_u64::<LittleEndian>(self.meta_offset)?;
        w.write_all(&self.key)?;
        Ok(())
    }

    pub fn read_from<R: Read>(mut r: R) -> anyhow::Result<Self> {
        let key_hash = r.read_u64::<LittleEndian>()?;
        let key_len = r.read_u16::<LittleEndian>()?;
        let val_offset = r.read_u64::<LittleEndian>()?;
        let val_len = r.read_u32::<LittleEndian>()?;
        let meta_offset = r.read_u64::<LittleEndian>()?;
        let mut key = vec![0u8; key_len as usize];
        r.read_exact(&mut key)?;

        Ok(Self {
            key_hash,
            key_len,
            val_offset,
            val_len,
            meta_offset,
            key,
        })
    }

    pub fn size(&self) -> usize {
        8 + 2 + 8 + 4 + 8 + self.key.len()
    }
}
