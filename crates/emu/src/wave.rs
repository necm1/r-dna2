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
}
