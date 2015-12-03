pub struct Camera {
    pub position : (f64, f64),
    pub size : (u32, u32),
    world_size : (u32, u32),
    min_x : f64,
    max_x : f64,
    min_y : f64,
    max_y : f64
}

impl Camera {
    pub fn new(size : (u32, u32)) -> Camera {
        Camera {
            position : (0f64, 0f64),
            size : size,
            world_size : size,
            min_x : 0f64,
            max_x : 0f64,
            min_y : 0f64,
            max_y : 0f64
        }
    }

    pub fn set_world_size(&mut self, w : u32, h : u32) {
        self.world_size = (w, h);
        self.min_x = self.size.0 as f64 / 2f64;
        self.min_y = self.size.1 as f64 / 2f64;
        self.max_x = self.world_size.0 as f64 - self.min_x;
        self.max_y = self.world_size.1 as f64 - self.min_y;
    }

    pub fn set_position(&mut self, x : f64, y : f64) {
        let mut camera_x = x;
        let mut camera_y = y;

        if camera_x < self.min_x {
            camera_x = self.min_x;
        }
        if camera_y < self.min_y {
            camera_y = self.min_y;
        }
        if camera_x > self.max_x {
            camera_x = self.max_x;
        }
        if camera_y > self.max_y {
            camera_y = self.max_y;
        }
        self.position = (camera_x, camera_y);
    }
}
