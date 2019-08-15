use super::{Cell, message, context, grid};
use crate::test::zombie::agents;
use agents::package;

use rand::Rng;

use log::*;

pub struct FeatureSeed<I,M,T> {
    message: MessageSeed<I,M>,
    grid_seed: GridSeed<T>,
    observer: ObserverSeed
}

impl<I,M,T> Clone for FeatureSeed<I,M,T> {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            grid_seed: self.grid_seed.clone(),
            observer: self.observer.clone()
        }
    }
}

impl<I:Clone,M,T> FeatureSeed<I,M,T> 
where
    I: Eq + std::hash::Hash
{
    pub fn new(
        message_seed: MessageSeed<I,M>, 
        grid_seed: GridSeed<T>
    ) -> Self {
        Self {
            message: message_seed,
            grid_seed,
            observer: ObserverSeed::new()
        }
    }

    pub fn new_communicator(&self, id: I) -> message::MessageInterface<I,M> {
        let addresses: _ = self.message.0.clone();
        message::MessageInterface::new(id, addresses)
    }

    pub fn new_position(&self) -> context::GridPosition<T> {
        let total_points: usize = self.grid_seed.row*self.grid_seed.col;
        if self.grid_seed.occupied_points < total_points {
            let point: _ = self.grid_seed.unchecked_new_point();
            let grid: _ = self.grid_seed.grid.clone();
            context::GridPosition::new(point,grid)
        } else { unimplemented!() }            
    }

    pub fn new_observer(&self, kind: &agents::Kind) -> Cell<package::Observer> {
        self.observer
            .observer_cell
            .borrow_mut()
            .adjust(1,kind);
        info!("{:?}", self.observer.observer_cell);
        self.observer.observer_cell.clone()
    }
}

pub struct MessageSeed<I,M>(Cell<message::AddressCollection<I,M>>);

impl<I,M> Clone for MessageSeed<I,M> {
    fn clone(&self) -> Self {
        MessageSeed(self.0.clone())
    }
}

impl<I,M> MessageSeed<I,M> {
    pub fn new(inner: Cell<message::AddressCollection<I,M>>) -> Self { MessageSeed(inner) }
}

pub struct GridSeed<T> {
    row: usize,
    col: usize,
    occupied_points: usize,
    grid: Cell<grid::Grid<T>>
}

impl<T> Clone for GridSeed<T> {
    fn clone(&self) -> Self {
        Self {
            row: self.row,
            col: self.col,
            occupied_points: self.occupied_points,
            grid: self.grid.clone()
        }
    }
}

impl<T> GridSeed<T> {
    pub fn new(grid: Cell<grid::Grid<T>>) -> Self {
        let (row,col): _ = grid.borrow().get_dimensions();
        Self {
            occupied_points: 0,
            row,
            col,
            grid,
        }
    }

    // potential infinite loop if not checked before calling
    fn unchecked_new_point(&self) -> Cell<grid::Point<T>> {
        loop {
            let rng_row: usize = rand::thread_rng().gen_range(0, self.row);
            let rng_col: usize = rand::thread_rng().gen_range(0, self.col);

            let src_point: _ = self.grid.borrow();
            let mut try_point: std::cell::RefMut<_> = src_point[rng_row][rng_col].borrow_mut();

            if try_point.state().is_empty() {    
                try_point.replace(grid::PointState::Claimed);
                return src_point[rng_row][rng_col].clone()
            }
        }
    }
}

#[derive(Clone)]
struct ObserverSeed {
    observer_cell: Cell<package::Observer>
}

impl ObserverSeed {
    fn new() -> Self {
        Self {
            observer_cell: package::Observer::new().into_cell()
        }
    }
}
