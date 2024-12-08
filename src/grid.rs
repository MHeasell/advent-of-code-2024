use crate::{
    direction::{DIRECTIONS, EIGHT_WAY_DIRECTIONS},
    position::{pos, Position},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    pub width: usize,
    pub vec: Vec<T>,
}

impl Grid<char> {
    pub fn from_strings(lines: &[String]) -> Self {
        if lines.is_empty() {
            return Grid {
                width: 0,
                vec: vec![],
            };
        }

        let width = lines[0].len();
        let vec = lines.iter().flat_map(|l| l.chars()).collect();
        Grid { width, vec }
    }
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, val: T) -> Self {
        Self {
            width,
            vec: vec![val; width * height],
        }
    }

    pub fn from_vecs(lines: &[Vec<T>]) -> Self {
        if lines.is_empty() {
            return Grid {
                width: 0,
                vec: vec![],
            };
        }

        let width = lines[0].len();
        let vec = lines.concat();
        Grid { width, vec }
    }

    pub fn resize_height(&mut self, rows: usize, val: T) {
        self.vec.resize(self.width * rows, val);
    }
}

impl<T> Grid<T> {
    pub fn height(&self) -> usize {
        self.vec.len() / self.width
    }

    pub fn to_vec_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height() {
            Some((y * self.width) + x)
        } else {
            None
        }
    }

    pub fn signed_to_vec_index(&self, x: i64, y: i64) -> Option<usize> {
        let x2 = usize::try_from(x).ok()?;
        let y2 = usize::try_from(y).ok()?;
        self.to_vec_index(x2, y2)
    }

    pub fn pos_to_vec_index(&self, pos: &Position) -> Option<usize> {
        self.signed_to_vec_index(pos.x, pos.y)
    }

    pub fn to_pos(&self, vec_index: usize) -> Option<Position> {
        if vec_index < self.vec.len() {
            let x = vec_index % self.width;
            let y = vec_index / self.width;
            Some(pos(i64::try_from(x).unwrap(), i64::try_from(y).unwrap()))
        } else {
            None
        }
    }

    pub fn is_in_bounds(&self, pos: &Position) -> bool {
        self.pos_to_vec_index(pos).is_some()
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.vec[self.to_vec_index(x, y).unwrap()]
    }

    pub fn get_pos(&self, pos: &Position) -> &T {
        self.get(pos.x.try_into().unwrap(), pos.y.try_into().unwrap())
    }

    pub fn try_get(&self, x: usize, y: usize) -> Option<&T> {
        self.to_vec_index(x, y).map(|i| &self.vec[i])
    }

    pub fn try_get_signed(&self, x: i64, y: i64) -> Option<&T> {
        self.signed_to_vec_index(x, y).map(|i| &self.vec[i])
    }

    pub fn try_get_pos(&self, pos: &Position) -> Option<&T> {
        self.pos_to_vec_index(pos).map(|i| &self.vec[i])
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        let index = self.to_vec_index(x, y).unwrap();
        self.vec[index] = val;
    }

    pub fn set_pos(&mut self, pos: &Position, val: T) {
        let index = self.pos_to_vec_index(pos).unwrap();
        self.vec[index] = val;
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.vec.iter()
    }

    pub fn pos_iter(&self) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height();
        (0..height).flat_map(move |y| {
            (0..width).map(move |x| Position {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            })
        })
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Position, &T)> {
        self.pos_iter().zip(self.iter())
    }

    pub fn into_enumerate(self) -> impl Iterator<Item = (Position, T)> {
        self.pos_iter().zip(self.vec)
    }

    pub fn neighbours(&self, pos: Position) -> impl Iterator<Item = (Position, &T)> {
        DIRECTIONS.iter().filter_map(move |d| {
            let p = pos.move_in_direction(*d);
            self.try_get_pos(&p).map(|v| (p, v))
        })
    }

    pub fn neighbours8(&self, pos: Position) -> impl Iterator<Item = (Position, &T)> {
        EIGHT_WAY_DIRECTIONS.iter().filter_map(move |d| {
            let p = pos.move_in_direction8(*d);
            self.try_get_pos(&p).map(|v| (p, v))
        })
    }

    pub fn position<F: Fn(&T) -> bool>(&self, pred: F) -> Option<Position> {
        self.vec
            .iter()
            .position(pred)
            .map(|x| self.to_pos(x).unwrap())
    }

    pub fn cols(&self) -> impl Iterator<Item = usize> {
        0..self.width
    }

    pub fn rows(&self) -> impl Iterator<Item = usize> {
        0..self.height()
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter()
    }
}

