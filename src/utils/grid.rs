//! Fast 2 dimensional Grid backed by a single `vec`. This module is designed to work with [`Point`].
//!
//! The traits [`Index`] and [`IndexMut`] are implemented for [`Point`] to allow usage like:
//!
//! ```
//!   # use aoc::utils::grid::Grid;
//!   # use aoc::utils::point::Point;
//!
//!   let mut grid = Grid::parse("1");
//!   let point = Point::new(0, 0);
//!
//!   let foo = grid[point];
//!   assert_eq!(foo, b'1');
//!
//!   grid[point] = foo + 1;
//!   assert_eq!(grid[point], b'2');
//! ```
//!
//! A convenience [`parse`] method creates a `Grid` directly from a 2 dimenionsal set of
//! ASCII characters, a common occurence in Advent of Code inputs. The [`same_size_with`] function
//! creates a grid of the same size, that can be used for in BFS algorithms for tracking visited
//! location or for tracking cost in Djikstra.
//!
//! [`Point`]: crate::util::point
//! [`parse`]: Grid::parse
//! [`same_size_with`]: Grid::same_size_with
use crate::utils::point::*;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    index: Point<i32>,
}

impl Grid<u8> {
    #[inline]
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = raw[0].len() as i32;
        let height = raw.len() as i32;
        let mut bytes = Vec::with_capacity((width * height) as usize);
        raw.iter().for_each(|slice| bytes.extend_from_slice(slice));
        Grid {
            width,
            height,
            bytes,
        }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    #[inline]
    pub fn find(&self, needle: T) -> Option<Point<i32>> {
        let to_point = |index| {
            let x = (index as i32) % self.width;
            let y = (index as i32) / self.width;
            Point::new(x, y)
        };
        self.bytes.iter().position(|&h| h == needle).map(to_point)
    }
}

impl<T: Copy> Grid<T> {
    pub fn new(width: i32, height: i32, value: T) -> Grid<T> {
        Grid {
            width,
            height,
            bytes: vec![value; (width * height) as usize],
        }
    }
}

impl<T> Grid<T> {
    #[inline]
    pub fn same_size_with<U: Copy>(&self, value: U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            bytes: vec![value; (self.width * self.height) as usize],
        }
    }

    #[inline]
    pub fn contains(&self, point: Point<i32>) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }

    #[inline]
    pub fn iter(&self) -> GridIter<T> {
        GridIter {
            grid: self,
            index: Point::new(0, 0),
        }
    }
}

impl<T> Index<Point<i32>> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point<i32>) -> &Self::Output {
        &self.bytes[(self.width * index.y + index.x) as usize]
    }
}

impl<T> IndexMut<Point<i32>> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point<i32>) -> &mut Self::Output {
        &mut self.bytes[(self.width * index.y + index.x) as usize]
    }
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (Point<i32>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.grid.contains(self.index) {
            return None;
        }
        let ret = Some((self.index, &self.grid[self.index]));

        if self.index.x >= self.grid.width - 1 {
            self.index.x = 0;
            self.index.y += 1;
        } else {
            self.index.x += 1;
        }
        ret
    }
}
