use std::ops::{Add, Sub};

#[derive(Debug, Clone)]
pub struct Coord<const D: usize> {
    axes: [isize; D],
}

impl<const D: usize> Coord<D> {
    const ZERO: Self = Self { axes: [0; D] };
    const ONE: Self = Self { axes: [1; D] };

    pub fn new(axes: [isize; D]) -> Self {
        Self { axes }
    }

    pub fn volume(&self) -> usize {
        self.axes.iter().sum::<isize>() as usize
    }

    pub fn iter_volume(&self, size: &Self) -> CartesianIter<D> {
        // CartesianIter expects inclusive range, so subtract one
        CartesianIter::new(self, &(self.clone() + (size.clone() - Self::ONE)))
    }
}

macro_rules! impl_coord_new {
    ($typename:ident, $dim:expr, $new_fn_name:ident, [$($ax_name:ident),+]) => {
        impl $typename<$dim> {
            pub fn $new_fn_name($($ax_name: isize),+) -> Self {
                Self::new([$($ax_name),+])
            }
        }
    }
}

impl_coord_new!(Coord, 1, new_1d, [x]);
impl_coord_new!(Coord, 2, new_2d, [x, y]);
impl_coord_new!(Coord, 3, new_3d, [x, y, z]);
impl_coord_new!(Coord, 4, new_4d, [x, y, z, w]);

impl<const D: usize> Sub for Coord<D> {
    type Output = Coord<D>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = [0; D];
        for i in 0..D {
            out[i] = self.axes[i] - rhs.axes[i];
        }
        Coord { axes: out }
    }
}

impl<const D: usize> Add for Coord<D> {
    type Output = Coord<D>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = [0; D];
        for i in 0..D {
            out[i] = self.axes[i] + rhs.axes[i];
        }
        Coord { axes: out }
    }
}

impl<const D: usize> PartialEq for Coord<D> {
    fn eq(&self, other: &Self) -> bool {
        self.axes == other.axes
    }
}

impl<const D: usize> Eq for Coord<D> {}

pub struct CartesianIter<const D: usize> {
    begin: Coord<D>,
    end_inclusive: Coord<D>,
    i: Coord<D>,
    /// Since i is always one ahead of the rest of the iterator, we need an overflow flag to know
    /// when it's gone off the end of the n-dimension extent
    overflow: bool,
}

impl<const D: usize> CartesianIter<D> {
    fn new(begin: &Coord<D>, end_inclusive: &Coord<D>) -> Self {
        Self {
            begin: begin.clone(),
            end_inclusive: end_inclusive.clone(),
            i: begin.clone(),
            overflow: false,
        }
    }
}

impl<const D: usize> Iterator for CartesianIter<D> {
    type Item = Coord<D>;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.i.clone();

        // check and early return on overflow so self.i is not changed on the next iteration
        if self.overflow {
            return None;
        }

        for digit in 0..D {
            // no overflow condition in the current digit, we can increment and return safely
            if self.i.axes[digit] < self.end_inclusive.axes[digit] {
                self.i.axes[digit] += 1;
                return Some(cur);
            } else {
                // overflow case, reset current digit back to minimum and continue to the next
                // digit (carry)
                self.i.axes[digit] = self.begin.axes[digit];
            }
        }

        // catch the very last item of the iter
        if cur == self.end_inclusive {
            self.overflow = true;
            Some(cur)
        } else {
            // unreachable since overflow is checked at top of function
            unreachable!();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eq() {
        assert_eq!(Coord::new_4d(1, 2, 3, 4), Coord::new_4d(1, 2, 3, 4));
        assert!(Coord::new_4d(1, 2, 3, 4) != Coord::new_4d(-100, 2, 3, 4));
    }

    #[test]
    fn add() {
        assert_eq!(
            Coord::new_4d(1, 2, 3, 4) + Coord::new_4d(5, 6, 7, 8),
            Coord::new_4d(6, 8, 10, 12)
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Coord::new_4d(1, 2, 3, 4) - Coord::new_4d(0, 2, 4, 8),
            Coord::new_4d(1, 0, -1, -4)
        );
    }

    #[test]
    fn iter_origin() {
        let mut i = Coord::new_3d(0, 0, 0).iter_volume(&Coord::new_3d(3, 3, 3));
        for z in 0..3 {
            for y in 0..3 {
                for x in 0..3 {
                    assert_eq!(i.next(), Some(Coord::new_3d(x, y, z)));
                }
            }
        }
        assert_eq!(i.next(), None);
    }

    #[test]
    fn iter_neg_1() {
        let mut i = Coord::new_3d(-1, -1, -1).iter_volume(&Coord::new_3d(3, 3, 3));
        for z in -1..2 {
            for y in -1..2 {
                for x in -1..2 {
                    assert_eq!(i.next(), Some(Coord::new_3d(x, y, z)));
                }
            }
        }
        assert_eq!(i.next(), None);
    }

    #[test]
    fn iter() {
        let mut i = Coord::new_4d(-1, -2, -3, -4).iter_volume(&Coord::new_4d(10, 10, 10, 10));
        for w in -4..6 {
            for z in -3..7 {
                for y in -2..8 {
                    for x in -1..9 {
                        assert_eq!(i.next(), Some(Coord::new_4d(x, y, z, w)));
                    }
                }
            }
        }
        assert_eq!(i.next(), None);
    }
}
