use crate::fmt::{sop1, sop2, sopc, sopp};

pub enum Instruction {
    Sop1(sop1::Sop1),
    Sop2(sop2::Sop2),
    Sopc(sopc::Sopc),
    Sopp(sopp::Sopp),
}
