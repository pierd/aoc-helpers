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

pub trait Execute<S> {
    fn execute(&self, state: S) -> (S, Jump);
}

impl<S, E: Execute<S>> Execute<S> for Vec<E> {
    fn execute(&self, state: S) -> (S, Jump) {
        self.as_slice().execute(state)
    }
}

impl<S, E: Execute<S>> Execute<S> for &[E] {
    fn execute(&self, mut state: S) -> (S, Jump) {
        let mut instr_idx = 0;
        while let Some(instr) = self.get(instr_idx) {
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
        (state, Jump::Stop)
    }
}
