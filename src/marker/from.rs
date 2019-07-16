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
            0xc5 => Marker::Bin16,
            0xc6 => Marker::Bin32,
            0xc7 => Marker::Ext8,
            0xc8 => Marker::Ext16,
            0xc9 => Marker::Ext32,
            0xca => Marker::Float32,
            0xcb => Marker::Float64,
            0xcc => Marker::UInt8,
            0xcd => Marker::UInt16,
            0xce => Marker::UInt32,
            0xcf => Marker::UInt64,
            0xd0 => Marker::Int8,
            0xd1 => Marker::Int16,
            0xd2 => Marker::Int32,
            0xd3 => Marker::Int64,
            0xd4 => Marker::FixExt1,
            _ => unimplemented!(),
        }
    }
}
