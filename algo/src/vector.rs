use anyhow::Result;
use std::ops::{Add, AddAssign, Index, Mul};

#[derive(Debug)]
pub struct Vector<T> {
    pub data: Vec<T>,
}

/// 下面这些可以实现deref trait替换
impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        let data = data.into();
        Self { data }
    }
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.data.len()
    }
    #[allow(unused)]
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("The length of two vectors must be equal"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}
