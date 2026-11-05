use super::{Error, File, Files, HEADER, Kind, MAGIC, MAX_FILES, MAX_UNITS, Unit, Units, u32le};
use crate::ledger::capsule::{Capsule, PAGE};

/// A completely validated borrowed initrd. Constructing this allocates nothing.
#[derive(Clone, Copy)]
pub struct Archive<'a> {
    body: &'a [u8],
    units: usize,
    files: usize,
    files_at: usize,
    boot_unit: usize,
    boot_image: usize,
}

impl<'a> Archive<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, Error> {
        if bytes.len() > u32::MAX as usize || bytes.get(8..12) != Some(MAGIC) {
            return Err(Error::Header);
        }
        if bytes.get(12..16) != Some(&[1, 0, 0, 0]) {
            return Err(Error::Version);
        }
        let units = u32le(bytes, 16)?;
        let files = u32le(bytes, 20)?;
        let boot_unit = u32le(bytes, 24)?;
        let boot_image = u32le(bytes, 28)?;
        let end = u32le(bytes, 32)?;
        if units == 0
            || units > MAX_UNITS
            || files == 0
            || files > MAX_FILES
            || boot_unit >= units
            || boot_image >= files
            || end < HEADER
        {
            return Err(Error::Header);
        }
        let offset = u32le(bytes, 0)?;
        let len = u32le(bytes, 4)?;
        if end.checked_next_multiple_of(PAGE) != Some(offset)
            || offset.checked_add(len) != Some(bytes.len())
            || !bytes
                .get(end..offset)
                .is_some_and(|padding| padding.iter().all(|b| *b == 0))
            || !bytes
                .get(offset..)
                .is_some_and(|capsule| Capsule::parse(capsule).is_some())
        {
            return Err(Error::Capsule);
        }
        let body = bytes.get(..end).ok_or(Error::Range)?;
        let mut records = Units {
            bytes: body,
            at: HEADER,
            left: units,
        };
        for index in 0..units {
            let current = records.read()?;
            let mut prior = Units {
                bytes: body,
                at: HEADER,
                left: index,
            };
            for _ in 0..index {
                if prior.read()?.id == current.id {
                    return Err(Error::Duplicate);
                }
            }
        }
        let files_at = records.at;
        let mut entries = Files {
            bytes: body,
            at: files_at,
            left: files,
        };
        for index in 0..files {
            let current = entries.read()?;
            if index == boot_image && current.kind == Kind::Resource {
                return Err(Error::Record);
            }
            let mut prior = Files {
                bytes: body,
                at: files_at,
                left: index,
            };
            for _ in 0..index {
                if prior.read()?.name == current.name {
                    return Err(Error::Duplicate);
                }
            }
        }
        if entries.at != end {
            return Err(Error::Record);
        }
        Ok(Self {
            body,
            units,
            files,
            files_at,
            boot_unit,
            boot_image,
        })
    }
    pub fn units(&self) -> Units<'a> {
        Units {
            bytes: self.body,
            at: HEADER,
            left: self.units,
        }
    }
    pub fn files(&self) -> Files<'a> {
        Files {
            bytes: self.body,
            at: self.files_at,
            left: self.files,
        }
    }
    pub fn system(&self) -> Unit<'a> {
        self.units()
            .nth(self.boot_unit)
            .expect("validated boot Unit")
    }
    pub fn image(&self) -> File<'a> {
        self.files()
            .nth(self.boot_image)
            .expect("validated boot image")
    }
    pub fn find(&self, name: &str) -> Option<File<'a>> {
        self.files().find(|file| file.name == name)
    }
}
