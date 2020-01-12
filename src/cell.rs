use std::num::Wrapping;
use std::fmt::{Debug, UpperHex};
use std::ops::{Sub, Add};

pub trait Cell:
    Copy + Clone +
    Add<Output=Self> + Sub<Output=Self> +
    PartialEq<Self> +
    Default + From<u8> +
    Sized +
    Debug + UpperHex
{
    fn add_overflow(&self, other: &Self) -> Self;
    fn sub_overflow(&self, other: &Self) -> Self;

    fn to_char(&self) -> char;
}

pub trait IntCell: Cell {}

impl<T: IntCell> Cell for T
    where
        Wrapping<T>: Add<Output=Wrapping<T>> + Sub<Output=Wrapping<T>>
{
    fn add_overflow(&self, other: &Self) -> Self {
        (Wrapping(*self) + Wrapping(*other)).0
    }

    fn sub_overflow(&self, other: &Self) -> Self {
        (Wrapping(*self) - Wrapping(*other)).0
    }

    fn to_char(&self) -> char {
        let data = unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                ::std::mem::size_of::<Self>(),
            )
        };

        char::from(data[0])
    }
}

impl IntCell for u8 {}

impl IntCell for u16 {}

impl IntCell for u32 {}