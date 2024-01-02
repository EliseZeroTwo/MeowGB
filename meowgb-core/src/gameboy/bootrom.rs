use std::{path::Path, io::Read};

use sha1::{Digest, Sha1};

#[derive(Debug, thiserror::Error)]
pub enum BootromParseError {
    #[error("Bootrom file cannot be found")]
    BootromNotFound,
    #[error("IO error whilst reading bootrom: {0}")]
    Io(#[from] std::io::Error),
    #[error("Bootrom size is {0} bytes, expected 256 bytes")]
    InvalidSize(u64),
    #[error("Bootrom has an invalid SHA1 (expected \"4ed31ec6b0b175bb109c0eb5fd3d193da823339f\")")]
    InvalidHash,
    #[error("Failed to open bootrom file: {0}")]
    FileOpen(std::io::Error),
    #[error("Failed to read bootrom file: {0}")]
    FileRead(std::io::Error),
}

pub fn verify_parse_bootrom(path: &Path) -> Result<[u8; 0x100], BootromParseError> {
    if !path.is_file() {
        return Err(BootromParseError::BootromNotFound);
    }

    let mut bootrom_slice = [0u8; 0x100];

    let mut file = std::fs::File::open(path).map_err(BootromParseError::FileOpen)?;
    let metadata = file.metadata()?;

    if metadata.len() != 256 {
        return Err(BootromParseError::InvalidSize(metadata.len()));
    }

    file.read_exact(&mut bootrom_slice).map_err(BootromParseError::FileRead)?;

    let mut hash_ctx = Sha1::new();
    hash_ctx.update(&bootrom_slice);
    let digest = hash_ctx.finalize();

    if digest.as_slice()
        != b"\x4e\xd3\x1e\xc6\xb0\xb1\x75\xbb\x10\x9c\x0e\xb5\xfd\x3d\x19\x3d\xa8\x23\x33\x9f"
    {
        return Err(BootromParseError::InvalidHash);
    }

    Ok(bootrom_slice)
}