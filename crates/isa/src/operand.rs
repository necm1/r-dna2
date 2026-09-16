#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operand {
    Sgpr(u8),           // 0 - 105
    VccLo,              // 106
    VccHi,              // 107
    Ttmp(u8),           // 108 - 123
    M0,                 // Misc - 124
    Null,               // 125
    ExecLo,             // 126
    ExecHi,             // 127
    Zero,               // 128
    Int(i8),            // 129 - 192 POS & 193 - 208 NEG
    SharedBase,         // 235 - 32 or 64bit
    SharedLimit,        // 236 - 32 or 64bit
    PrivateBase,        // 237 - 32 or 64bit
    PrivateLimit,       // 238 - 32 or 64bit
    PopsExistingWaveId, // 239
    InlineFloat(f32),   // 240 - 248 - FLOAT
    Vccz,               // 251
    Execz,              // 252
    Scc,                // 253
    LiteralPending,     // 255 marker - value in next including decoder will replace to Literal(u32)
    Literal(u32),       // returned by decoder
    Vgpr(u16),          // 0 - 255 ^ & 256 - 511 VGPR
}

impl Operand {
    pub fn decode(value: u32) -> Self {
        match value {
            0..=105 => Operand::Sgpr(value as u8),
            106 => Operand::VccLo,
            107 => Operand::VccHi,
            108..=123 => Operand::Ttmp((value - 108) as u8),
            124 => Operand::M0,
            125 => Operand::Null,
            126 => Operand::ExecLo,
            127 => Operand::ExecHi,
            128 => Operand::Zero,
            129..=192 => Operand::Int((value - 128) as i8),
            193..=208 => Operand::Int(192_i32.wrapping_sub(value as i32) as i8),
            235 => Operand::SharedBase,
            236 => Operand::SharedLimit,
            237 => Operand::PrivateBase,
            238 => Operand::PrivateLimit,
            239 => Operand::PopsExistingWaveId,
            240..=248 => Operand::InlineFloat(Self::decode_inline_float(value)),
            251 => Operand::Vccz,
            252 => Operand::Execz,
            253 => Operand::Scc,
            255 => Operand::LiteralPending,
            256..=511 => Operand::Vgpr((value - 256) as u16),
            _ => panic!("Invalid operand value: {}", value),
        }
    }

    fn decode_inline_float(value: u32) -> f32 {
        match value {
            240 => 0.5,
            241 => -0.5,
            242 => 1.0,
            243 => -1.0,
            244 => 2.0,
            245 => -2.0,
            246 => 4.0,
            247 => -4.0,
            248 => f32::from_bits(0x3E22F983), // ~0.159
            _ => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_operands() {
        assert_eq!(Operand::decode(0), Operand::Sgpr(0));
        assert_eq!(Operand::decode(105), Operand::Sgpr(105));
        assert_eq!(Operand::decode(106), Operand::VccLo);
        assert_eq!(Operand::decode(107), Operand::VccHi);
        assert_eq!(Operand::decode(128), Operand::Zero);
        assert_eq!(Operand::decode(129), Operand::Int(1));
        assert_eq!(Operand::decode(193), Operand::Int(-1));
        assert_eq!(Operand::decode(192), Operand::Int(64));
        assert_eq!(Operand::decode(208), Operand::Int(-16));
        assert_eq!(Operand::decode(240), Operand::InlineFloat(0.5));
        assert_eq!(
            Operand::decode(248),
            Operand::InlineFloat(f32::from_bits(0x3E22F983))
        );
        assert_eq!(Operand::decode(255), Operand::LiteralPending);
        assert_eq!(Operand::decode(256), Operand::Vgpr(0));
        assert_eq!(Operand::decode(511), Operand::Vgpr(255));
    }
}
