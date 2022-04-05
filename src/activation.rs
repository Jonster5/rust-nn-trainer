#![allow(dead_code)]

pub type ActivationFunction = fn(x: f64, der: bool) -> f64;

pub fn get(name: &str) -> ActivationFunction {
    match name {
        "sigmoid" => sigmoid,
        "tanh" => tanh,
        "relu" => relu,
        "leakyRelu" => leaky_relu,
        "elu" => elu,
        "binaryStep" => binary_step,
        "logistic" => logistic,
        "arctan" => arctan,
        _ => panic!("Activation function name not recognized"),
    }
}

pub fn sigmoid(x: f64, der: bool) -> f64 {
    if der {
        (1.0 / (1.0 + (-x).exp())) * (1.0 - 1.0 / (1.0 + (-x).exp()))
    } else {
        1.0 / (1.0 + (-x).exp())
    }
}

pub fn tanh(x: f64, der: bool) -> f64 {
    if der {
        1.0 - x.tanh() * x.tanh()
    } else {
        x.tanh()
    }
}

pub fn relu(x: f64, der: bool) -> f64 {
    if der {
        return if x > 0.0 { 1.0 } else { 0.0 };
    } else {
        return if x > 0.0 { x } else { 0.0 };
    }
}

pub fn leaky_relu(x: f64, der: bool) -> f64 {
    if der {
        return if x > 0.0 { 1.0 } else { 0.01 };
    } else {
        return if x > 0.0 { x } else { 0.01 * x };
    }
}

pub fn elu(x: f64, der: bool) -> f64 {
    if der {
        return if x > 0.0 { 1.0 } else { x.exp2() };
    } else {
        return if x > 0.0 { x } else { x.exp() - 1.0 };
    }
}

pub fn binary_step(x: f64, der: bool) -> f64 {
    if der {
        return if x > 0.0 { 1.0 } else { 0.0 };
    } else {
        return if x > 0.0 { 1.0 } else { 0.0 };
    }
}

pub fn logistic(x: f64, der: bool) -> f64 {
    if der {
        (1.0 / (1.0 + (-x).exp())) * (1.0 - (1.0 / (1.0 + (-x).exp())))
    } else {
        1.0 / (1.0 + (-x).exp())
    }
}

pub fn arctan(x: f64, der: bool) -> f64 {
    if der {
        1.0 / (1.0 + x * x)
    } else {
        x.atan()
    }
}
