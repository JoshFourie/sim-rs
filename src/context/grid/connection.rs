use super::{PointIndex, Point, Cell, Grid};

use std::fmt;

pub struct Connection<T> 
{
    pub(super) north: Cell<Point<T>>,
    pub(super) east: Cell<Point<T>>,
    pub(super) south: Cell<Point<T>>,
    pub(super) west: Cell<Point<T>>
}

impl<T:fmt::Debug> fmt::Debug for Connection<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Connection: north: {:?} east: {:?}, south: {:?}, west: {:?}",
            self.north.borrow().index,
            self.east.borrow().index,
            self.south.borrow().index,
            self.west.borrow().index,
        )
    }
}   

impl<T> Connection<T> 
{
    pub(super) fn new<'a>(index: &'a PointIndex, grid: &'a Grid<T>) -> Self 
    {
        let row: usize = index.row;
        let col: usize = index.col; 

        let max_row: usize = grid.row-1;
        let max_col: usize = grid.col-1;

        let south: _ = if row < max_row {
            grid[row + 1][col].clone()
        } else {
            grid[0][col].clone()
        };

        let north: _ = if row > 0 {
            grid[row - 1][col].clone()
        } else {
            grid[max_row][col].clone()
        };

        let west: _ = if col > 0 {
            grid[row][col - 1].clone()
        } else {
            grid[row][max_col].clone()
        };

        let east: _ = if col < max_col  {
            grid[row][col + 1].clone()
        } else {
            grid[row][0].clone()
        };

        Connection{ 
            north,
            east,
            south,
            west
        }
    }

    pub(super) fn get_connections(&self) -> [Cell<Point<T>>; 4] {
        [self.north.clone(), self.east.clone(), self.south.clone(), self.west.clone()]
    }   
}

#[cfg(test)]
mod tests 
{
    use super::*;

    macro_rules! test_connection {
        ($dir:ident,$src_row:literal,$src_col:literal,$exp_row:literal,$exp_col:literal) => {

            let grid: Grid<()> = Grid::new(5,5);
            let source_point: _ = &grid[$src_row][$src_col].clone();
            let connections: _ = &source_point
                .borrow()
                .connections;
            let test_point: _ = &connections 
                .as_ref()
                .expect("expected connections for the point");

            let test: _ = &test_point.$dir
                .borrow()
                .index; 
            assert_eq!(&PointIndex::new($exp_row,$exp_col), test)
        }
    }

    #[test]
    fn test_east_connection() {
        test_connection!(east, 2,2, 2,3);
    }

    #[test]
    fn test_east_edge_connection() {
        test_connection!(east, 1,4, 1,0);
    }

    #[test]
    fn test_west_connection() {
        test_connection!(west, 2,2, 2,1);
    }
    
    #[test]
    fn test_west_edge_connection() {
        test_connection!(west, 2,0, 2,4);
    }

    #[test]
    fn test_north_connection() {
        test_connection!(north, 2,2, 1,2);
    }
       
    #[test]
    fn test_north_edge_connection() {
        test_connection!(north, 0,3, 4,3);
    }

    #[test]
    fn test_south_connection() {
        test_connection!(south, 2,2, 3,2);
    }

    #[test]
    fn test_south_edge_connection() {
        test_connection!(south, 4,1, 0,1);
    }
}