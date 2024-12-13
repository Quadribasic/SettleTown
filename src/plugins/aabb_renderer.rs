// use bevy::{
//     color::palettes::css::GRAY,
//     gizmos::prelude::GizmoPrimitive2d,
//     math::EulerRot,
//     prelude::{Gizmos, Query, Transform, Vec3Swizzles},
// };

// use crate::components::shapes::Shape;

// fn render_shapes(mut gizmos: Gizmos, query: Query<(&Shape, &Transform)>) {
//     let color = GRAY;
//     for (shape, transform) in query.iter() {
//         let translation = transform.translation.xy();
//         let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
//         // let isometry = Isometry2d::new(translation, Rot2::radians(rotation));
//         match shape {
//             Shape::Rectangle(r) => {
//                 gizmos.primitive_2d(r, translation, rotation, color);
//             } // Shape::Circle(c) => {
//               //     gizmos.primitive_2d(c, translation, 0.0, color);
//               // }
//               // Shape::Triangle(t) => {
//               //     gizmos.primitive_2d(t, translation, 0.0, color);
//               // }
//               // Shape::Line(l) => {
//               //     gizmos.primitive_2d(l, translation, 0.0, color);
//               // }
//               // Shape::Capsule(c) => {
//               //     gizmos.primitive_2d(c, translation, 0.0, color);
//               // }
//               // Shape::Polygon(p) => {
//               //     gizmos.primitive_2d(p, translation, 0.0, color);
//               // }
//         }
//     }
// }
