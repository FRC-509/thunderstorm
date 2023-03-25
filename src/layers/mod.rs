mod arm;
mod drive;
#[macro_export]
macro_rules! mulr {
    ($x:expr, $y:expr) => {
        ($x as f64 * $y).round() as i32
    };
}
pub use arm::Arm;
pub use drive::Drive;
