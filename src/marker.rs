mod from;
mod into;

#[derive(Clone, Debug, PartialEq)]
pub enum Marker {
    PositiveFixInt(u8),
    FixMap(u8),
    FixArray(u8),
    FixStr(u8),
}
