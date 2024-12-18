#![allow(unused)]

use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

use crate::vector::{dot_product, Vector};
use anyhow::Result;
use rand::seq::index;

const NUM_THREADS: usize = 4;

pub struct Matrix<T> {
    pub data: Vec<T>,
    pub row: usize,
    pub col: usize,
}
#[derive(Debug)]
pub struct MsgInput<T> {
    index: usize,
    row: Vector<T>,
    col: Vector<T>,
}

impl<T> MsgInput<T> {
    fn new(index: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { index, row, col }
    }
}
#[derive(Debug)]
pub struct MsgOutput<T> {
    index: usize,
    value: T,
}
impl<T> MsgOutput<T> {
    fn new(index: usize, value: T) -> Self {
        Self { index, value }
    }
}
#[derive(Debug)]
pub struct Msg<T> {
    pub input: MsgInput<T>,
    pub sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

// [[1,2],[1,2],[1,2]]
impl<T> Mul for Matrix<T>
where
    T: Mul<Output = T> + Add<Output = T> + AddAssign + Copy + Default + Send + 'static + Debug,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("cannot multiply")
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Mul<Output = T> + Add<Output = T> + AddAssign + Copy + Default + Send + 'static + Debug,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!(
            "Matrix A's column must be equal to Matrix B's row"
        ));
    }
    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col).unwrap();
                    let output = MsgOutput {
                        index: msg.input.index,
                        value,
                    };
                    if let Err(e) = msg.sender.send(output) {
                        println!("Error: {:?}", e)
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    // let mut data: Vec<T> = Vec::with_capacity(a.row * b.col);
    let mut data = vec![T::default(); a.row * b.col];
    let mut receivers = Vec::with_capacity(a.row * b.col);
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..b.col * b.row]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);

            let index = i * b.col + j;
            let input = MsgInput::new(index, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[index % NUM_THREADS].send(msg) {
                println!("Error: {:?}", e);
            }
            receivers.push(rx);
        }
    }
    for rx in receivers {
        let output = rx.recv()?;
        data[output.index] = output.value;
    }
    Ok(Matrix::new(data, a.row, b.col))
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        let data = data.into();
        Self { data, row, col }
    }
}

impl<T: Debug> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.row {
            write!(f, "[")?;
            for j in 0..self.col {
                write!(f, "{:?} ", self.data[i * self.col + j])?;
            }
            if i == self.row - 1 {
                write!(f, "]")?;
            } else {
                writeln!(f, "],")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "row: {}, col: {}, {:?}", self.row, self.col, self.data)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(&a, &b).unwrap();
        assert!(c.col == 2 && c.row == 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        Ok(())
    }
}
