pub fn bits(word: u32, hi: u32, lo: u32) -> u32 {
    debug_assert!(hi >= lo && hi < 32);
    let width = hi - lo + 1;

    let mask = if width == 32 {
        u32::MAX
    } else {
        (1u32 << width) - 1
    };
    (word >> lo) & mask
}

pub fn bit(word: u32, pos: u32) -> bool {
    (word >> pos) & 1 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    // s_mov_b32 s0, s1
    const S_MOV_B32_S0_S1: u32 = 0xBE800301;

    #[test]
    fn extract_sop1_fields() {
        assert_eq!(bits(S_MOV_B32_S0_S1, 31, 23), 0b1011_1110_1);
        assert_eq!(bits(S_MOV_B32_S0_S1, 22, 16), 0); // SDST = s0
        assert_eq!(bits(S_MOV_B32_S0_S1, 15, 8), 3); // OP = s_mov_b32
        assert_eq!(bits(S_MOV_B32_S0_S1, 7, 0), 1); // SSRC0 = s1
    }

    #[test]
    fn misc() {
        assert_eq!(bits(u32::MAX, 31, 0), u32::MAX); // full word
        assert_eq!(bits(0x8000_0000, 31, 31), 1);
        assert!(bit(0b100, 2));
        assert!(!bit(0b100, 1));
    }
}
