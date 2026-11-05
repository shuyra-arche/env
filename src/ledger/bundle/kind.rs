#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Kind {
    User = 0,
    Supervisor = 1,
    Resource = 2,
}

impl TryFrom<usize> for Kind {
    type Error = super::Error;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::User),
            1 => Ok(Self::Supervisor),
            2 => Ok(Self::Resource),
            _ => Err(super::Error::Record),
        }
    }
}
