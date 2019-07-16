use super::Marker;

impl From<u8> for Marker {
    fn from(n: u8) -> Marker {
        match n {
            0x00...0x7f => Marker::PositiveFixInt(n),
            0x80...0x8f => Marker::FixMap(n & 0x0f),
            _ => unimplemented!(),
        }
    }
}
