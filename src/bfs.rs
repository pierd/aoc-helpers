use core::hash::Hash;
use std::collections::{HashMap, VecDeque};

pub trait Driver<S> {
    type Cost: Ord;
    type TransitionsIterator: Iterator<Item = (Self::Cost, S)>;

    fn iter_transitions(&self, from_cost: &Self::Cost, from_state: &S)
        -> Self::TransitionsIterator;
    fn should_continue(&self, cost: &Self::Cost, state: &S) -> bool;
}

pub trait FlatCostDriver<S> {
    type TransitionsIterator: Iterator<Item = S>;

    fn iter_transitions(&self, from_state: &S) -> Self::TransitionsIterator;
    fn is_final(&self, state: &S) -> bool;
}

pub struct CostAddingIterator<I> {
    cost: usize,
    iter: I,
}

impl<I> Iterator for CostAddingIterator<I>
where
    I: Iterator,
{
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| (self.cost, x))
    }
}

impl<S, T> Driver<S> for T
where
    T: FlatCostDriver<S>,
{
    type Cost = usize;
    type TransitionsIterator = CostAddingIterator<<Self as FlatCostDriver<S>>::TransitionsIterator>;

    fn iter_transitions(
        &self,
        from_cost: &Self::Cost,
        from_state: &S,
    ) -> Self::TransitionsIterator {
        let new_cost = *from_cost + 1;
        CostAddingIterator {
            cost: new_cost,
            iter: FlatCostDriver::<S>::iter_transitions(self, from_state),
        }
    }

    fn should_continue(&self, _cost: &Self::Cost, state: &S) -> bool {
        !FlatCostDriver::<S>::is_final(self, state)
    }
}

pub struct BFSResult<S, C> {
    pub final_state: Option<S>,
    pub final_cost: Option<C>,
    pub seen_states: HashMap<S, C>,
}

pub fn find_lowest_cost<D, S>(
    driver: &D,
    initial_cost: D::Cost,
    initial_state: S,
    cost_limit: Option<D::Cost>,
) -> BFSResult<S, D::Cost>
where
    D: Driver<S>,
    D::Cost: Clone,
    S: Eq + Hash + Clone,
{
    let mut seen_states: HashMap<S, D::Cost> = Default::default();
    let mut queue = VecDeque::new();
    seen_states.insert(initial_state.clone(), initial_cost.clone());
    queue.push_back((initial_cost, initial_state));
    while let Some((cost, state)) = queue.pop_front() {
        for (cost, state) in driver.iter_transitions(&cost, &state) {
            if !driver.should_continue(&cost, &state) {
                return BFSResult {
                    final_state: Some(state),
                    final_cost: Some(cost),
                    seen_states,
                };
            }
            if let Some(ref limit) = cost_limit {
                if limit < &cost {
                    continue;
                }
            }
            if seen_states
                .get(&state)
                .map(|existing_cost| &cost < existing_cost)
                .unwrap_or(true)
            {
                seen_states.insert(state.clone(), cost.clone());
                queue.push_back((cost, state));
            }
        }
    }
    BFSResult { final_state: None, final_cost: None, seen_states }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_adding_iterator() {
        let mut iter = CostAddingIterator {
            cost: 42,
            iter: [1, 2, 3].into_iter(),
        };
        assert_eq!(iter.next(), Some((42, 1)));
        assert_eq!(iter.next(), Some((42, 2)));
        assert_eq!(iter.next(), Some((42, 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_find_lowest_cost() {
        struct StringFinder;
        impl FlatCostDriver<String> for StringFinder {
            type TransitionsIterator = Box<dyn Iterator<Item = String>>;

            fn iter_transitions(&self, from_state: &String) -> Self::TransitionsIterator {
                let from_state = from_state.clone();
                Box::new("ABCDEFGHIJKLMNO".chars().map(move |c| {
                    let mut new_state = from_state.clone();
                    new_state.push(c);
                    new_state
                }))
            }

            fn is_final(&self, state: &String) -> bool {
                state == "DONE"
            }
        }

        let result = find_lowest_cost(&StringFinder, 0, Default::default(), None);
        assert_eq!(result.final_cost, Some(4));
        assert_eq!(result.final_state, Some("DONE".to_owned()));
    }
}
