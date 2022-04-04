use crate::math;
use std::{fmt, fs::File, io::Read};

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![vec![0.0; cols]; rows];

        Self { rows, cols, data }
    }

    pub fn from_vec(vec: Vec<f64>) -> Self {
        let rows = vec.len();
        let cols = 1;

        let data = vec![vec![0.0; cols]; rows];

        Self { rows, cols, data }
    }

    pub fn clone(&self) -> Self {
        let rows = self.rows;
        let cols = self.cols;

        let mut data = vec![vec![0.0; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                data[i][j] = self.data[i][j];
            }
        }

        Self { rows, cols, data }
    }

    pub fn import(path: &str) -> Self {
        let mut file = File::open(path).expect("File not found");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        let mut rows = 0;
        let mut cols = 0;
        let mut data = Vec::new();

        contents.lines().for_each(|line| {
            let mut row = Vec::new();

            line.split_whitespace().for_each(|num| {
                row.push(num.parse::<f64>().expect("Failed to parse number"));
            });

            cols = row.len();

            data.push(row);
            rows += 1;
        });

        Self { rows, cols, data }
    }

    pub fn export(&self) -> String {
        let mut output = String::new();

        output += "{";

        output += format!("\"rows\": {0},\"cols\": {1},", self.rows, self.cols).as_str();

        output += "\"data\": [";

        for i in 0..self.rows {
            output += "[";

            for j in 0..self.cols {
                output += format!("{0},", self.data[i][j]).as_str();
            }

            output.pop();
            output += "],";
        }

        output.pop();
        output += "]}";

        output
    }

    pub fn get(self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    pub fn set(mut self, row: usize, col: usize, value: f64) {
        self.data[row][col] = value;
    }

    pub fn to_array(self) -> Vec<f64> {
        let mut vec = Vec::new();

        for i in 0..self.rows {
            for j in 0..self.cols {
                vec.push(self.data[i][j]);
            }
        }

        vec
    }

    pub fn randomize(mut self) -> Self {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] = rand::random::<f64>();
            }
        }

        self
    }

    pub fn map(mut self, func: fn(x: f64, r: usize, c: usize) -> f64) -> Self {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] = func(self.data[i][j], i, j);
            }
        }

        self
    }

    pub fn Map(m: &Self, func: fn(x: f64, r: usize, c: usize) -> f64) -> Self {
        let mut matrix = m.clone();

        for i in 0..matrix.rows {
            for j in 0..matrix.cols {
                matrix.data[i][j] = func(matrix.data[i][j], i, j);
            }
        }

        matrix
    }

    pub fn transpose(mut self) -> Self {
        let mut d: Vec<Vec<f64>> = vec![vec![0.0; self.rows]; self.cols];

        for i in 0..self.rows {
            for j in 0..self.cols {
                d[j][i] = self.data[i][j];
            }
        }

        self.data = d;

        let tmp = self.rows;
        self.rows = self.cols;
        self.cols = tmp;

        self
    }

    pub fn Transpose(m: &Self) -> Self {
        let mut matrix = Matrix::new(m.cols, m.rows);

        for i in 0..m.rows {
            for j in 0..m.cols {
                matrix.data[j][i] = m.data[i][j];
            }
        }

        matrix
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        for i in 0..self.rows {
            for j in 0..self.cols {
                output += format!("{0:.2} ", self.data[i][j]).as_str();
            }

            output += "\n";
        }

        write!(f, "{}", output)
    }
}

impl math::SelfAddition for Matrix {
    fn add(mut self, other: &Self) -> Self {
        assert_eq!(self.rows, other.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] += other.data[i][j];
            }
        }

        self
    }

    fn add_value(mut self, value: f64) -> Self {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] += value;
            }
        }

        self
    }
}

impl math::Addition for Matrix {
    fn Add(a: &Self, b: &Self) -> Self {
        // panic if rows and colums don't match
        assert_eq!(a.rows, b.rows);

        let mut matrix = Matrix::new(a.rows, a.cols);

        for i in 0..a.rows {
            for j in 0..a.cols {
                matrix.data[i][j] = a.data[i][j] + b.data[i][j];
            }
        }

        matrix
    }
}

impl math::SelfSubtraction for Matrix {
    fn subtract(mut self, other: &Self) -> Self {
        assert_eq!(self.rows, other.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] -= other.data[i][j];
            }
        }

        self
    }

    fn subtract_value(mut self, value: f64) -> Self {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] -= value;
            }
        }

        self
    }
}

impl math::Subtraction for Matrix {
    fn Subtract(a: &Self, b: &Self) -> Self {
        // panic if rows and colums don't match
        assert_eq!(a.rows, b.rows);

        let mut matrix = Matrix::new(a.rows, a.cols);

        for i in 0..a.rows {
            for j in 0..a.cols {
                matrix.data[i][j] = a.data[i][j] - b.data[i][j];
            }
        }

        matrix
    }
}

impl math::SelfMultiplication for Matrix {
    fn hadamard(mut self, other: &Self) -> Self {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] *= other.data[i][j];
            }
        }

        self
    }

    fn scale(mut self, value: f64) -> Self {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] *= value;
            }
        }

        self
    }
}

impl math::Multiplication for Matrix {
    fn Multiply(a: &Self, b: &Self) -> Self {
        // panic if rows and colums don't match
        assert_eq!(a.cols, b.rows);

        let mut matrix = Self::new(a.rows, b.cols);

        for i in 0..matrix.rows {
            for j in 0..matrix.cols {
                let mut sum = 0.0;

                for k in 0..a.cols {
                    sum += a.data[i][k] * b.data[k][j];
                }

                matrix.data[i][j] = sum;
            }
        }

        matrix
    }
}
