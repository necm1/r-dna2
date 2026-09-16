use crate::bits::bits;
use crate::operand::Operand;

#[derive(PartialEq, Debug)]
pub enum Sop1Op {
    SMovB32,
    SMovB64,
    SCMovB32,
    SCmovB64,
    SNotB32,
    SNotB64,
    SWqmB32,
    SWqmB64,
    SBrevB32,
    SBrevB64,
    SBcnt0i32B32,
    SBcnt0i32B64,
    SBcnt1i32B32,
    SBcnt1i32B64,
    SFF0i32B32,
    SFF0i32B64,
    SFF1i32B32,
    SFF1i32B64,
    SFLBiti32B32,
    SFLBiti32B64,
}

#[derive(PartialEq, Debug)]
pub struct Sop1 {
    pub op: Sop1Op,
    pub sdst: Operand,
    pub ssrc0: Operand,
}

pub fn decode(words: &[u32]) -> Option<(Sop1, usize)> {
    let word = *words.first()?;

    if bits(word, 31, 23) != 0b101111101 {
        return None;
    }

    let sdst = Operand::decode(bits(word, 22, 16));

    let op = match bits(word, 15, 8) {
        0x03 => Sop1Op::SMovB32,
        0x04 => Sop1Op::SMovB64,
        0x05 => Sop1Op::SCMovB32,
        0x06 => Sop1Op::SCmovB64,
        0x07 => Sop1Op::SNotB32,
        0x08 => Sop1Op::SNotB64,
        0x09 => Sop1Op::SWqmB32,
        0x0A => Sop1Op::SWqmB64,
        0x0B => Sop1Op::SBrevB32,
        0x0C => Sop1Op::SBrevB64,
        0x0D => Sop1Op::SBcnt0i32B32,
        0x0E => Sop1Op::SBcnt0i32B64,
        0x0F => Sop1Op::SBcnt1i32B32,
        0x10 => Sop1Op::SBcnt1i32B64,
        0x11 => Sop1Op::SFF0i32B32,
        0x12 => Sop1Op::SFF0i32B64,
        0x13 => Sop1Op::SFF1i32B32,
        0x14 => Sop1Op::SFF1i32B64,
        0x15 => Sop1Op::SFLBiti32B32,
        0x16 => Sop1Op::SFLBiti32B64,
        _ => return None,
    };

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

        assert_eq!(sop1.ssrc0, Operand::Literal(0x12345678));
    }

    #[test]
    fn decode_truncated_literal() {
        assert_eq!(decode(&[0xBE8003FF]), None);
    }
}
