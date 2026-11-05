#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    Header,
    Version,
    Range,
    Record,
    Duplicate,
    Capsule,
    Memory,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid initrd bundle: {self:?}")
    }
}
impl core::error::Error for Error {}
