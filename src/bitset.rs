pub trait BitSet: Clone + Copy + PartialEq + Eq {
    const CAPACITY: usize;

    fn insert(&mut self, idx: usize);

    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;

    fn contains(&self, idx: &usize) -> bool;
    fn intersect(&self, rhs: &Self) -> Self;
    fn difference(&self, rhs: &Self) -> Self;
}

impl BitSet for u64 {
    const CAPACITY: usize = Self::BITS as usize;

    fn insert(&mut self, idx: usize) {
        assert!(idx < Self::CAPACITY);
        *self |= 1 << idx;
    }

    fn is_empty(&self) -> bool {
        *self == 0
    }

    fn len(&self) -> usize {
        self.count_ones() as usize
    }

    fn contains(&self, idx: &usize) -> bool {
        assert!(*idx < Self::CAPACITY);
        *self & (1 << idx) != 0
    }

    fn intersect(&self, rhs: &Self) -> Self {
        *self & *rhs
    }

    fn difference(&self, rhs: &Self) -> Self {
        *self & (!*rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(0u64.is_empty(), true);
        assert_eq!(0u64.len(), 0);
        assert_eq!(0u64.contains(&0), false);
        assert_eq!(0u64.contains(&2), false);
        assert_eq!(0u64.contains(&42), false);
    }
}
