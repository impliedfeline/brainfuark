#[derive(Debug)]
pub struct ProgramState<const LEN: usize> {
    pub data: [u8; LEN],
    pub data_ptr: usize,
    pub instr_ptr: usize,
}

impl<const LEN: usize> Default for ProgramState<LEN> {
    fn default() -> Self {
        Self {
            data: [0u8; LEN],
            data_ptr: Default::default(),
            instr_ptr: Default::default(),
        }
    }
}
