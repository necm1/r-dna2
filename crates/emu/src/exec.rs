use r_dna2_isa::{
    fmt::{sop1, sopp},
    operand::Operand,
};

use crate::wave::Wave;

pub enum Step {
    Continue,
    EndPgm,
}

pub fn step(wave: &mut Wave, program: &[u32]) -> Step {
    let words = &program[wave.pc..];

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
                let simm16 = inst.simm16;

                let new_pc = (wave.pc as i64 + 1 + simm16 as i64) as usize;

                if new_pc >= program.len() {
                    panic!(
                        "Branch target out of bounds: pc={}, target={}",
                        wave.pc, new_pc
                    );
                }

                wave.pc = new_pc;

                return Step::Continue;
            }
            _ => panic!("Unsupported instruction: {:?}", inst.op),
        }
    } else {
        panic!("Unknown instruction at pc={}", wave.pc)
    }
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
}
