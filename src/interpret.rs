#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Jump {
    Absolute(usize),
    Relative(isize),
    Stop,
}

impl Default for Jump {
    fn default() -> Self {
        Self::Relative(1)
    }
}

pub trait Instruction<S> {
    fn execute(&self, state: S) -> (S, Jump);
}

pub fn execute_all<S, I: Instruction<S>>(initial: S, instrs: &[I]) -> S {
    let mut state = initial;
    let mut instr_idx = 0;
    while let Some(instr) = instrs.get(instr_idx) {
        let (new_state, jump) = instr.execute(state);
        state = new_state;
        match jump {
            Jump::Absolute(idx) => instr_idx = idx,
            Jump::Relative(d) => {
                if let Ok(new_idx) = usize::try_from(instr_idx as isize + d) {
                    instr_idx = new_idx
                } else {
                    break;
                }
            }
            Jump::Stop => break,
        }
    }
    state
}

pub fn execute_all_default<S: Default, I: Instruction<S>>(instrs: &[I]) -> S {
    execute_all(Default::default(), instrs)
}
