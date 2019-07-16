use super::Marker;

impl From<u8> for Marker {
    fn from(n: u8) -> Marker {
        match n {
            0x00...0x7f => Marker::PositiveFixInt(n),
            0x80...0x8f => Marker::FixMap(n & 0x0f),
            0x90...0x9f => Marker::FixArray(n & 0x0f),
            0xa0...0xbf => Marker::FixStr(n & 0x1f),
            0xc0 => Marker::Nil,
            0xc1 => Marker::Reserved,
            0xc2 => Marker::False,
            0xc3 => Marker::True,
            0xc4 => Marker::Bin8,
            _ => unimplemented!(),
        }
    }
}
