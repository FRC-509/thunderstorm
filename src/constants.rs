pub const SIMULATION: bool = true;
pub const MAX_SPEED: f64 = 4.96824;
pub const MODULE_PIXEL_LOCATIONS: [(i32, i32); 4] = [
    // Front Left
    (216, 84),
    // Back Left
    (216, 84 + 329),
    // Back Right
    (216 + 329, 84 + 329),
    // Front Right
    (216 + 329, 84),
];
pub const CHASSIS_LENGTH_METERS: f64 = 0.7112;
pub const OFFSET_TO_SWERVE_MODULE_METERS: f64 = CHASSIS_LENGTH_METERS / 2.0 - 0.08255;
