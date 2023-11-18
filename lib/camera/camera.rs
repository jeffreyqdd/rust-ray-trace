// impl RelativeEq for Camera
// where
//     Self::Epsilon: Copy,
// {
//     fn default_max_relative() -> Self::Epsilon {
//         todo!()
//     }

//     fn relative_eq(
//         &self,
//         other: &Self,
//         epsilon: Self::Epsilon,
//         max_relative: Self::Epsilon,
//     ) -> bool {
//         todo!()
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use approx;
//     use nalgebra::{point, vector};
//     #[test]
//     fn camera_placement_simple() {
//         let camera = Camera::new(
//             1.,
//             point![3., 1.7, 5.],
//             point![0., 0., 0.],
//             vector![0., 1., 0.],
//             25.,
//             16. / 9.,
//         );

//         assert_eq!(camera.f, 1.);
//         assert_eq!(camera.height, 0.4433893252858798);
//         assert_eq!(camera.width, 0.7882476893971196);
//         approx::assert_relative_eq!(
//             camera.image_plane,
//             Plane::new(
//                 Point3::new(2.5060682687212723, 1.4201053522753877, 4.176780447868787),
//                 UnitVector3::new_unchecked(vector![
//                     0.4939317301933588,
//                     0.2798946549604034,
//                     0.8232195503222647
//                 ])
//             ),
//             epsilon = 1e-12
//         );

//         assert_eq!(camera.eye, point![3., 1.7, 5.]);
//         approx::assert_relative_eq!(
//             camera.m,
//             Transform3::from_matrix_unchecked(Matrix4::new(
//                 0.8574929257125442,
//                 -0.14400460822119582,
//                 0.4939317312787275,
//                 0.,
//                 0.,
//                 0.9600307214746386,
//                 0.27989464772461226,
//                 0.,
//                 -0.5144957554275266,
//                 -0.24000768036865966,
//                 0.8232195521312126,
//                 0.,
//                 0.,
//                 0.,
//                 0.,
//                 1.
//             )),
//             epsilon = 1e-12
//         );
//     }
// }
