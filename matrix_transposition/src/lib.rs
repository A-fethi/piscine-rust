#[derive(Debug, PartialEq, )]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    let Matrix((a, b), (c, d)) = m;
    Matrix((a, c), (b, d))
}
