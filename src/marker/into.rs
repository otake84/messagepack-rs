use super::Marker;

impl Into<u8> for Marker {
    fn into(self) -> u8 {
        match self {
            Marker::PositiveFixInt(n) => n,
            Marker::FixMap(n) => 0x80 | (n & 0x0f),
            Marker::FixArray(n) => 0x90 | (n & 0x0f),
            Marker::FixStr(n) => 0xa0 | (n & 0x1f),
            Marker::Nil => 0xc0,
            Marker::Reserved => 0xc1,
            Marker::False => 0xc2,
            Marker::True => 0xc3,
            Marker::Bin8 => 0xc4,
            Marker::Bin16 => 0xc5,
            Marker::Bin32 => 0xc6,
            Marker::Ext8 => 0xc7,
            Marker::Ext16 => 0xc8,
            Marker::Ext32 => 0xc9,
            Marker::Float32 => 0xca,
            Marker::Float64 => 0xcb,
            Marker::UInt8 => 0xcc,
            Marker::UInt16 => 0xcd,
            Marker::UInt32 => 0xce,
            Marker::UInt64 => 0xcf,
        }
    }
}
