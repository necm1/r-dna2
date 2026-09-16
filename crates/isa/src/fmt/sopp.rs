use super::opcodes;
use crate::bits::bits;

pub const MASK: u32 = 0xFF80_0000;
pub const MAGIC: u32 = 0b1011_1111_1 << 23;

opcodes! {
    SoppOp {
        0x00 => SNop,
        0x01 => SEndPgm,
        0x02 => SBranch,
        0x03 => SWakeUp,
        0x04 => SCBranchScc0,
        0x05 => SCBranchScc1,
        0x06 => SCBranchVccz,
    }
}

#[derive(PartialEq, Debug)]
pub struct Sopp {
    pub op: SoppOp,
    pub simm16: i16,
}

pub fn decode(words: &[u32]) -> Option<(Sopp, usize)> {
    let word = *words.first()?;

    if word & MASK != MAGIC {
        return None;
    }

    let simm16 = bits(word, 15, 0) as u16 as i16;
    let op = SoppOp::decode(bits(word, 22, 16))?;

    Some((Sopp { op, simm16 }, 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_s_endpgm() {
        let (sopp, len) = decode(&[0xBF810000]).unwrap();
        assert_eq!(len, 1);
        assert_eq!(
            sopp,
            Sopp {
                op: SoppOp::SEndPgm,
                simm16: 0
            }
        );
    }

    #[test]
    fn decode_s_branch_backwards() {
        let (sopp, _) = decode(&[0xBF82FFFD]).unwrap();
        assert_eq!(
            sopp,
            Sopp {
                op: SoppOp::SBranch,
                simm16: -3
            }
        );
    }

    #[test]
    fn rejects_other_encoding() {
        assert_eq!(decode(&[0xBE800301]), None);
    }
}
