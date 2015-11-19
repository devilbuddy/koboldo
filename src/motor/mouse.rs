use sdl2::event::Event;

pub struct MotorMouse {
    x : i32,
    y : i32
}

impl MotorMouse {
    pub fn new() -> MotorMouse {
        MotorMouse {
            x : 0,
            y : 0
        }
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn handle_event(&mut self, mouse_event : Event) {
        match mouse_event {
            Event::MouseMotion {x, y, ..} => {
                self.x = x;
                self.y = y;
            },
            Event::MouseButtonDown {mouse_btn, ..} => {
                println!("MouseButtonDown {:?}", mouse_btn);
            },
            Event::MouseButtonUp {mouse_btn, ..} => {
                println!("MouseButtonUp {:?}", mouse_btn);
            },
            _ => {}
        }
    }

}
