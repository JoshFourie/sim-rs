pub mod grid;

use crate::utils;
use std::fmt;

#[derive(Clone)]
pub struct GridPosition<T> {
    tile: utils::Cell<grid::Point<T>>,
    grid: utils::Cell<grid::Grid<T>>
}

impl<T> GridPosition<T> {
    pub fn new(tile: utils::Cell<grid::Point<T>>, grid: utils::Cell<grid::Grid<T>>) -> Self {
        Self {tile, grid}
    }

    pub fn into_cell(self) -> utils::Cell<Self> {
        std::rc::Rc::new(std::cell::RefCell::new(self))
    }

    pub fn get_tile(&self) -> utils::Cell<grid::Point<T>> {
        self.tile.clone()
    }

    pub fn get_grid(&self) -> utils::Cell<grid::Grid<T>> {
        self.grid.clone()
    }

    pub fn borrow(&self) -> std::cell::Ref<grid::Point<T>> {
        self.tile.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<grid::Point<T>> {
        self.tile.borrow_mut()
    }

    pub fn get_dimensions(&self) -> (usize,usize) {
        self.grid.borrow().get_dimensions()
    }
}

impl<T:fmt::Debug> fmt::Debug for GridPosition<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Position: {:?}",
            self.tile
        )
    }
}
