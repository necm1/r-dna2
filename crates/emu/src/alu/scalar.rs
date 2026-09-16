use r_dna2_isa::fmt::{sop1, sop2, sopc};

use crate::wave::Wave;

pub fn exec_sop1(wave: &mut Wave, inst: &sop1::Sop1) {
    match inst.op {
        sop1::Sop1Op::SMovB32 => {
            let value = wave.read(inst.ssrc0);
            wave.write(inst.sdst, value);
        }
        _ => panic!("Unsupported instruction: {:?}", inst.op),
    }
}

pub fn exec_sop2(wave: &mut Wave, inst: &sop2::Sop2) {
    match inst.op {
        sop2::Sop2Op::SAddU32 => {
            let s0 = wave.read(inst.ssrc0);
            let s1 = wave.read(inst.ssrc1);

            let (d, carry) = s0.overflowing_add(s1);

            wave.write(inst.sdst, d);
            wave.scc = carry;
        }
        sop2::Sop2Op::SSubU32 => {
            let s0 = wave.read(inst.ssrc0);
            let s1 = wave.read(inst.ssrc1);

            let (d, borrow) = s0.overflowing_sub(s1);

            wave.write(inst.sdst, d);
            wave.scc = borrow;
        }
    }
}

pub fn exec_sopc(wave: &mut Wave, inst: &sopc::Sopc) {
    use sopc::SopcOp::*;

    let s0 = wave.read(inst.ssrc0);
    let s1 = wave.read(inst.ssrc1);

    wave.scc = match inst.op {
        SCmpEqI32 => (s0 as i32) == (s1 as i32),
        SCmpLgI32 => (s0 as i32) != (s1 as i32),
        SCmpGtI32 => (s0 as i32) > (s1 as i32),
        SCmpGeI32 => (s0 as i32) >= (s1 as i32),
        SCmpLtI32 => (s0 as i32) < (s1 as i32),
        SCmpLeI32 => (s0 as i32) <= (s1 as i32),
        SCmpEqU32 => s0 == s1,
        SCmpLgU32 => s0 != s1,
        SCmpGtU32 => s0 > s1,
        SCmpGeU32 => s0 >= s1,
        SCmpLtU32 => s0 < s1,
        SCmpLeU32 => s0 <= s1,
        SBitCmp0B32 => (s0 >> (s1 & 31)) & 1 == 0,
        SBitCmp1B32 => (s0 >> (s1 & 31)) & 1 == 1,
        SBitCmp0B64 => (wave.read64(inst.ssrc0) >> (s1 & 63)) & 1 == 0,
        SBitCmp1B64 => (wave.read64(inst.ssrc0) >> (s1 & 63)) & 1 == 1,
        SCmpEqU64 => wave.read64(inst.ssrc0) == wave.read64(inst.ssrc1),
        SCmpLgU64 => wave.read64(inst.ssrc0) != wave.read64(inst.ssrc1),
    };
}
