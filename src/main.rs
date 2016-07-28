#![allow(dead_code, unused_imports, unused_variables, unused_mut, unreachable_code)]

#[macro_use]
extern crate glium;
extern crate image;

// imported files to run
mod part01drawtriangle;
mod part02animatetriangle;
mod part03transformtriangle;
mod part04colortriangle;
mod part05texturetriangle;
mod part06load3d;
mod part07shade3d;

// run main() from the mentioned file
fn main() {
   part02animatetriangle::main();
}
