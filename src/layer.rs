#![allow(dead_code)]

use crate::{
    activation::{self, get, ActivationFunction},
    math::Matrix,
};

pub struct Layer<'a> {
    pub size: usize,

    pub activation: ActivationFunction,
    pub a_name: String,

    pub weights: Matrix,
    pub biases: Matrix,

    pub output: &'a Matrix,

    pub error: &'a Matrix,
    pub gradient: &'a Matrix,

    pub p_gradient: &'a Matrix,
}

impl Layer<'_> {
    pub fn new(size: usize, p_size: usize, activation: &str) -> Self {
        Self {
            size,

            activation: get(activation),
            a_name: String::from(activation),

            weights: Matrix::new(size, p_size).randomize(),
            biases: Matrix::new(size, p_size).randomize(),

            output: &Matrix::new(size, 1),

            error: &Matrix::new(size, 1),
            gradient: &Matrix::new(size, 1),

            p_gradient: &Matrix::new(size, 1),
        }
    }
}
