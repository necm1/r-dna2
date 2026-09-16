use r_dna2_isa::{
    fmt::{sop1, sop2, sopc, sopp},
    operand::Operand,
};

use crate::wave::Wave;

pub enum Step {
    Continue,
    EndPgm,
}

pub fn step(wave: &mut Wave, program: &[u32]) -> Step {
    let words = &program[wave.pc..];

    // TODO: refactor this...
    // TODO: match based on signature length sop1 > sopp > sopc > sopk > sop2
    if let Some((inst, len)) = sop1::decode(words) {
        println!("Executing instruction: {:?}", inst);

        match inst.op {
            sop1::Sop1Op::SMovB32 => {
                let value = read(wave, inst.ssrc0);
                write(wave, inst.sdst, value);
            }
            _ => panic!("Unsupported instruction: {:?}", inst.op),
        }

        wave.pc += len;
        Step::Continue
    } else if let Some((inst, len)) = sopp::decode(words) {
        println!("Executing instruction: {:?}", inst);

        match inst.op {
            sopp::SoppOp::SEndPgm => {
                println!("End of program reached.");
                return Step::EndPgm;
            }
            sopp::SoppOp::SBranch => {
                branch(wave, program, inst.simm16);

                return Step::Continue;
            }
            sopp::SoppOp::SCBranchScc0 => {
                if !wave.scc {
                    branch(wave, program, inst.simm16);
                } else {
                    wave.pc += len;
                }

                return Step::Continue;
            }
            sopp::SoppOp::SCBranchScc1 => {
                if wave.scc {
                    branch(wave, program, inst.simm16);
                } else {
                    wave.pc += len;
                }

                return Step::Continue;
            }
            _ => panic!("Unsupported instruction: {:?}", inst.op),
        }
    } else if let Some((inst, len)) = sopc::decode(words) {
        match inst.op {
            sopc::SopcOp::SCmpEqI32 => {
                wave.scc = (read(wave, inst.ssrc0) as i32) == (read(wave, inst.ssrc1) as i32)
            }
            sopc::SopcOp::SCmpLgI32 => {
                wave.scc = (read(wave, inst.ssrc0) as i32) != (read(wave, inst.ssrc1) as i32)
            }
            sopc::SopcOp::SCmpGtI32 => {
                wave.scc = (read(wave, inst.ssrc0) as i32) > (read(wave, inst.ssrc1) as i32)
            }
            sopc::SopcOp::SCmpGeI32 => {
                wave.scc = (read(wave, inst.ssrc0) as i32) >= (read(wave, inst.ssrc1) as i32)
            }
            sopc::SopcOp::SCmpLtI32 => {
                wave.scc = (read(wave, inst.ssrc0) as i32) < (read(wave, inst.ssrc1) as i32)
            }
            sopc::SopcOp::SCmpLeI32 => {
                wave.scc = (read(wave, inst.ssrc0) as i32) <= (read(wave, inst.ssrc1) as i32)
            }
            sopc::SopcOp::SCmpEqU32 => wave.scc = read(wave, inst.ssrc0) == read(wave, inst.ssrc1),
            sopc::SopcOp::SCmpLgU32 => wave.scc = read(wave, inst.ssrc0) != read(wave, inst.ssrc1),
            sopc::SopcOp::SCmpGtU32 => wave.scc = read(wave, inst.ssrc0) > read(wave, inst.ssrc1),
            sopc::SopcOp::SCmpGeU32 => wave.scc = read(wave, inst.ssrc0) >= read(wave, inst.ssrc1),
            sopc::SopcOp::SCmpLtU32 => wave.scc = read(wave, inst.ssrc0) < read(wave, inst.ssrc1),
            sopc::SopcOp::SCmpLeU32 => wave.scc = read(wave, inst.ssrc0) <= read(wave, inst.ssrc1),
            sopc::SopcOp::SBitCmp0B32 => {
                let bit = read(wave, inst.ssrc1) & 31;
                wave.scc = (read(wave, inst.ssrc0) >> bit) & 1 == 0;
            }
            sopc::SopcOp::SBitCmp1B32 => {
                let bit = read(wave, inst.ssrc1) & 31;
                wave.scc = (read(wave, inst.ssrc0) >> bit) & 1 == 1;
            }
            sopc::SopcOp::SBitCmp0B64 => {
                let bit = read(wave, inst.ssrc1) & 63;
                wave.scc = (read64(wave, inst.ssrc0) >> bit) & 1 == 0;
            }
            sopc::SopcOp::SBitCmp1B64 => {
                let bit = read(wave, inst.ssrc1) & 63;
                wave.scc = (read64(wave, inst.ssrc0) >> bit) & 1 == 1;
            }
            sopc::SopcOp::SCmpEqU64 => {
                wave.scc = read64(wave, inst.ssrc0) == read64(wave, inst.ssrc1)
            }
            sopc::SopcOp::SCmpLgU64 => {
                wave.scc = read64(wave, inst.ssrc0) != read64(wave, inst.ssrc1)
            }
        }

        wave.pc += len;
        Step::Continue
    } else if let Some((inst, len)) = sop2::decode(words) {
        match inst.op {
            sop2::Sop2Op::SAddU32 => {
                let s0 = read(wave, inst.ssrc0);
                let s1 = read(wave, inst.ssrc1);

                let (d, carry) = s0.overflowing_add(s1);

                write(wave, inst.sdst, d);
                wave.scc = carry;
            }
            sop2::Sop2Op::SSubU32 => {
                let s0 = read(wave, inst.ssrc0);
                let s1 = read(wave, inst.ssrc1);

                let (d, borrow) = s0.overflowing_sub(s1);

                write(wave, inst.sdst, d);
                wave.scc = borrow;
            }
        }
        wave.pc += len;
        Step::Continue
    } else {
        panic!("Unknown instruction at pc={}", wave.pc)
    }
}

