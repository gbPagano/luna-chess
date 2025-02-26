use std::env;
use std::fs::File;
use std::path::Path;

mod bitboard;
mod color;
mod file;
mod gen_files;
mod pieces;
mod rank;
mod square;

use gen_files::{gen_all_magic, gen_rays, write_magics, write_rays};

fn main() {
    gen_rays();
    gen_all_magic();

    let out_dir = env::var("OUT_DIR").unwrap();
    let magic_path = Path::new(&out_dir).join("magic_file.rs");
    let mut f = File::create(&magic_path).unwrap();

    write_rays(&mut f);
    write_magics(&mut f);
}
