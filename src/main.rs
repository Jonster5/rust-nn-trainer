use crate::{math::SelfAddition, matrix::Matrix};

mod math;
mod matrix;

fn main() {
    let m1 = matrix::Matrix::new(3, 3);
    let m2 = matrix::Matrix::from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    println!("{}", m2);
    println!("{}", m2.add(&m1));
}
