use std::collections::VecDeque;

pub trait Queue<S>: Default {
    fn push(&mut self, state: S);
    fn pop(&mut self) -> Option<S>;
}

impl<S> Queue<S> for Vec<S> {
    fn push(&mut self, state: S) {
        self.push(state);
    }

    fn pop(&mut self) -> Option<S> {
        self.pop()
    }
}

impl<S> Queue<S> for VecDeque<S> {
    fn push(&mut self, state: S) {
        self.push_back(state);
    }

    fn pop(&mut self) -> Option<S> {
        self.pop_front()
    }
}

pub enum VisitDecision<R, T> {
    Break(R),
    Continue,
    Next(T),
}

pub trait Generator<T> {
    fn generate<F: FnMut(T)>(&mut self, callback: F);
}

impl<T, I> Generator<T> for I
where
    I: Iterator<Item = T>,
{
    fn generate<F: FnMut(T)>(&mut self, mut callback: F) {
        for x in self {
            callback(x);
        }
    }
}

pub trait Walker<S> {
    type NextGenerator: Generator<S>;
    type Result;
    fn visit(&mut self, state: &S) -> VisitDecision<Self::Result, Self::NextGenerator>;
}

pub fn walk<S, W, Q>(walker: &mut W, initial_state: S) -> Option<W::Result>
where
    W: Walker<S>,
    Q: Queue<S>,
{
    let mut queue = Q::default();
    queue.push(initial_state);
    while let Some(state) = queue.pop() {
        match walker.visit(&state) {
            VisitDecision::Break(result) => return Some(result),
            VisitDecision::Continue => continue,
            VisitDecision::Next(mut generator) => {
                generator.generate(|state| queue.push(state));
            }
        }
    }
    None
}

pub fn walk_deep<S, W>(walker: &mut W, initial_state: S) -> Option<W::Result>
where
    W: Walker<S>,
{
    walk::<S, W, Vec<S>>(walker, initial_state)
}

pub fn walk_broad<S, W>(walker: &mut W, initial_state: S) -> Option<W::Result>
where
    W: Walker<S>,
{
    walk::<S, W, VecDeque<S>>(walker, initial_state)
}
