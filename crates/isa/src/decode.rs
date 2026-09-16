use crate::fmt::{sop1, sop2, sopc, sopp};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Sop1(sop1::Sop1),
    Sop2(sop2::Sop2),
    Sopc(sopc::Sopc),
    Sopp(sopp::Sopp),
}

pub fn decode(words: &[u32]) -> Option<(Instruction, usize)> {
    sop1::decode(words)
        .map(|(inst, len)| (Instruction::Sop1(inst), len))
        .or_else(|| sopc::decode(words).map(|(inst, len)| (Instruction::Sopc(inst), len)))
        .or_else(|| sopp::decode(words).map(|(inst, len)| (Instruction::Sopp(inst), len)))
        .or_else(|| sop2::decode(words).map(|(inst, len)| (Instruction::Sop2(inst), len)))
}
