use super::opcodes;
use crate::bits::bits;
use crate::operand::Operand;

pub const MASK: u32 = 0xFE00_0000;
pub const MAGIC: u32 = 0x7E00_0000;

opcodes! {
    Vop1Op {
        0x00 => VNop,
        0x01 => VMovB32,
        0x02 => VReadFirstLaneB32,
        0x03 => VCvtI32F64,
        0x04 => VCvtF64I32,
    }
}

#[derive(PartialEq, Debug)]
pub struct Vop1 {
    pub op: Vop1Op,
    pub vdst: u8,
    pub src0: Operand,
}

pub fn decode(words: &[u32]) -> Option<(Vop1, usize)> {
    let word = *words.first()?;

    if word & MASK != MAGIC {
        return None;
    }

    let vdst = bits(word, 24, 17) as u8;
    let op = Vop1Op::decode(bits(word, 16, 9))?;
    let mut src0 = Operand::decode(bits(word, 8, 0));

    let mut len = 1;
    if src0 == Operand::LiteralPending {
        let lit = *words.get(1)?;
        src0 = Operand::Literal(lit);
        len = 2;
    }

    Some((Vop1 { op, vdst, src0 }, len))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_other_encoding() {
        assert_eq!(decode(&[0xBF810000]), None);
    }
}
