pub mod point;
pub mod great_circle;
pub mod great_circle_arc;

pub const VEC_LEN_IS_ZERO: f32 = 10e-6;

#[derive(Debug)]
pub enum SphericalError {
    AntipodalOrTooClosePoints,
    IdenticalGreatCircles,
    PoleAndPointNotNormal
}