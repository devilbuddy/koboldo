pub struct Camera {
    pub position : (f64, f64),
    size : (u32, u32)
}

impl Camera {
    pub fn new(size : (u32, u32)) -> Camera {
        Camera {
            position : (0f64, 0f64),
            size : size
        }
    }
}