pub fn print_grid<T, F>(g: &Grid<T>, f: F)
where
    F: Fn(&T) -> char,
{
    for r in g.rows() {
        for c in g.cols() {
            print!("{}", f(g.get(c, r)));
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_new() {
        let g = Grid::new(4, 3, 'a');
        assert_eq!(g.width, 4);
        assert_eq!(g.height(), 3);
        assert_eq!(g.get(0, 0), &'a');
    }

    #[test]
    fn test_grid_from_vecs() {
        let g = Grid::from_vecs(&vec![
            vec!['a', 'b', 'c', 'd'],
            vec!['e', 'f', 'g', 'h'],
            vec!['i', 'j', 'k', 'l'],
        ]);
        assert_eq!(g.width, 4);
        assert_eq!(g.height(), 3);
        assert_eq!(g.get(1, 0), &'b');
        assert_eq!(g.get(1, 1), &'f');
        assert_eq!(g.get(1, 2), &'j');
    }

    #[test]
    fn test_grid_get_set() {
        let mut g = Grid::new(4, 3, 'a');
        assert_eq!(g.get(1, 2), &'a');
        assert_eq!(g.try_get(1, 2), Some(&'a'));
        assert_eq!(g.try_get_signed(1, 2), Some(&'a'));
        assert_eq!(g.get_pos(&pos(1, 2)), &'a');
        assert_eq!(g.try_get_pos(&pos(1, 2)), Some(&'a'));

        g.set(1, 2, 'b');
        assert_eq!(g.get(1, 2), &'b');
        assert_eq!(g.try_get(1, 2), Some(&'b'));
        assert_eq!(g.try_get_signed(1, 2), Some(&'b'));
        assert_eq!(g.get_pos(&pos(1, 2)), &'b');
        assert_eq!(g.try_get_pos(&pos(1, 2)), Some(&'b'));

        g.set_pos(&pos(3, 2), 'c');

        assert_eq!(g.get(1, 2), &'b');
        assert_eq!(g.try_get(1, 2), Some(&'b'));
        assert_eq!(g.try_get_signed(1, 2), Some(&'b'));
        assert_eq!(g.get_pos(&pos(1, 2)), &'b');

        assert_eq!(g.get(3, 2), &'c');
        assert_eq!(g.try_get(3, 2), Some(&'c'));
        assert_eq!(g.try_get_signed(3, 2), Some(&'c'));
        assert_eq!(g.get_pos(&pos(3, 2)), &'c');
        assert_eq!(g.get_pos(&pos(3, 2)), &'c');
    }

    #[test]
    fn test_grid_try_get() {
        let g = Grid::new(4, 3, 'a');

        assert_eq!(g.try_get(3, 0), Some(&'a'));
        assert_eq!(g.try_get(4, 0), None);

        assert_eq!(g.try_get(0, 2), Some(&'a'));
        assert_eq!(g.try_get(0, 3), None);
    }

    #[test]
    fn test_grid_try_get_signed() {
        let g = Grid::new(4, 3, 'a');
        assert_eq!(g.try_get_signed(0, 0), Some(&'a'));
        assert_eq!(g.try_get_signed(-1, 0), None);
        assert_eq!(g.try_get_signed(0, -1), None);
        assert_eq!(g.try_get_signed(4, 0), None);
        assert_eq!(g.try_get_signed(0, 3), None);
    }

    #[test]
    fn test_grid_iter() {
        let mut g = Grid::new(4, 3, "x");
        g.set(0, 0, "a1");
        g.set(1, 0, "b1");
        g.set(2, 0, "c1");
        g.set(3, 0, "d1");

        g.set(0, 1, "a2");
        g.set(1, 1, "b2");
        g.set(2, 1, "c2");
        g.set(3, 1, "d2");

        g.set(0, 2, "a3");
        g.set(1, 2, "b3");
        g.set(2, 2, "c3");
        g.set(3, 2, "d3");

        let v = g.iter().copied().collect::<Vec<_>>();
        let expected = vec![
            "a1", "b1", "c1", "d1", "a2", "b2", "c2", "d2", "a3", "b3", "c3", "d3",
        ];
        assert_eq!(v, expected);

        let v2 = g.into_iter().collect::<Vec<_>>();
        assert_eq!(v2, expected);
    }

    #[test]
    fn test_grid_pos_iter() {
        let g = Grid::new(4, 3, "x");
        let v = g.pos_iter().collect::<Vec<_>>();
        let expected = vec![
            pos(0, 0),
            pos(1, 0),
            pos(2, 0),
            pos(3, 0),
            pos(0, 1),
            pos(1, 1),
            pos(2, 1),
            pos(3, 1),
            pos(0, 2),
            pos(1, 2),
            pos(2, 2),
            pos(3, 2),
        ];
        assert_eq!(v, expected);
    }

    #[test]
    fn test_grid_enumerate() {
        let mut g = Grid::new(3, 2, "x");
        g.set(0, 0, "a1");
        g.set(1, 0, "b1");
        g.set(2, 0, "c1");

        g.set(0, 1, "a2");
        g.set(1, 1, "b2");
        g.set(2, 1, "c2");

        let v = g.enumerate().map(|(p, &v)| (p, v)).collect::<Vec<_>>();
        let expected = vec![
            (pos(0, 0), "a1"),
            (pos(1, 0), "b1"),
            (pos(2, 0), "c1"),
            (pos(0, 1), "a2"),
            (pos(1, 1), "b2"),
            (pos(2, 1), "c2"),
        ];
        assert_eq!(v, expected);

        let v2 = g.into_enumerate().collect::<Vec<_>>();
        assert_eq!(v2, expected);
    }

    #[test]
    fn test_resize_height() {
        let mut g = Grid::new(2, 1, "a");

        assert_eq!(g.height(), 1);

        g.resize_height(3, "b");

        assert_eq!(g.height(), 3);
        assert_eq!(g.get(0, 0), &"a");
        assert_eq!(g.get(0, 1), &"b");
        assert_eq!(g.get(0, 2), &"b");
    }

    #[test]
    fn test_to_pos() {
        let g = Grid::new(3, 2, 'a');
        assert_eq!(g.to_pos(2), Some(pos(2, 0)));
        assert_eq!(g.to_pos(5), Some(pos(2, 1)));
        assert_eq!(g.to_pos(6), None);
    }

    #[test]
    fn test_neighbours() {
        let g = Grid::new(3, 4, 'a');
        let v = g.neighbours(pos(1, 1)).collect::<Vec<_>>();
        assert_eq!(
            v,
            vec![
                (pos(1, 0), &'a'),
                (pos(2, 1), &'a'),
                (pos(1, 2), &'a'),
                (pos(0, 1), &'a')
            ]
        );

        let v2 = g.neighbours(pos(0, 0)).collect::<Vec<_>>();
        assert_eq!(v2, vec![(pos(1, 0), &'a'), (pos(0, 1), &'a')]);
    }

    #[test]
    fn test_neighbours8() {
        let g = Grid::new(3, 4, 'a');
        let v2 = g.neighbours8(pos(0, 0)).collect::<Vec<_>>();
        assert_eq!(
            v2,
            vec![(pos(1, 0), &'a'), (pos(1, 1), &'a'), (pos(0, 1), &'a')]
        );
    }
}
