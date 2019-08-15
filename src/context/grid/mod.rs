use std::{rc,cell,ops,fmt};

use crate::utils;

use utils::Cell;

use utils::sync::GreedyLock;

mod connection;

pub use connection::Connection;

#[derive(Debug)]
pub struct Grid<T> {
    row: usize,
    col: usize,
    points: Vec<Cell<Point<T>>>
}

impl<T> Grid<T> 
{
    pub fn new(row: usize, col: usize) -> Self 
    {
        let mut points: Vec<Cell<Point<T>>> = Vec::new();

        for row_idx in 0..row {
            for col_idx in 0..col
            {
                let index: _ = PointIndex::new(row_idx,col_idx);

                let point: Cell<Point<T>> = {
                    let inner: _ = Point::new(index);
                    rc::Rc::new(cell::RefCell::new(inner)) 
                };

                points.push(point)
            }
        }

        let grid: Self = Grid {row,col,points};

        for locked_point in grid.points
            .iter()
        {
            let mut point: cell::RefMut<Point<T>> = locked_point.await_greedy_lock();

            point.connections = Some(connection::Connection::new(&point.index,&grid))
        }

        grid
    }

    pub fn get_dimensions(&self) -> (usize,usize) {
        (self.row, self.col)
    }

    pub fn into_cell(self) -> Cell<Self> {
        rc::Rc::new(cell::RefCell::new(self))
    }

    pub fn toroidal_distance_between(&self, lhs: &Point<T>, rhs: &Point<T>) -> usize {
        let delta: _ = |a1,a2,bound| -> f32 {
            let abs_no_bound: _ = f32::abs(a2-a1);
            let abs_bound: _ = f32::abs(a2 + bound - a1);
            f32::min(abs_bound, abs_no_bound)
        };

        let width: f32 = self.row as f32;
        let x1: f32 = lhs.index.row as f32;
        let x2: f32 = rhs.index.row as f32;

        let dx: f32 = delta(x1,x2,width);

        let height: f32 = self.col as f32;
        let y1: f32 = lhs.index.col as f32;
        let y2: f32 = rhs.index.col as f32;

        let dy: f32 = delta(y1,y2,height);

        (dx*dx + dy*dy).sqrt() as usize
    }
}

impl<T> ops::Index<usize> for Grid<T> 
{
    type Output = [Cell<Point<T>>];
    
    fn index(&self, idx: usize) -> &Self::Output 
    {
        let range_start: usize = idx * self.col;
        let range_end: usize = range_start + self.col;
        let range: std::ops::Range<usize> = range_start..range_end;

        &self.points[range]
    }
}

pub struct Point<T> {
    index: PointIndex,
    state: PointState<T>,
    connections: Option<connection::Connection<T>> 
}

impl<T> fmt::Debug for Point<T> 
where
    T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} Point {:?}", self.state, self.index)
    }
}

impl<T:PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.index == other.index && self.state == other.state) 
    }
}

impl<T> Point<T> 
{
    fn new(index: PointIndex) -> Self {
        Self {
            index,
            state: PointState::Empty,
            connections: None
        }
    }

    pub fn get_connections(&self) -> Result<[Cell<Point<T>>; 4], std::option::NoneError> {
        Ok(self.connections
            .as_ref()?
            .get_connections())
    }   

    pub fn state(&self) -> &PointState<T> {
        &self.state
    }

    pub fn replace(&mut self, new_value: PointState<T>) -> PointState<T> {
        std::mem::replace(&mut self.state, new_value)
    }

    pub fn move_inner_into(&mut self, new_position: &mut Self) {
        new_position.replace(self.replace(PointState::Empty));
    }

    pub fn get_idx(&self) -> PointIndex {
        self.index.clone()
    }
}

#[derive(PartialEq)]
pub enum PointState<T> {
    Occupied(T),
    Claimed,
    Empty
}

impl<T> fmt::Debug for PointState<T> 
where
    T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PointState::Occupied(_) => write!(f, "Occupied"),
            PointState::Claimed => write!(f, "Claimed"),
            PointState::Empty => write!(f, "Empty")  
        }
    }
}

impl<T> PointState<T> {
    pub fn is_empty(&self) -> bool {
        match self {
            PointState::Empty => true,
            _ => false
        }
    }

    pub fn as_inner(&self) -> Option<&T> {
        match self {
            PointState::Occupied(inner) => Some(inner),
            _ => None
        }
    }
}

#[derive(PartialEq,Clone)]
pub struct PointIndex {
    row: usize,
    col: usize,
}

impl PointIndex {
    fn new(row: usize, col: usize) -> Self {
        Self {row,col}
    }

    pub fn as_dimensions(&self) -> (usize,usize) {
        (self.row, self.col)
    }
}

impl fmt::Debug for PointIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}


#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_grid_index() 
    {
        let grid: Grid<()> = Grid::new(5,5);

        let source_point: _ = grid[2][2].borrow();
        let expected_point: Point<()> = Point::new(PointIndex::new(2,2));

        assert_eq!(source_point.index, expected_point.index)
    }

    #[test]
    fn test_get_connections()
    {
        let grid: Grid<()> = Grid::new(5,5);
        let source_point: _ = grid[2][2].borrow();

        let test_connections: _ = source_point
            .get_connections()
            .unwrap();

        let expected_connections: _ = [
            grid[1][2].clone(), 
            grid[2][3].clone(), 
            grid[3][2].clone(), 
            grid[2][1].clone()
        ];

        assert_eq!(test_connections, expected_connections);
    }

    #[ignore] #[test]
    fn test_pointstate() {
        unimplemented!()
    }

    #[test]
    fn test_move_into() 
    {
        let grid: Grid<()> = Grid::new(5,5);

        grid[2][2].borrow_mut().replace(PointState::Occupied(()));
        
        match grid[2][2].borrow().state() {
            PointState::Occupied(_) => { },
            _ => panic!("Point was not occupied!")
        }

        grid[2][2].borrow_mut().move_inner_into(&mut grid[3][3].borrow_mut());

        println!("{:?}", grid[3][3]);
        println!("{:?}", grid[2][2]);
    }

    #[test]
    fn test_distance()
    {
        let grid: Grid<()> = Grid::new(5,5);

        let test: _ = grid.toroidal_distance_between(&grid[4][4].borrow(), &grid[0][0].borrow());

        assert_eq!(test, 1);
    }
}   
