use super::Marker;

impl From<u8> for Marker {
    fn from(n: u8) -> Marker {
        match n {
            0x00...0x7f => Marker::PositiveFixInt(n),
            0x80...0x8f => Marker::FixMap(n & 0x0f),
            0x90...0x9f => Marker::FixArray(n & 0x0f),
            0xa0...0xbf => Marker::FixStr(n & 0x1f),
            _ => unimplemented!(),
        }
    }
}
