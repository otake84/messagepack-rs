use super::Marker;

impl Into<u8> for Marker {
    fn into(self) -> u8 {
        match self {
            Marker::PositiveFixInt(n) => n,
        }
    }
}
