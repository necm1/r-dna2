use super::opcodes;
use crate::bits::bits;
use crate::operand::Operand;

pub const MASK: u32 = 0xFF80_0000;
pub const MAGIC: u32 = 0b1011_1111_0 << 23;

opcodes! {
    SopcOp {
        0x00 => SCmpEqI32,
        0x01 => SCmpLgI32,
        0x02 => SCmpGtI32,
        0x03 => SCmpGeI32,
        0x04 => SCmpLtI32,
        0x05 => SCmpLeI32,
        0x06 => SCmpEqU32,
        0x07 => SCmpLgU32,
        0x08 => SCmpGtU32,
        0x09 => SCmpGeU32,
        0x0A => SCmpLtU32,
        0x0B => SCmpLeU32,
        0x0C => SBitCmp0B32,
        0x0D => SBitCmp1B32,
        0x0E => SBitCmp0B64,
        0x0F => SBitCmp1B64,
        0x12 => SCmpEqU64,
        0x13 => SCmpLgU64,
    }
}

#[derive(PartialEq, Debug)]
pub struct Sopc {
    pub op: SopcOp,
    pub ssrc0: Operand,
    pub ssrc1: Operand,
    // pub encoding
}

pub fn decode(words: &[u32]) -> Option<(Sopc, usize)> {
    let word = *words.first()?;

    if word & MASK != MAGIC {
        return None;
    }

    let op = SopcOp::decode(bits(word, 22, 16))?;
    let mut ssrc0 = Operand::decode(bits(word, 7, 0));
    let mut ssrc1 = Operand::decode(bits(word, 15, 8));

    let mut len = 1;

    if ssrc0 == Operand::LiteralPending || ssrc1 == Operand::LiteralPending {
        let lit = *words.get(1)?;
        len = 2;
        if ssrc0 == Operand::LiteralPending {
            ssrc0 = Operand::Literal(lit);
        }
        if ssrc1 == Operand::LiteralPending {
            ssrc1 = Operand::Literal(lit);
        }
    }

    Some((Sopc { op, ssrc0, ssrc1 }, len))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_s_cmp_eq_u32() {
        let (sopc, len) = decode(&[0xBF060100]).unwrap();
        assert_eq!(len, 1);
        assert_eq!(
            sopc,
            Sopc {
                op: SopcOp::SCmpEqU32,
                ssrc0: Operand::Sgpr(0),
                ssrc1: Operand::Sgpr(1),
            }
        );
    }

    #[test]
    fn decode_s_cmp_lg_u32_literal() {
        let (sopc, len) = decode(&[0xBF07FF00, 0x12345678]).unwrap();
        assert_eq!(len, 2);
        assert_eq!(sopc.op, SopcOp::SCmpLgU32);
        assert_eq!(sopc.ssrc1, Operand::Literal(0x12345678));
    }

    #[test]
    fn rejects_other_encoding() {
        assert_eq!(decode(&[0xBF810000]), None);
    }
}
