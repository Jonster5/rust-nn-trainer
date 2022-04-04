pub(crate) trait SelfAddition {
    fn add(self, other: &Self) -> Self;
    fn add_value(self, value: f64) -> Self;
}

pub(crate) trait Addition {
    fn Add(a: &Self, b: &Self) -> Self;
}

pub(crate) trait SelfSubtraction {
    fn subtract(self, other: &Self) -> Self;
    fn subtract_value(self, value: f64) -> Self;
}

pub(crate) trait Subtraction {
    fn Subtract(a: &Self, b: &Self) -> Self;
}

pub(crate) trait SelfMultiplication {
    fn hadamard(self, other: &Self) -> Self;
    fn scale(self, value: f64) -> Self;
}

pub(crate) trait Multiplication {
    fn Multiply(a: &Self, b: &Self) -> Self;
}
