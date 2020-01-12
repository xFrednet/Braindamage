use std::num::Wrapping;
use std::fmt::{Debug, UpperHex};
use std::ops::{Sub, Add};

pub trait Cell:
    Copy + Clone + PartialEq +
    Add<Output=Self> + Sub<Output=Self> +
    Default + From<u8> +
    Debug + UpperHex
{
    fn add_overflow(&self, other: &Self) -> Self;
    fn sub_overflow(&self, other: &Self) -> Self;

    fn to_char(&self) -> char;
}

impl Cell for u8 {
    fn add_overflow(&self, other: &Self) -> Self {
        (Wrapping(*self) + Wrapping(*other)).0
    }

    fn sub_overflow(&self, other: &Self) -> Self {
        (Wrapping(*self) - Wrapping(*other)).0
    }

    fn to_char(&self) -> char {
        char::from(*self)
    }
}

impl Cell for u16 {
    fn add_overflow(&self, other: &Self) -> Self {
        (Wrapping(*self) + Wrapping(*other)).0
    }

    fn sub_overflow(&self, other: &Self) -> Self {
        (Wrapping(*self) - Wrapping(*other)).0
    }

    fn to_char(&self) -> char {
        unimplemented!()
    }
}

impl Cell for u32 {
    fn add_overflow(&self, other: &Self) -> Self {
        (Wrapping(*self) + Wrapping(*other)).0
    }

    fn sub_overflow(&self, other: &Self) -> Self {
        (Wrapping(*self) - Wrapping(*other)).0
    }

    fn to_char(&self) -> char {
        unimplemented!()
    }
}