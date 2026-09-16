use r_dna2_isa::fmt::sopp;

use crate::exec::Step;
use crate::wave::Wave;

pub fn exec_sopp(wave: &mut Wave, program: &[u32], inst: &sopp::Sopp, len: usize) -> Step {
    match inst.op {
        sopp::SoppOp::SEndPgm => return Step::EndPgm,
        sopp::SoppOp::SNop => wave.pc += len,
        sopp::SoppOp::SBranch => branch(wave, program, inst.simm16),
        sopp::SoppOp::SCBranchScc0 => cbranch(wave, program, inst.simm16, len, !wave.scc),
        sopp::SoppOp::SCBranchScc1 => cbranch(wave, program, inst.simm16, len, wave.scc),
        _ => panic!("Unsupported instruction: {:?}", inst.op),
    }
    Step::Continue
}

fn cbranch(wave: &mut Wave, program: &[u32], simm16: i16, len: usize, taken: bool) {
    if taken {
        branch(wave, program, simm16);
    } else {
        wave.pc += len;
    }
}

/// PDF (SOPP): Sprungziel = PC der Folge-Instruktion + SIMM16 (in DWORDs).
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
