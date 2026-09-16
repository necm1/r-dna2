use r_dna2_isa::decode::{Instruction, decode};

use crate::alu::scalar;
use crate::flow;
use crate::wave::Wave;

pub enum Step {
    Continue,
    EndPgm,
}

pub fn step(wave: &mut Wave, program: &[u32]) -> Step {
    let Some((inst, len)) = decode(&program[wave.pc..]) else {
        panic!("Unknown instruction at pc={}", wave.pc);
    };

    match inst {
        Instruction::Sop1(i) => {
            scalar::exec_sop1(wave, &i);
            wave.pc += len;
            Step::Continue
        }
        Instruction::Sop2(i) => {
            scalar::exec_sop2(wave, &i);
            wave.pc += len;
            Step::Continue
        }
        Instruction::Sopc(i) => {
            scalar::exec_sopc(wave, &i);
            wave.pc += len;
            Step::Continue
        }
        Instruction::Sopp(i) => flow::exec_sopp(wave, program, &i, len),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn run(wave: &mut Wave, program: &[u32]) {
        while let Step::Continue = step(wave, program) {}
    }

    #[test]
    fn test_step() {
        // s_mov_b32 s0, 0x12345678  (literal)
        // s_mov_b32 s1, s0
        // s_endpgm
        let program = [0xBE8003FF, 0x12345678, 0xBE810300, 0xBF810000];

        let mut wave = Wave::new();
        run(&mut wave, &program);

        assert_eq!(wave.sgpr[0], 0x12345678);
        assert_eq!(wave.sgpr[1], 0x12345678);
    }

    #[test]
    fn test_sub_branch_loop() {
        // s_mov_b32 s0, 10
        // loop:
        // s_sub_u32 s0, s0, 1
        // s_cbranch_scc0 loop
        // s_endpgm
        let program = [0xBE80038A, 0x80808100, 0xBF84FFFE, 0xBF810000];

        let mut wave = Wave::new();
        run(&mut wave, &program);

        assert_eq!(wave.sgpr[0], 0xFFFF_FFFF);
        assert!(wave.scc);
    }

    #[test]
    fn test_cmp_loop_ends_at_zero() {
        // s_mov_b32 s0, 10
        // loop:
        // s_sub_u32 s0, s0, 1
        // s_cmp_lg_u32 s0, 0      ; SCC = (s0 != 0)
        // s_cbranch_scc1 loop     ; simm16 = 1 - (3+1) = -3 = 0xFFFD
        // s_endpgm
        let program = [0xBE80038A, 0x80808100, 0xBF078000, 0xBF85FFFD, 0xBF810000];

        let mut wave = Wave::new();
        run(&mut wave, &program);

        assert_eq!(wave.sgpr[0], 0);
        assert!(!wave.scc);
    }

    #[test]
    fn test_signed_vs_unsigned_compare() {
        let program = [0xBE8003C1, 0xBF048100, 0xBF810000];
        let mut wave = Wave::new();
        run(&mut wave, &program);
        assert_eq!(wave.sgpr[0], 0xFFFF_FFFF);
        assert!(wave.scc, "-1 < 1 must be true as signed compare");

        let program = [0xBE8003C1, 0xBF0A8100, 0xBF810000];
        let mut wave = Wave::new();
        run(&mut wave, &program);
        assert!(!wave.scc, "0xFFFFFFFF < 1 must be false as u32");
    }

    #[test]
    fn test_bitcmp_is_bit_index_not_mask() {
        let mut wave = Wave::new();
        wave.sgpr[0] = 0b110;
        wave.sgpr[1] = 1;
        run(&mut wave, &[0xBF0D0100, 0xBF810000]);
        assert!(wave.scc);

        let mut wave = Wave::new();
        wave.sgpr[0] = 0b110;
        wave.sgpr[1] = 0;
        run(&mut wave, &[0xBF0D0100, 0xBF810000]);
        assert!(!wave.scc);
    }

    #[test]
    fn test_cmp_u64_uses_register_pair() {
        let mut wave = Wave::new();
        wave.sgpr[0] = 0xDEAD;
        wave.sgpr[1] = 1;
        wave.sgpr[2] = 0xDEAD;
        wave.sgpr[3] = 0;
        run(&mut wave, &[0xBF120200, 0xBF810000]);
        assert!(!wave.scc, "Pair differ in high half");

        let mut wave = Wave::new();
        wave.sgpr[0] = 0xDEAD;
        wave.sgpr[1] = 1;
        wave.sgpr[2] = 0xDEAD;
        wave.sgpr[3] = 1;
        run(&mut wave, &[0xBF120200, 0xBF810000]);
        assert!(wave.scc);
    }

    #[test]
    fn test_bitcmp_b64_reaches_high_half() {
        let mut wave = Wave::new();
        wave.sgpr[0] = 0;
        wave.sgpr[1] = 1;
        wave.sgpr[2] = 32;
        run(&mut wave, &[0xBF0E0200, 0xBF810000]);
        assert!(!wave.scc);
    }
}
