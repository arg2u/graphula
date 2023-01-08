use std::fmt::{Display, Formatter, Result, Write};
use std::ops::{Index, IndexMut};
use std::vec;

#[derive(Clone)]
pub struct Row {
    data: Vec<i32>,
}

impl Row {
    pub fn new(size: usize, filler: i32) -> Self {
        Self {
            data: vec![filler; size],
        }
    }
}

impl Index<usize> for Row {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Row {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut str_row = String::new();
        self.data
            .iter()
            .for_each(|c| write!(&mut str_row, "{} ", c).unwrap());
        write!(f, "{}", str_row.trim_end())
    }
}