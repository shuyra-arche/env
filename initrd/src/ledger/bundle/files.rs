use super::{Error, File, Kind};

#[derive(Clone)]
pub struct Files<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) at: usize,
    pub(super) left: usize,
}

impl<'a> Files<'a> {
    pub(super) fn read(&mut self) -> Result<File<'a>, Error> {
        let kind = Kind::try_from(super::u32le(self.bytes, self.at)?)?;
        self.at = self.at.checked_add(4).ok_or(Error::Range)?;
        let (name, bytes) = super::read(self.bytes, &mut self.at)?;
        Ok(File { kind, name, bytes })
    }
}
impl<'a> Iterator for Files<'a> {
    type Item = File<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.left == 0 {
            return None;
        }
        self.left -= 1;
        Some(self.read().expect("validated file record"))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.left, Some(self.left))
    }
}
impl ExactSizeIterator for Files<'_> {}
impl core::iter::FusedIterator for Files<'_> {}