fn branch(wave: &mut Wave, program: &[u32], simm16: i16) {
    let new_pc = (wave.pc as i64 + 1 + simm16 as i64) as usize;

    if new_pc >= program.len() {
        panic!(
            "Branch target out of bounds: pc={}, target={}",
            wave.pc, new_pc
        );
    }

    wave.pc = new_pc;
}

pub fn read(wave: &Wave, operand: Operand) -> u32 {
    match operand {
        Operand::Zero => 0,
        Operand::Int(k) => k as i32 as u32,
        Operand::Sgpr(idx) => wave.sgpr[idx as usize],
        Operand::Literal(lit) => lit,
        _ => panic!("Unsupported operand type for read: {:?}", operand),
    }
}

pub fn read64(wave: &Wave, operand: Operand) -> u64 {
    match operand {
        Operand::Sgpr(idx) => {
            let lo = wave.sgpr[idx as usize] as u64;
            let hi = wave.sgpr[idx as usize + 1] as u64;
            (hi << 32) | lo
        }
        Operand::Zero => 0,
        Operand::Int(k) => k as i64 as u64,
        Operand::Literal(lit) => lit as i32 as i64 as u64,
        _ => panic!("Unsupported operand type for read64: {:?}", operand),
    }
}

pub fn write(wave: &mut Wave, operand: Operand, value: u32) {
    match operand {
        Operand::Sgpr(idx) => wave.sgpr[idx as usize] = value,
        _ => panic!("Unsupported operand type for write: {:?}", operand),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_step() {
        // s_mov_b32 s0, 0x12345678  (literal)
        // s_mov_b32 s1, s0
        // s_endpgm
        let program = [0xBE8003FF, 0x12345678, 0xBE810300, 0xBF810000];

        let mut wave = Wave::new();

        while let Step::Continue = step(&mut wave, &program) {}

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

        while let Step::Continue = step(&mut wave, &program) {}

        assert_eq!(wave.sgpr[0], 0xFFFF_FFFF);
        assert!(wave.scc);
    }

    #[test]
    fn test_cmp_loop_ends_at_zero() {
        // Die "saubere" Schleife: läuft exakt 10x und endet mit s0 == 0.
        // s_mov_b32 s0, 10
        // loop:
        // s_sub_u32 s0, s0, 1
        // s_cmp_lg_u32 s0, 0      ; SCC = (s0 != 0)
        // s_cbranch_scc1 loop     ; simm16 = 1 - (3+1) = -3 = 0xFFFD
        // s_endpgm
        let program = [0xBE80038A, 0x80808100, 0xBF078000, 0xBF85FFFD, 0xBF810000];

        let mut wave = Wave::new();
        while let Step::Continue = step(&mut wave, &program) {}

        assert_eq!(wave.sgpr[0], 0);
        assert!(!wave.scc); // letzter Vergleich: 0 != 0 ist false
    }

    #[test]
    fn test_signed_vs_unsigned_compare() {
        // Dieselben Bits 0xFFFFFFFF, zwei Wahrheiten:
        // als i32 ist es -1 (also < 1), als u32 ist es 4294967295 (also > 1).
        // s_mov_b32 s0, -1        ; Inline-Konstante 193
        // s_cmp_lt_i32 s0, 1      ; SCC = (-1 < 1) = true
        // s_endpgm
        let program = [0xBE8003C1, 0xBF048100, 0xBF810000];
        let mut wave = Wave::new();
        while let Step::Continue = step(&mut wave, &program) {}
        assert_eq!(wave.sgpr[0], 0xFFFF_FFFF);
        assert!(wave.scc, "-1 < 1 muss als i32 wahr sein");

        // s_cmp_lt_u32 s0, 1      ; SCC = (0xFFFFFFFF < 1) = false
        let program = [0xBE8003C1, 0xBF0A8100, 0xBF810000];
        let mut wave = Wave::new();
        while let Step::Continue = step(&mut wave, &program) {}
        assert!(!wave.scc, "0xFFFFFFFF < 1 muss als u32 falsch sein");
    }

    /// Führt ein Mini-Programm auf einer vorbereiteten Wave bis s_endpgm aus.
    fn run(wave: &mut Wave, program: &[u32]) {
        while let Step::Continue = step(wave, program) {}
    }

    #[test]
    fn test_bitcmp_is_bit_index_not_mask() {
        // s_bitcmp1_b32 s0, s1 (0xBF0D0100): SCC = Bit Nr. s1 von s0.
        // s0 = 0b110, s1 = 1: Bit 1 von 0b110 ist 1 -> SCC = true.
        // (Der alte Masken-Test 0b110 & 0b001 haette false ergeben.)
        let mut wave = Wave::new();
        wave.sgpr[0] = 0b110;
        wave.sgpr[1] = 1;
        run(&mut wave, &[0xBF0D0100, 0xBF810000]);
        assert!(wave.scc);

        // Bit 0 von 0b110 ist 0 -> SCC = false.
        let mut wave = Wave::new();
        wave.sgpr[0] = 0b110;
        wave.sgpr[1] = 0;
        run(&mut wave, &[0xBF0D0100, 0xBF810000]);
        assert!(!wave.scc);
    }

    #[test]
    fn test_cmp_u64_uses_register_pair() {
        // s_cmp_eq_u64 s[0:1], s[2:3] (0xBF120200):
        // gleiche Low-Haelfte, unterschiedliche High-Haelfte -> NICHT gleich.
        // (Ein 32-Bit-Vergleich haette faelschlich true ergeben.)
        let mut wave = Wave::new();
        wave.sgpr[0] = 0xDEAD; // low  von s[0:1]
        wave.sgpr[1] = 1; // high von s[0:1]
        wave.sgpr[2] = 0xDEAD; // low  von s[2:3]
        wave.sgpr[3] = 0; // high von s[2:3]
        run(&mut wave, &[0xBF120200, 0xBF810000]);
        assert!(!wave.scc, "Paare unterscheiden sich im High-Teil");

        // Jetzt auch high gleich -> gleich.
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
        // s_bitcmp0_b64 s[0:1], s2 (0xBF0E0200) mit Bit-Index 32:
        // Bit 32 des Paars = Bit 0 von sgpr[1] = 1 -> SCC (== 0?) = false.
        let mut wave = Wave::new();
        wave.sgpr[0] = 0;
        wave.sgpr[1] = 1; // Bit 32 des 64-Bit-Werts
        wave.sgpr[2] = 32; // Bit-Index
        run(&mut wave, &[0xBF0E0200, 0xBF810000]);
        assert!(!wave.scc);
    }
}
