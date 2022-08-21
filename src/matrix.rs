use std::ops::{Index, Range};

pub trait Matrix<'a, T> {
    fn get(&self, row: usize, col: usize) -> &T;
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;

    fn to_vec_vec(&self) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        (0..self.rows())
            .map(|row| {
                (0..self.cols())
                    .map(|col| self.get(row, col).clone())
                    .collect()
            })
            .collect()
    }

    fn flip_vertically(&'a self) -> VerticallyFlipped<'a, Self>
    where
        Self: Sized,
    {
        VerticallyFlipped::wrap(self)
    }

    fn flip_horizontally(&'a self) -> HorizontallyFlipped<'a, Self>
    where
        Self: Sized,
    {
        HorizontallyFlipped::wrap(self)
    }

    fn rotate(&'a self) -> Rotated<'a, Self>
    where
        Self: Sized,
    {
        Rotated::wrap(self)
    }

    fn slice(&'a self, rows_range: Range<usize>, cols_range: Range<usize>) -> Sliced<'a, Self>
    where
        Self: Sized,
    {
        Sliced::slice(self, rows_range, cols_range)
    }

    fn iter_by_rows(&'a self) -> MatrixIterator<'a, T>
    where
        Self: Sized,
    {
        MatrixIterator {
            matrix: self,
            row: 0,
            col: 0,
        }
    }
}

impl<'a, T> Matrix<'a, T> for Vec<Vec<T>> {
    fn get(&self, row: usize, col: usize) -> &T {
        self.index(row).get(col).unwrap()
    }

    fn rows(&self) -> usize {
        self.len()
    }

    fn cols(&self) -> usize {
        self[0].len()
    }
}

pub struct Identity<'a, M> {
    matrix: &'a M,
}

impl<'a, T, M> Matrix<'a, T> for Identity<'a, M>
where
    M: Matrix<'a, T>,
{
    fn get(&self, row: usize, col: usize) -> &T {
        self.matrix.get(row, col)
    }

    fn rows(&self) -> usize {
        self.matrix.rows()
    }

    fn cols(&self) -> usize {
        self.matrix.cols()
    }
}

pub struct VerticallyFlipped<'a, M> {
    matrix: &'a M,
}

impl<'a, T, M> Matrix<'a, T> for VerticallyFlipped<'a, M>
where
    M: Matrix<'a, T>,
{
    fn get(&self, row: usize, col: usize) -> &T {
        self.matrix.get(self.matrix.rows() - row - 1, col)
    }

    fn rows(&self) -> usize {
        self.matrix.rows()
    }

    fn cols(&self) -> usize {
        self.matrix.cols()
    }
}

impl<'a, M> VerticallyFlipped<'a, M> {
    pub fn wrap<T>(matrix: &'a M) -> Self
    where
        M: Matrix<'a, T>,
    {
        Self { matrix }
    }
}

pub struct HorizontallyFlipped<'a, M> {
    matrix: &'a M,
}

impl<'a, T, M> Matrix<'a, T> for HorizontallyFlipped<'a, M>
where
    M: Matrix<'a, T>,
{
    fn get(&self, row: usize, col: usize) -> &T {
        self.matrix.get(row, self.matrix.cols() - col - 1)
    }

    fn rows(&self) -> usize {
        self.matrix.rows()
    }

    fn cols(&self) -> usize {
        self.matrix.cols()
    }
}

impl<'a, M> HorizontallyFlipped<'a, M> {
    pub fn wrap<T>(matrix: &'a M) -> Self
    where
        M: Matrix<'a, T>,
    {
        Self { matrix }
    }
}

pub struct Rotated<'a, M> {
    matrix: &'a M,
}

impl<'a, T, M> Matrix<'a, T> for Rotated<'a, M>
where
    M: Matrix<'a, T>,
{
    fn get(&self, row: usize, col: usize) -> &T {
        self.matrix.get(col, self.matrix.rows() - row - 1)
    }

    fn rows(&self) -> usize {
        self.matrix.cols()
    }

    fn cols(&self) -> usize {
        self.matrix.rows()
    }
}

impl<'a, M> Rotated<'a, M> {
    pub fn wrap<T>(matrix: &'a M) -> Self
    where
        M: Matrix<'a, T>,
    {
        Self { matrix }
    }
}

pub struct Sliced<'a, M> {
    matrix: &'a M,
    rows_range: Range<usize>,
    cols_range: Range<usize>,
}

impl<'a, T, M> Matrix<'a, T> for Sliced<'a, M>
where
    M: Matrix<'a, T>,
{
    fn get(&self, row: usize, col: usize) -> &T {
        self.matrix
            .get(self.rows_range.start + row, self.cols_range.start + col)
    }

    fn rows(&self) -> usize {
        self.rows_range.len()
    }

    fn cols(&self) -> usize {
        self.cols_range.len()
    }
}

impl<'a, M> Sliced<'a, M> {
    pub fn slice<T>(matrix: &'a M, rows_range: Range<usize>, cols_range: Range<usize>) -> Self
    where
        M: Matrix<'a, T>,
    {
        Self {
            matrix,
            rows_range,
            cols_range,
        }
    }
}

pub struct MatrixIterator<'a, T> {
    matrix: &'a dyn Matrix<'a, T>,
    row: usize,
    col: usize,
}

impl<'a, T> Iterator for MatrixIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.matrix.rows() && self.col < self.matrix.cols() {
            let result = self.matrix.get(self.row, self.col);
            self.col += 1;
            if self.col == self.matrix.cols() {
                self.col = 0;
                self.row += 1;
            }
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_vec() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(m.get(0, 0), &1);
        assert_eq!(m.get(1, 0), &4);
        assert_eq!(m.get(2, 1), &8);
    }

    #[test]
    fn test_flipped() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let v = VerticallyFlipped { matrix: &m };
        assert_eq!(v.get(0, 0), &7);
        assert_eq!(v.get(1, 0), &4);
        assert_eq!(v.get(2, 1), &2);

        let h = HorizontallyFlipped { matrix: &m };
        assert_eq!(h.get(0, 0), &3);
        assert_eq!(h.get(1, 0), &6);
        assert_eq!(h.get(2, 1), &8);
    }

    #[test]
    fn test_rotated() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let r = Rotated { matrix: &m };
        assert_eq!(r.get(0, 0), &3);
        assert_eq!(r.get(1, 0), &2);
        assert_eq!(r.get(2, 1), &4);
    }

    #[test]
    fn test_sliced() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let s = Sliced::slice(&m, 1..3, 1..3);
        assert_eq!(s.get(0, 0), &5);
        assert_eq!(s.get(1, 0), &8);
        assert_eq!(s.get(1, 1), &9);
    }

    #[test]
    fn test_iter() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let mut iter = m.iter_by_rows();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&8));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
