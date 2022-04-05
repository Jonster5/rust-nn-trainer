#![allow(dead_code)]

use crate::{layer::Layer, math::Matrix};

pub struct NeuralNetwork {
    pub layers: Vec<Layer>,

    pub l_rate: f64,
    pub momentum: f64,
}

impl NeuralNetwork {
    pub fn new(model: Vec<(usize, &str)>, l_rate: f64, momentum: f64) -> Self {
        let mut layers = Vec::new();

        for i in 0..model.len() {
            if i == 0 {
                layers.push(Layer::new(model[i].0, 0, model[i].1));
            } else {
                layers.push(Layer::new(model[i].0, model[i - 1].0, model[i].1));
            }
        }
        
        Self {
            layers,
            l_rate,
            momentum
        }
    }

    fn predict(mut self, input_arr: Vec<f64>) -> Vec<f64> {
        self.layers[0].output = &Matrix::from_vec(input_arr);

        for i in 1..self.layers.len() {
            self.layers[i].output = &Matrix::multiplication(
                &self.layers[i].weights, &self.layers[i - 1].output)
                .add(&self.layers[i].biases)
                .map(|x, r,c|
                    self.layers[i].activation(x, false)
                );

        }

        self.layers[self.layers.len() - 1].output.to_vec()
    }


}
