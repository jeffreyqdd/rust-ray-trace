pub mod camera;
pub mod color;
pub mod common;
pub mod geometry;
pub mod illumination;

// fn main() {
// let m = Matrix3::new(1., 0., 0., 0., 2., 0., 0., 0., 3.0);
// let v = Vector3::new(1., 2., 3.);

// println!("{}", m);
// println!("{}", v);
// println!("{}", m * v);
// let c = color::Color::new(0.2, 0.4, 0.23);
// println!("{}", c);

// let image_width = 256;
// let image_height = 256;

// println!("P3\n{} {}\n255", image_width, image_height);

// for r in 0..image_height {
//     for c in 0..image_width {
//         let r_gain = r as f64 / image_height as f64;
//         let g_gain = c as f64 / image_width as f64;
//         let b_gain = 0.0;

//         let pr = (255.0 * r_gain) as u8;
//         let pb = (255.0 * g_gain) as u8;
//         let pg = (255.0 * b_gain) as u8;
//         println!("{pr} {pb} {pg}");
//     }
// }

// for (int j = 0; j < image_height; ++j) {
//     for (int i = 0; i < image_width; ++i) {
//         auto r = double(i) / (image_width-1);
//         auto g = double(j) / (image_height-1);
//         auto b = 0;

//         int ir = static_cast<int>(255.999 * r);
//         int ig = static_cast<int>(255.999 * g);
//         int ib = static_cast<int>(255.999 * b);

//         std::cout << ir << ' ' << ig << ' ' << ib << '\n';
//     }
// }
// }
