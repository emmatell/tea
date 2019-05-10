#![deny(rust_2018_idioms, unused_must_use)]

mod circle;
mod direction;
mod lerp;
mod max;
mod min;
mod point;
mod rect;
mod rect_position;
mod size;
mod transform;
mod vector;

#[cfg(feature = "euclid")]
pub use euclid;

pub use self::{
    circle::*, direction::*, lerp::*, max::*, min::*, point::*, rect::*, rect_position::*, size::*,
    transform::*, vector::*,
};
