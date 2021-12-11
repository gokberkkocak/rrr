mod compression;

use num::{FromPrimitive, Zero};
use std::{
    fs,
    ops::{Add, Div},
};

pub enum Mode {
    SRTime,
    SolverTime,
    NbSolutions,
}

pub trait Mean {
    type Item;
    fn mean(&self) -> Self::Item;
}

impl<T> Mean for Vec<T>
where
    T: Copy + Zero + Add<T, Output = T> + Div<T, Output = T> + FromPrimitive,
{
    type Item = T;
    fn mean(&self) -> T {
        let sum = self.iter().fold(T::zero(), |sum, &val| sum + val);
        sum / FromPrimitive::from_usize(self.len()).unwrap()
    }
}
pub trait FSort {
    /// Sort the iterable types which are only part ord
    fn f_sort(&mut self);
}

impl<T> FSort for Vec<T>
where
    T: PartialOrd,
{
    fn f_sort(&mut self) {
        self.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
}

pub const JSON_SUFFIX: &str = ".json";

pub const ZST_SUFFIX: &str = ".zst";

pub fn write_to_file(filepath: &str, content: String, compress: bool) {
    match compress {
        true => compression::compress_string_to_file(content, filepath),
        false => fs::write(filepath, content).expect("Unable to write to file"),
    }
}

pub fn read_file(filepath: &str, decompress: bool) -> String {
    match decompress {
        true => compression::decompress_file_to_string(filepath),
        false => String::from_utf8(fs::read(filepath).expect("Unable to read from file"))
            .expect("Unable to read file as utf8"),
    }
}
