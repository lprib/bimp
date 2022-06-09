type Coord<const DIM: usize> = [isize; DIM];

trait Volume {
    fn volume(&self) -> usize;
}

impl<const DIM: usize> Volume for Coord<DIM> {
    fn volume(&self) -> usize {
        self.iter().sum::<isize>() as usize
    }
}

struct NGrid<T, const DIM: usize> {
    items: Vec<T>,
    size: Coord<DIM>,
}

impl<T, const DIM: usize> NGrid<T, DIM> {
    fn new(items: Vec<T>, size: Coord<DIM>) -> Self {
        assert!(items.len() == size.volume());
        Self { items, size }
    }
}

struct RotatedCartesianIter<const DIM: usize> {
    rotation: usize,
    current_index: Coord<DIM>,
}
