/// Methods take owned self since this requires T: Copy
pub trait Coord: Sized + Copy {
    const ZERO: Self;

    /// If coord is N-dim width, height, depth, calculate volume
    fn extent(self) -> usize;
    /// Convert N-dim coord to flat array index
    fn to_flat(self, size: Self) -> usize;

    const NUM_ROTATIONS: usize;

    /// the coords rotated n times
    fn rotated(self, times: usize, grid_size: Self) -> Self;

    fn canonical_rotation_times(times: usize) -> usize {
        times % Self::NUM_ROTATIONS
    }

    fn cartesian_iter(self) -> CoordIter<Self> {
        CoordIter {
            index: Self::ZERO,
            target: self,
        }
    }
}

pub struct CoordIter<C: Coord> {
    index: C,
    target: C,
}

/// 1D
impl Coord for usize {
    const ZERO: Self = 0;

    fn extent(self) -> usize {
        self
    }
    fn to_flat(self, _: Self) -> Self {
        self
    }

    const NUM_ROTATIONS: usize = 2;

    fn rotated(self, times: usize, grid_size: Self) -> Self {
        let times = Self::canonical_rotation_times(times);
        match times {
            0 => self,
            1 => grid_size - 1 - self,
            _ => unreachable!(),
        }
    }
}

impl Iterator for CoordIter<usize> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.index;
        self.index += 1;
        if cur < self.target {
            Some(cur)
        } else {
            None
        }
    }
}

/// 2D: (x, y)
impl Coord for (usize, usize) {
    const ZERO: Self = (0, 0);

    fn extent(self) -> usize {
        self.0 * self.1
    }

    fn to_flat(self, size: Self) -> usize {
        size.1 * self.0 + self.1
    }

    const NUM_ROTATIONS: usize = 4;

    fn rotated(self, times: usize, grid_size: Self) -> Self {
        let times = Self::canonical_rotation_times(times);
        match times {
            0 => self,
            1 => (grid_size.1 - 1 - self.1, self.0),
            2 => (grid_size.0 - 1 - self.0, grid_size.1 - 1 - self.1),
            3 => (self.1, grid_size.0 - 1 - self.0),
            _ => unreachable!(),
        }
    }
}

impl<'target> Iterator for CoordIter<(usize, usize)> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.index;

        // next col
        self.index.0 += 1;

        // next row
        if self.index.0 >= self.target.0 {
            self.index.1 += 1;
            self.index.0 = 0;
        }

        if cur.0 < self.target.0 && cur.1 < self.target.1 {
            Some(cur)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cartesian_iter_1d() {
        let mut i = 3.cartesian_iter();
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(2));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn cartesian_iter_2d() {
        let mut i = (2, 2).cartesian_iter();
        assert_eq!(i.next(), Some((0, 0)));
        assert_eq!(i.next(), Some((1, 0)));
        assert_eq!(i.next(), Some((0, 1)));
        assert_eq!(i.next(), Some((1, 1)));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn cartesian_is_for_loop() {
        let mut i = (10, 10).cartesian_iter();
        for y in 0..10 {
            for x in 0..10 {
                assert_eq!(i.next(), Some((x, y)));
            }
        }
    }

    #[test]
    fn rotate_1d() {
        let x = 1;
        let grid = 5;

        assert_eq!(x.rotated(0, &grid), 1);
        assert_eq!(x.rotated(1, &grid), 3);

        // wrap around rotations
        assert_eq!(x.rotated(0, &grid), x.rotated(2, &grid));
    }

    #[test]
    fn rotate_2d() {
        let x = (0, 0);
        let grid = (3, 3);

        assert_eq!(x.rotated(0, &grid), (0, 0));
        assert_eq!(x.rotated(1, &grid), (2, 0));
        assert_eq!(x.rotated(2, &grid), (2, 2));
        assert_eq!(x.rotated(3, &grid), (0, 2));

        // wrap around rotations
        assert_eq!(x.rotated(0, &grid), x.rotated(4, &grid));

        let x = (1, 0);

        assert_eq!(x.rotated(0, &grid), (1, 0));
        assert_eq!(x.rotated(1, &grid), (2, 1));
        assert_eq!(x.rotated(2, &grid), (1, 2));
        assert_eq!(x.rotated(3, &grid), (0, 1));
    }
}
