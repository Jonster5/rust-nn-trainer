#![allow(dead_code)]

use crate::{
    activation::{self, get, ActivationFunction},
    math::Matrix,
};

pub struct Layer {
    size: usize,

    activation: ActivationFunction,
    a_name: String,

    weights: Matrix,
    biases: Matrix,

    output: Matrix,

    error: Matrix,
    gradient: Matrix,

    p_gradient: Matrix,
}

impl Layer {
    pub fn new(size: usize, p_size: usize, activation: &str) -> Self {
        Self {
            size,

            activation: get(activation),
            a_name: String::from(activation),

            weights: Matrix::new(size, p_size).randomize(),
            biases: Matrix::new(size, p_size).randomize(),

            output: Matrix::new(size, 1),

            error: Matrix::new(size, 1),
            gradient: Matrix::new(size, 1),

            p_gradient: Matrix::new(size, 1),
        }
    }
}
