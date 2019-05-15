#![deny(rust_2018_idioms, unused_must_use)]

mod circle;
mod direction;
mod lerp;
mod mat3x2;
mod mat4;
mod max;
mod min;
mod point;
mod rect;
mod rect_position;
mod size;
mod vector;

#[cfg(feature = "euclid")]
pub use euclid;

pub use self::{
    circle::*, direction::*, lerp::*, mat3x2::*, mat4::*, max::*, min::*,
    point::*, rect::*, rect_position::*, size::*, vector::*,
};
