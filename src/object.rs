use turbo::{time::tick, *};

use crate::{
    util::{Direction, Point},
    world::World,
};

#[turbo::serialize]
#[derive(PartialEq)]
pub enum ObjectInfo {
    Cat,
    Goal,
    Box,
    Barrier,
    WallLeft(bool),
    WallRight(bool),
    WallBack(bool),
    WallFront,
    PushButton(Point, usize),
    ToggleButton(Point, usize),
    Door(Direction, bool),
    Trap,
    Death,
    ToggleableConveyor(Direction, bool),
    RotateableConveyor(Direction, Direction, bool),
    BurntBox,
    Fire,
    FireOut,
    Water,
}

#[turbo::serialize]
#[derive(PartialEq)]
pub struct Object {
    pub obj_type: ObjectInfo,
    pub draw_pos: (Tween<i32>, Tween<i32>),
    pub facing: Direction,
    pub position: Point,
    pub animation: Tween<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveType {
    NotAllowed,
    Push,
    MoveOver,
}

impl Object {
    pub fn draw_height(&self) -> i32 {
        match self.obj_type {
            ObjectInfo::Trap => -500,
            ObjectInfo::Cat => 500,
            ObjectInfo::Goal => 499,
            ObjectInfo::Box => 500,
            ObjectInfo::Barrier => 0,
            ObjectInfo::WallLeft(_) => 1500,
            ObjectInfo::WallRight(_) => -2000,
            ObjectInfo::WallBack(_) => -500,
            ObjectInfo::WallFront => 2000,
            ObjectInfo::PushButton(..) => -2000,
            ObjectInfo::ToggleButton(..) => -2000,
            ObjectInfo::Door(..) => 510,
            ObjectInfo::Death => -500,
            ObjectInfo::ToggleableConveyor(..) => -500,
            ObjectInfo::RotateableConveyor(..) => -500,
            ObjectInfo::Water => 500,
            ObjectInfo::Fire => -500,
            ObjectInfo::BurntBox => 500,
            ObjectInfo::FireOut => -500,
        }
    }
    pub fn draw(&mut self) {
        let x = self.draw_pos.0.get() as i32;
        let y = self.draw_pos.1.get() as i32;
        let anim = self.animation.get();
        match self.obj_type {
            // OBJECTS
            ObjectInfo::Box => sprite!("box", x = x - 1, y = y - 11),
            ObjectInfo::Cat => {
                sprite!(
                    "house/cat",
                    x = x + 5,
                    y = y - 10,
                    opacity = (1.0 - anim as f32 / 8.0).max(0.0)
                );
            }
            ObjectInfo::Goal => {
                if anim == 0 {
                    sprite!("goal", x = x, y = y - 16)
                } else if anim == 1 || anim == 2 {
                    sprite!("goal2", x = x, y = y - 16)
                } else if anim == 3 || anim == 4 {
                    sprite!("goal3", x = x, y = y - 16)
                } else if anim == 5 || anim == 6 {
                    sprite!("goal4", x = x, y = y - 16)
                } else if anim == 7 || anim == 8 {
                    sprite!("goal5", x = x, y = y - 16)
                } else {
                    sprite!("goal6", x = x, y = y - 16 - (anim - 8) * 10)
                }
            }
            // TRAPS
            ObjectInfo::Trap => sprite!("trap2", x = x, y = y,),
            ObjectInfo::Death => {
                if tick() % 30 < 15 {
                    sprite!("factory/acid", x = x, y = y)
                } else {
                    sprite!("factory/acid_1", x = x, y = y)
                }
            }
            ObjectInfo::Fire => {
                sprite!("factory/fire", x = x + 5, y = y - 25)
            }
            ObjectInfo::BurntBox => {
                sprite!(
                    "factory/smoke",
                    x = World::to_screen_space(self.position).0,
                    y = World::to_screen_space(self.position).1 - anim * 5,
                    opacity = 1.0 - anim as f32 / 10.0
                )
            }
            ObjectInfo::FireOut => {
                sprite!("factory/charcoal", x = x, y = y)
            }
            ObjectInfo::Water => {
                sprite!("factory/water_bucket", x = x, y = y - 10)
            }

            // WALLS
            ObjectInfo::Barrier => {}
            ObjectInfo::WallBack(true) => sprite!("factory/front_wall", x = x, y = y - 3),
            ObjectInfo::WallBack(false) => sprite!("factory/back_wall2", x = x, y = (y - 32)),
            ObjectInfo::WallFront => sprite!("factory/front_wall", x = x + 14, y = y + 25),
            ObjectInfo::WallLeft(true) => sprite!("factory/left_wall", x = x, y = y - 32),
            ObjectInfo::WallLeft(false) => sprite!("factory/right_wall", x = x, y = y - 27),
            ObjectInfo::WallRight(true) => {
                sprite!("factory/right_wall_short", x = x + 37, y = y - 34)
            }
            ObjectInfo::WallRight(false) => sprite!("factory/right_wall", x = x + 38, y = y - 27),

            // BUTTONS
            ObjectInfo::PushButton(..) => {
                if anim == 0 {
                    sprite!("house/push_button_open", x = x, y = y)
                } else {
                    sprite!("house/push_button", x = x, y = y)
                }
            }
            ObjectInfo::ToggleButton(..) => {
                if anim == 0 {
                    sprite!("house/toggle_button_open", x = x, y = y)
                } else {
                    sprite!("house/toggle_button", x = x, y = y)
                }
            }

            // DOORS
            ObjectInfo::Door(Direction::South | Direction::North, _) => {
                if anim == 0 {
                    sprite!("factory/door_vertical_closed", x = x + 19, y = y - 17)
                } else if anim == 1 {
                    sprite!("factory/door_vertical_middle", x = x + 19, y = y - 17)
                } else if anim == 2 {
                    sprite!("factory/door_vertical_open", x = x + 19, y = y - 17)
                }
            }
            ObjectInfo::Door(Direction::East | Direction::West, _) => {
                if anim == 0 {
                    sprite!("factory/door_horizontal_closed", x = x + 6, y = y - 7)
                } else if anim == 1 {
                    sprite!("factory/door_horizontal_middle", x = x + 6, y = y - 7)
                } else if anim == 2 {
                    sprite!("factory/door_horizontal_open", x = x + 6, y = y - 7)
                }
            }
            // CONVEYOR BELTS
            ObjectInfo::RotateableConveyor(dir, phant, false)
            | ObjectInfo::RotateableConveyor(phant, dir, true) => {
                match dir {
                    Direction::North => sprite!("factory/conveyor_up", x = x, y = y),
                    Direction::South => sprite!("factory/conveyor_down", x = x, y = y),
                    Direction::East => sprite!("factory/conveyor_right", x = x, y = y),
                    Direction::West => sprite!("factory/conveyor_left", x = x, y = y),
                }
                match phant {
                    Direction::North => sprite!("factory/phantom_up", x = x, y = y),
                    Direction::South => sprite!("factory/phantom_down", x = x, y = y),
                    Direction::East => sprite!("factory/phantom_right", x = x, y = y),
                    Direction::West => sprite!("factory/phantom_left", x = x, y = y),
                }
            }
            ObjectInfo::ToggleableConveyor(phant, false) => {
                sprite!("factory/conveyor_empty", x = x, y = y);
                match phant {
                    Direction::North => sprite!("factory/phantom_up", x = x, y = y),
                    Direction::South => sprite!("factory/phantom_down", x = x, y = y),
                    Direction::East => sprite!("factory/phantom_right", x = x, y = y),
                    Direction::West => sprite!("factory/phantom_left", x = x, y = y),
                }
            }
            ObjectInfo::ToggleableConveyor(dir, true) => match dir {
                Direction::North => sprite!("factory/conveyor_up", x = x, y = y),
                Direction::South => sprite!("factory/conveyor_down", x = x, y = y),
                Direction::East => sprite!("factory/conveyor_right", x = x, y = y),
                Direction::West => sprite!("factory/conveyor_left", x = x, y = y),
            },
        }
    }
    pub fn test_push_by(&self, pusher: &ObjectInfo) -> MoveType {
        match self.obj_type {
            ObjectInfo::RotateableConveyor(..) => MoveType::MoveOver,
            ObjectInfo::ToggleableConveyor(..) => MoveType::MoveOver,
            ObjectInfo::Trap => MoveType::MoveOver,
            ObjectInfo::Box => {
                if *pusher == ObjectInfo::Goal {
                    MoveType::NotAllowed
                } else {
                    MoveType::Push
                }
            }
            ObjectInfo::Goal => {
                if *pusher == ObjectInfo::Cat {
                    MoveType::MoveOver
                } else {
                    MoveType::Push
                }
            }
            ObjectInfo::Barrier => MoveType::NotAllowed,
            ObjectInfo::WallBack(_) | ObjectInfo::WallFront => MoveType::MoveOver,
            ObjectInfo::WallLeft(_) | ObjectInfo::WallRight(_) => MoveType::MoveOver,
            ObjectInfo::Cat => MoveType::Push,
            ObjectInfo::PushButton(..) => MoveType::MoveOver,
            ObjectInfo::ToggleButton(..) => MoveType::MoveOver,
            ObjectInfo::Death => MoveType::MoveOver,
            ObjectInfo::Door(_, true) => MoveType::MoveOver,
            ObjectInfo::Door(_, false) => MoveType::NotAllowed,
            ObjectInfo::Water => MoveType::Push,
            ObjectInfo::Fire => MoveType::MoveOver,
            ObjectInfo::BurntBox => MoveType::MoveOver,
            ObjectInfo::FireOut => MoveType::MoveOver,
        }
    }
    pub fn does_move(&self, world: &World) -> bool {
        if self.obj_type == ObjectInfo::Cat {
            if world[self.position]
                .iter()
                .any(|v| v.obj_type == ObjectInfo::Trap)
            {
                return false;
            }
            return true;
        }
        false
    }
}
