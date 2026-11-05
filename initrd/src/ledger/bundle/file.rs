#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct File<'a> {
    pub kind: super::Kind,
    pub name: &'a str,
    pub bytes: &'a [u8],
}
