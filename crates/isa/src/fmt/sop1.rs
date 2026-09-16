use super::opcodes;
use crate::bits::bits;
use crate::operand::Operand;

pub const MASK: u32 = 0xFF80_0000;
pub const MAGIC: u32 = 0b1011_1110_1 << 23;

opcodes! {
    Sop1Op {
        0x03 => SMovB32,
        0x04 => SMovB64,
        0x05 => SCmovB32,
        0x06 => SCmovB64,
        0x07 => SNotB32,
        0x08 => SNotB64,
        0x09 => SWqmB32,
        0x0A => SWqmB64,
        0x0B => SBrevB32,
        0x0C => SBrevB64,
        0x0D => SBcnt0I32B32,
        0x0E => SBcnt0I32B64,
        0x0F => SBcnt1I32B32,
        0x10 => SBcnt1I32B64,
        0x11 => SFf0I32B32,
        0x12 => SFf0I32B64,
        0x13 => SFf1I32B32,
        0x14 => SFf1I32B64,
        0x15 => SFlbitI32B32,
        0x16 => SFlbitI32B64,
    }
}

#[derive(PartialEq, Debug)]
pub struct Sop1 {
    pub op: Sop1Op,
    pub sdst: Operand,
    pub ssrc0: Operand,
}

pub fn decode(words: &[u32]) -> Option<(Sop1, usize)> {
    let word = *words.first()?;

    if word & MASK != MAGIC {
        return None;
    }

    let sdst = Operand::decode(bits(word, 22, 16));
    let op = Sop1Op::decode(bits(word, 15, 8))?;
    let mut ssrc0 = Operand::decode(bits(word, 7, 0));

    let mut len = 1;
    if ssrc0 == Operand::LiteralPending {
        let lit = *words.get(1)?;
        ssrc0 = Operand::Literal(lit);
        len = 2;
    }

    Some((Sop1 { op, sdst, ssrc0 }, len))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_s_mov_b32() {
        let words = [0xBE800301];
        let (sop1, len) = decode(&words).unwrap();
        assert_eq!(len, 1);
        assert_eq!(
            sop1,
            Sop1 {
                op: Sop1Op::SMovB32,
                sdst: Operand::Sgpr(0),
                ssrc0: Operand::Sgpr(1),
            }
        );
    }

    #[test]
    fn decode_s_mov_b32_literal() {
        let words = [0xBE8003FF, 0x12345678];
        let (sop1, len) = decode(&words).unwrap();

        assert_eq!(len, 2);
        assert_eq!(
            sop1,
            Sop1 {
                op: Sop1Op::SMovB32,
                sdst: Operand::Sgpr(0),
                ssrc0: Operand::Literal(0x12345678),
            }
        );
    }

    #[test]
    fn decode_truncated_literal() {
        assert_eq!(decode(&[0xBE8003FF]), None);
    }

    #[test]
    fn rejects_other_encoding() {
        assert_eq!(decode(&[0xBF810000]), None);
    }
}
