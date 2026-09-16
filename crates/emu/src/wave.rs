use r_dna2_isa::operand::Operand;

pub struct Wave {
    pub pc: usize,
    pub sgpr: [u32; 106],
    pub scc: bool,
}

impl Wave {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sgpr: [0; 106],
            scc: false,
        }
    }

    pub fn read(&self, operand: Operand) -> u32 {
        match operand {
            Operand::Zero => 0,
            Operand::Int(k) => k as i32 as u32,
            Operand::Sgpr(idx) => self.sgpr[idx as usize],
            Operand::Literal(lit) => lit,
            _ => panic!("Unsupported operand type for read: {:?}", operand),
        }
    }

    pub fn read64(&self, operand: Operand) -> u64 {
        match operand {
            Operand::Sgpr(idx) => {
                let lo = self.sgpr[idx as usize] as u64;
                let hi = self.sgpr[idx as usize + 1] as u64;
                (hi << 32) | lo
            }
            Operand::Zero => 0,
            Operand::Int(k) => k as i64 as u64,
            Operand::Literal(lit) => lit as i32 as i64 as u64,
            _ => panic!("Unsupported operand type for read64: {:?}", operand),
        }
    }

    pub fn write(&mut self, operand: Operand, value: u32) {
        match operand {
            Operand::Sgpr(idx) => self.sgpr[idx as usize] = value,
            _ => panic!("Unsupported operand type for write: {:?}", operand),
        }
    }
}
