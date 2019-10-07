extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::camera::FixedView;
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Translation3, Vector2};

fn main() {
    let mut window = Window::new("Solar System");

    let mut c1 = window.add_cube(1.0, 1.0, 1.0);
    let mut c2 = window.add_cube(1.0, 1.0, 1.0);
    let mut c3 = window.add_cube(1.0, 1.0, 1.0);

    c1.set_color(1.0, 0.0, 0.0);
    c2.set_color(0.0, 1.0, 0.0);
    c3.set_color(0.0, 0.0, 1.0);

    window.set_light(Light::StickToCamera);

    let iso1 = Translation3::new(1.0, 1.0, 1.0);
    let iso2 = Translation3::new(-1.0, 1.0, 1.0);

    c2.append_translation(&iso1);
    c3.append_translation(&iso2);

    while window.render() {
        c2.re
    }
}
