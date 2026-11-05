//! Versioned initrd containing Unit records and deduplicated images/resources.
//! The first eight bytes remain the kernel's capsule offset/length contract.
//! No deployment name, startup list, or host paths are stored here.

#[path = "bundle/archive.rs"]
mod archive;
#[path = "bundle/error.rs"]
mod error;
#[path = "bundle/file.rs"]
mod file;
#[path = "bundle/files.rs"]
mod files;
#[path = "bundle/kind.rs"]
mod kind;
#[path = "bundle/unit.rs"]
mod unit;
#[path = "bundle/units.rs"]
mod units;

pub use archive::Archive;
pub use error::Error;
pub use file::File;
pub use files::Files;
pub use kind::Kind;
pub use unit::Unit;
pub use units::Units;

use super::capsule::{Capsule, PAGE};
use alloc::vec::Vec;

pub const MAX_UNITS: usize = 96;
pub const MAX_FILES: usize = 256;
pub const MAX_RECORD: usize = 128 * 1024;
const HEADER: usize = 36;
const MAGIC: &[u8; 4] = b"VNI1";

/// Construct the entire initrd atomically in memory, including its boot capsule.
pub fn pack(
    units: &[Unit<'_>],
    files: &[File<'_>],
    boot_unit: usize,
    boot_image: usize,
    capsule: &[u8],
) -> Result<Vec<u8>, Error> {
    if units.is_empty()
        || units.len() > MAX_UNITS
        || files.is_empty()
        || files.len() > MAX_FILES
        || boot_unit >= units.len()
        || boot_image >= files.len()
        || Capsule::parse(capsule).is_none()
    {
        return Err(Error::Header);
    }
    let mut size = HEADER;
    for unit in units {
        if !name_valid(unit.id) || unit.record.is_empty() || unit.record.len() > MAX_RECORD {
            return Err(Error::Record);
        }
        size = size
            .checked_add(8)
            .and_then(|n| n.checked_add(unit.id.len()))
            .and_then(|n| n.checked_add(unit.record.len()))
            .ok_or(Error::Range)?;
    }
    for file in files {
        if !name_valid(file.name) || file.bytes.is_empty() {
            return Err(Error::Record);
        }
        size = size
            .checked_add(12)
            .and_then(|n| n.checked_add(file.name.len()))
            .and_then(|n| n.checked_add(file.bytes.len()))
            .ok_or(Error::Range)?;
    }
    let offset = size.checked_next_multiple_of(PAGE).ok_or(Error::Range)?;
    let total = offset
        .checked_add(capsule.len())
        .filter(|n| *n <= u32::MAX as usize)
        .ok_or(Error::Range)?;
    let mut out = Vec::new();
    out.try_reserve_exact(total).map_err(|_| Error::Memory)?;
    for value in [offset, capsule.len()] {
        number(&mut out, value)?;
    }
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    for value in [units.len(), files.len(), boot_unit, boot_image, size] {
        number(&mut out, value)?;
    }
    for unit in units {
        entry(&mut out, unit.id, unit.record)?;
    }
    for file in files {
        number(&mut out, file.kind as usize)?;
        entry(&mut out, file.name, file.bytes)?;
    }
    out.resize(offset, 0);
    out.extend_from_slice(capsule);
    Archive::parse(&out)?;
    Ok(out)
}

fn entry(out: &mut Vec<u8>, name: &str, bytes: &[u8]) -> Result<(), Error> {
    number(out, name.len())?;
    out.extend_from_slice(name.as_bytes());
    number(out, bytes.len())?;
    out.extend_from_slice(bytes);
    Ok(())
}

fn number(out: &mut Vec<u8>, value: usize) -> Result<(), Error> {
    out.extend_from_slice(
        &u32::try_from(value)
            .map_err(|_| Error::Range)?
            .to_le_bytes(),
    );
    Ok(())
}

fn name_valid(name: &str) -> bool {
    !name.is_empty() && name.len() <= 128 && !name.chars().any(char::is_control)
}

fn u32le(bytes: &[u8], at: usize) -> Result<usize, Error> {
    let raw = bytes
        .get(at..at.checked_add(4).ok_or(Error::Range)?)
        .ok_or(Error::Record)?;
    Ok(u32::from_le_bytes(raw.try_into().map_err(|_| Error::Record)?) as usize)
}

fn read<'a>(bytes: &'a [u8], at: &mut usize) -> Result<(&'a str, &'a [u8]), Error> {
    let len = u32le(bytes, *at)?;
    *at = at.checked_add(4).ok_or(Error::Range)?;
    let end = at.checked_add(len).ok_or(Error::Range)?;
    let name = core::str::from_utf8(bytes.get(*at..end).ok_or(Error::Record)?)
        .map_err(|_| Error::Record)?;
    if !name_valid(name) {
        return Err(Error::Record);
    }
    *at = end;
    let len = u32le(bytes, *at)?;
    *at = at.checked_add(4).ok_or(Error::Range)?;
    let end = at.checked_add(len).ok_or(Error::Range)?;
    let data = bytes
        .get(*at..end)
        .filter(|data| !data.is_empty())
        .ok_or(Error::Record)?;
    *at = end;
    Ok((name, data))
}
