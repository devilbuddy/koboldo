use sdl2::{TimerSubsystem};

pub struct MotorTimer {
    timer_subsystem : TimerSubsystem,
    interval : u32,
    last_tick : u32,
    last_second : u32,
    fps : u16,
    log_enabled : bool
}

impl MotorTimer {
    pub fn new(target_fps : u32, mut timer_subsystem : TimerSubsystem) -> MotorTimer {

        let now = timer_subsystem.ticks();

        MotorTimer {
            timer_subsystem : timer_subsystem,
            interval : 1000 / target_fps,
            last_tick : now,
            last_second : now,
            fps : 0,
            log_enabled : false
        }
    }

    pub fn tick(&mut self) -> (bool, f64) {
        let now = self.timer_subsystem.ticks();
        let dt = now - self.last_tick;
        let elapsed = dt as f64 / 1000.0;
        if dt < self.interval {
            self.timer_subsystem.delay(self.interval - dt);
            return (false, 0f64);
        }
        self.last_tick = now;

        if self.log_enabled {
            self.fps += 1;
            if now - self.last_second > 1000 {
                println!("FPS: {}", self.fps);
                self.last_second = now;
                self.fps = 0;
            }
        }
        return (true, elapsed);
    }

    pub fn set_enable_fps_log(&mut self, enabled : bool) {
        self.log_enabled = enabled;
    }
}
