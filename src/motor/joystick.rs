use sdl2::{EventPump, GameControllerSubsystem, JoystickSubsystem};
use sdl2::event::Event;
use sdl2::controller::GameController;
use sdl2::joystick::Joystick;

use std::fmt;

pub struct Controller {
    id : i32,
    name : String,
    pub game_controller : GameController
}

impl fmt::Debug for Controller {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.name)
    }
}

pub struct MotorJoystick {
    game_controller_subsystem : GameControllerSubsystem,
    controllers : Vec<Controller>
}

impl MotorJoystick {
    pub fn new(game_controller_subsystem : GameControllerSubsystem) -> MotorJoystick {
        MotorJoystick {
            game_controller_subsystem : game_controller_subsystem,
            controllers : Vec::new()
        }
    }

    pub fn get_controller_id(&self) -> Option<i32> {
        println!("get_controller_id {}", self.controllers.len());
        if self.controllers.len() > 0 {
            return Some(self.controllers[0].id);
        }
        None
    }

    pub fn get_controller(&self, id : i32) -> &Controller {
        let index = self.controllers.iter().position(|ref c| c.id == id).unwrap();
        &self.controllers[index]
    }

    pub fn handle_event(&mut self, event : Event) {
        match event {
            Event::ControllerDeviceAdded {which, ..} => {
                self.add_controller(which);
            },
            Event::ControllerDeviceRemapped {..} => {
                println!("ControllerDeviceRemapped");
            },
            Event::ControllerDeviceRemoved {which, ..} => {
                self.remove_controller(which);
            },
            Event::ControllerAxisMotion{ axis, value: val, .. } => {
                println!("ControllerAxisMotion{:?} {:?} {:?}", event, axis, val);
                // Axis motion is an absolute value in the range
                // [-32768, 32767]. Let's simulate a very rough dead
                // zone to ignore spurious events.
                let dead_zone = 10000;
                if val > dead_zone || val < -dead_zone {
                    println!("Axis {:?} moved to {}", axis, val);
                }
            },
            Event::ControllerButtonDown{ button, .. } => {
               println!("Button {:?} down", button);
            },
            Event::ControllerButtonUp{ button, .. } => {
                println!("Button {:?} up", button);
            },
            _ => {
                //println!("joystick unhandled event {:?}", event );
            }
       }
    }

    fn add_controller(&mut self, id : i32) {
        println!("add_controller {:?}", id);
        let _id = id as u32;
        if self.game_controller_subsystem.is_game_controller(_id) {
            match self.game_controller_subsystem.open(_id) {
               Ok(c) => {
                   let controller = Controller {
                       id : id,
                       name : c.name(),
                       game_controller : c,
                   };
                   println!("added {:?}", controller);
                   self.controllers.push(controller);
               },
               Err(e) => {
                   println!("failed: {:?}", e)
               }
           }
        } else {
             println!("{} is not a game controller", id);
        }
    }

    fn remove_controller(&mut self, id : i32) {
        println!("remove_controller {}", id);

        let removed = self.controllers.iter()
                        .position(|ref c| c.id == id)
                        .map(|i| self.controllers.remove(i));

        if removed.is_some() {
            println!("removed {:?}", removed.unwrap());
        }
    }
}
