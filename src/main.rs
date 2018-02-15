#![allow(dead_code)]

#[macro_use]
mod matrix;
use matrix::Matrix;

mod game;
use game::*;

fn main() {
    let m = matrix![1,    2, 0;
                    4, -100, 0;
                    7,    8, 9];
    let r1: Matrix<_> = m.row(1).into();
    let d: Matrix<_> = m.diag().into();

    println!("{}", r1);
}
