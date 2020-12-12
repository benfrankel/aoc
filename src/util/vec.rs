use nalgebra::{Scalar, Vector2, Vector3};

pub fn vec2<T>(a: T, b: T) -> Vector2<T>
where
    T: Scalar,
{
    Vector2::from_vec(vec![a, b])
}

pub fn vec3<T>(a: T, b: T, c: T) -> Vector3<T>
where
    T: Scalar,
{
    Vector3::from_vec(vec![a, b, c])
}
