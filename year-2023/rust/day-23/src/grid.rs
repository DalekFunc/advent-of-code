use std::ops::{Index, IndexMut, RangeInclusive};

use crate::coordinates::Coord;

pub struct Grid<T, const N: usize> {
    pub cells: [[T; N]; N],
}

impl<T, const N: usize> std::fmt::Display for Grid<T, N>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for line in &self.cells {
            for elem in line {
                write!(f, "{}", elem)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T, const N: usize> Grid<T, N>
where
    T: Default,
{
    pub fn new() -> Self {
        Self {
            cells: core::array::from_fn(|_| core::array::from_fn(|_| Default::default())),
        }
    }
}

impl<T, const N: usize> Grid<T, N>
where
    T: Copy,
{
    pub fn fill(value: T) -> Self {
        Self {
            cells: [[value; N]; N],
        }
    }
}

impl<T, const N: usize> Grid<T, N> {
    pub fn row_bound(&self) -> RangeInclusive<usize> {
        0..=N - 1
    }

    pub fn col_bound(&self) -> RangeInclusive<usize> {
        0..=N - 1
    }
}

impl<T, const N: usize> Index<Coord> for Grid<T, N> {
    type Output = T;

    fn index(&self, co: Coord) -> &Self::Output {
        &self.cells[co.row][co.col]
    }
}

impl<T, const N: usize> IndexMut<Coord> for Grid<T, N> {
    fn index_mut(&mut self, co: Coord) -> &mut Self::Output {
        &mut self.cells[co.row][co.col]
    }
}

impl<T, const N: usize> Index<usize> for Grid<T, N> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        &self.cells[row]
    }
}

impl<T, const N: usize> IndexMut<usize> for Grid<T, N> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.cells[row]
    }
}
