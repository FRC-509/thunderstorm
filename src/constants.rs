pub const SIMULATION: bool = true;
pub const MAX_SPEED: f64 = 4.96824;
pub const MODULE_LOCATIONS: [(i32, i32); 4] = [
    // Front Left
    (216 + 40 / 2 - 20, 84 + 92 / 2 - 45),
    // Back Left
    (216 + 40 / 2 - 20, 84 + 92 / 2 + 329 - 45),
    // Back Right
    (216 + 40 / 2 + 329 - 20, 84 + 92 / 2 + 329 - 45),
    // Front Right
    (216 + 40 / 2 + 329 - 20, 84 + 92 / 2 - 45),
];
