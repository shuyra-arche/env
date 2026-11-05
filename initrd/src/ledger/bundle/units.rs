use super::{Error, Unit};

#[derive(Clone)]
pub struct Units<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) at: usize,
    pub(super) left: usize,
}

impl<'a> Units<'a> {
    pub(super) fn read(&mut self) -> Result<Unit<'a>, Error> {
        let (id, record) = super::read(self.bytes, &mut self.at)?;
        if record.len() > super::MAX_RECORD {
            return Err(Error::Record);
        }
        Ok(Unit { id, record })
    }
}

impl<'a> Iterator for Units<'a> {
    type Item = Unit<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.left == 0 {
            return None;
        }
        self.left -= 1;
        // Only Archive can create a public iterator, after validating all entries.
        Some(self.read().expect("validated Unit record"))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.left, Some(self.left))
    }
}
impl ExactSizeIterator for Units<'_> {}
impl core::iter::FusedIterator for Units<'_> {}
