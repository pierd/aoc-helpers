#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TileMap<T: Copy> {
    map: Vec<Vec<T>>,
}

impl<T: Copy> From<&Vec<Vec<T>>> for TileMap<T> {
    fn from(map: &Vec<Vec<T>>) -> Self {
        Self { map: map.clone() }
    }
}

impl<T: Copy> TileMap<T> {
    pub fn get(&self, row: isize, col: isize) -> Option<T> {
        if let (Ok(row), Ok(col)) = (usize::try_from(row), usize::try_from(col)) {
            if row < self.map.len() && col < self.map[row].len() {
                return Some(self.map[row][col]);
            }
        }
        None
    }

    pub fn get_surroundings(&self, row: isize, col: isize) -> [[Option<T>; 3]; 3] {
        [
            [
                self.get(row - 1, col - 1),
                self.get(row - 1, col),
                self.get(row - 1, col + 1),
            ],
            [
                self.get(row, col - 1),
                self.get(row, col),
                self.get(row, col + 1),
            ],
            [
                self.get(row + 1, col - 1),
                self.get(row + 1, col),
                self.get(row + 1, col + 1),
            ],
        ]
    }

    pub fn set(&mut self, row: isize, col: isize, tile: T) -> bool {
        if let (Ok(row), Ok(col)) = (usize::try_from(row), usize::try_from(col)) {
            if row < self.map.len() && col < self.map[row].len() {
                self.map[row][col] = tile;
                return true;
            }
        }
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.map.iter().flat_map(|row| row.iter()).cloned()
    }

    pub fn step<S: Stepper<T>>(&mut self, stepper: &mut S) -> Option<S::Result> {
        let mut result = stepper.step(self);
        let mut temp = self.clone();

        while result.is_none() {
            for row in 0..self.map.len() {
                for col in 0..self.map[row].len() {
                    temp.map[row][col] =
                        stepper.apply_step_rule(self.get_surroundings(row as isize, col as isize));
                }
            }
            std::mem::swap(self, &mut temp);
            result = stepper.step(self);
        }
        result
    }
}

pub trait Stepper<T: Copy> {
    type Result;

    fn apply_step_rule(&mut self, zone: [[Option<T>; 3]; 3]) -> T;

    fn step(&mut self, tile_map: &TileMap<T>) -> Option<Self::Result>;
}

pub fn iter_all<T: Copy>(zone: [[Option<T>; 3]; 3]) -> impl Iterator<Item = T> {
    zone.into_iter().flat_map(|row| row.into_iter()).flatten()
}

pub fn iter_all_neighbours<T: Copy>(zone: [[Option<T>; 3]; 3]) -> impl Iterator<Item = T> {
    zone[0]
        .into_iter()
        .chain([zone[1][0], zone[1][2]].into_iter())
        .chain(zone[2].into_iter())
        .flatten()
}

pub fn iter_direct_neighbours<T: Copy>(zone: [[Option<T>; 3]; 3]) -> impl Iterator<Item = T> {
    [zone[0][1], zone[1][0], zone[1][2], zone[2][1]]
        .into_iter()
        .flatten()
}
