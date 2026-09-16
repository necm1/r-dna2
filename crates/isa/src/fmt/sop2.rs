use super::opcodes;
use crate::bits::bits;
use crate::operand::Operand;

pub const MASK: u32 = 0xC000_0000;
pub const MAGIC: u32 = 0x8000_0000;

opcodes! {
    Sop2Op {
        0x00 => SAddU32,
        0x01 => SSubU32,
    }
}

#[derive(PartialEq, Debug)]
pub struct Sop2 {
    pub op: Sop2Op,
    pub sdst: Operand,
    pub ssrc0: Operand,
    pub ssrc1: Operand,
    // pub encoding: u32
}

pub fn decode(words: &[u32]) -> Option<(Sop2, usize)> {
    let word = *words.first()?;

    if word & MASK != MAGIC {
        return None;
    }

    let sdst = Operand::decode(bits(word, 22, 16));
    let op = Sop2Op::decode(bits(word, 29, 23))?;
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

    Some((
        Sop2 {
            op,
            sdst,
            ssrc0,
            ssrc1,
        },
        len,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_s_add_u32() {
        let words = [0x80000000];
        let (sop2, len) = decode(&words).unwrap();
        assert_eq!(len, 1);
        assert_eq!(
            sop2,
            Sop2 {
                op: Sop2Op::SAddU32,
                sdst: Operand::Sgpr(0),
                ssrc0: Operand::Sgpr(0),
                ssrc1: Operand::Sgpr(0),
            }
        );
    }

    #[test]
    fn decode_s_sub_u32() {
        let words = [0x80800000];
        let (sop2, len) = decode(&words).unwrap();
        assert_eq!(len, 1);
        assert_eq!(
            sop2,
            Sop2 {
                op: Sop2Op::SSubU32,
                sdst: Operand::Sgpr(0),
                ssrc0: Operand::Sgpr(0),
                ssrc1: Operand::Sgpr(0),
            }
        );
    }

    #[test]
    fn rejects_other_encoding() {
        assert_eq!(decode(&[0xBF810000]), None);
    }
}
