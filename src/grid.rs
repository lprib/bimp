use std::iter;
use std::ops::Index;

use crate::coord::{Coord, CoordIter};

struct Grid<TItem, TCoord: Coord> {
    items: Vec<TItem>,
    size: TCoord,
}

/// A Gridview is any type which implemts Index[Coord]->T
trait GridView<TItem, TCoord: Coord>: Index<TCoord, Output = TItem> {
    fn size(&self) -> TCoord;

    /// Cartesian iterator over the items in the grid.
    ///
    /// Has a fucked up type signature because no GATs so I can't figure out how to do it with
    /// static dispatch.
    ///
    /// Returns a boxed iterator over (coordinate, &item). The box can only live as long as self,
    /// since it references items in self.
    fn cartesian_iter<'a>(&'a self) -> Box<dyn Iterator<Item = (TCoord, &'a TItem)> + 'a>
    where
        CoordIter<TCoord>: Iterator<Item = TCoord>;
}

impl<TItem, TCoord: Coord> GridView<TItem, TCoord> for Grid<TItem, TCoord> {
    fn size(&self) -> TCoord {
        todo!()
    }

    fn cartesian_iter<'a>(&'a self) -> Box<dyn Iterator<Item = (TCoord, &'a TItem)> + 'a>
    where
        CoordIter<TCoord>: Iterator<Item = TCoord>,
    {
        Box::new(self.size.cartesian_iter().map(|i| (i, &self[i])))
    }
}

impl<TItem, TCoord: Coord> Index<TCoord> for Grid<TItem, TCoord> {
    type Output = TItem;

    fn index(&self, index: TCoord) -> &Self::Output {
        &self.items[index.to_flat(&self.size)]
    }
}

/// Has a reference to a grid, and a rotation amount.
/// Replaces the Index trait with one that accesses the grid in a rotated fashion
struct RotatedGridView<'grid, TItem, TCoord: Coord> {
    grid_view: &'grid Grid<TItem, TCoord>,
    rotation_times: usize,
}

impl<'grid, TItem, TCoord: Coord> Index<TCoord> for RotatedGridView<'grid, TItem, TCoord> {
    type Output = TItem;

    fn index(&self, index: TCoord) -> &Self::Output {
        &self.grid_view[index.rotated(self.rotation_times, &self.grid_view.size)]
    }
}

impl<TItem, TCoord: Coord> Grid<TItem, TCoord> {
    fn new(items: Vec<TItem>, size: TCoord) -> Self {
        assert!(items.len() == size.extent());
        Self { items, size }
    }

    fn with_rotation(&self, times: usize) -> RotatedGridView<TItem, TCoord> {
        RotatedGridView {
            grid_view: self,
            rotation_times: times,
        }
    }
}

impl<TItem: Default, TCoord: Coord> Grid<TItem, TCoord> {
    fn from_default(size: TCoord) -> Self {
        Self {
            items: iter::repeat_with(|| Default::default())
                .take(size.extent())
                .collect(),
            size,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotation_1d() {
        let g: Grid<usize, usize> = Grid::new(vec![1, 2, 3], 3);
        assert_eq!(g.items, vec![1, 2, 3]);
    }
}