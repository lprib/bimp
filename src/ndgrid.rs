use crate::ndcoord::Coord;

struct NGrid<T, const D: usize> {
    items: Vec<T>,
    size: Coord<D>,
}

impl<T, const D: usize> NGrid<T, D> {
    fn new(items: Vec<T>, size: Coord<D>) -> Self {
        assert!(items.len() == size.volume());
        Self { items, size }
    }
}

struct RotatedCartesianIter<const D: usize> {
    rotation: usize,
    current_index: Coord<D>,
}
