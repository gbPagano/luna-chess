pub mod attacks;
pub mod between;
pub mod lines;
pub mod magics;
pub mod rays;
pub mod knight;

pub use between::{gen_between, write_between};
pub use lines::{gen_lines, write_lines};
pub use magics::{gen_all_magic, write_magics};
pub use rays::{gen_rays, write_rays};
pub use knight::{gen_knight_moves, write_knight_moves};
