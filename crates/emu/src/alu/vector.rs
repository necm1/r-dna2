use r_dna2_isa::fmt::vop1;

use crate::wave::{LANES, Wave};

pub fn exec_vop1(wave: &mut Wave, inst: &vop1::Vop1) {
    match inst.op {
        vop1::Vop1Op::VNop => {}
        vop1::Vop1Op::VMovB32 => {
            for lane in 0..LANES {
                if wave.lane_active(lane) {
                    let value = wave.read_lane(inst.src0, lane);
                    wave.write_lane(inst.vdst, lane, value);
                }
            }
        }
        _ => panic!("Unsupported instruction: {:?}", inst.op),
    }
}
