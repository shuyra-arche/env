#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Unit<'a> {
    pub id: &'a str,
    pub record: &'a [u8],
}
