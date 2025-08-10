use serde::Serialize;
use tf_demo_parser::demo::vector::Vector;

extern crate core;

pub mod parser;
pub mod schema;
pub mod web;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

type Vec3 = nalgebra::Point<f32, 3>;
type Vec2 = nalgebra::Point2<f32>;

fn convert_vec(vec: Vector) -> Vec3 {
    Vec3::new(vec.x, vec.y, vec.z)
}

/// For use with serde's [serialize_with] attribute
fn ordered_vec<S, V: Ord + Serialize + Clone>(
    value: &Vec<V>,
    serializer: S,
) -> core::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut ordered = (*value).clone();
    ordered.sort();
    ordered.serialize(serializer)
}
