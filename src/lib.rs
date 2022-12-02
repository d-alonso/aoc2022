#![allow(dead_code)]

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

mod p1_1;
mod p1_2;
mod p2_1;
