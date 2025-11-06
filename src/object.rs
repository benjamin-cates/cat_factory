use turbo::{time::tick, *};

use crate::{
    util::{Direction, Point},
    world::World,
};

#[turbo::serialize]
#[derive(PartialEq)]
pub enum ObjectInfo {
    Cat,
    CatCrouch,
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
    Portal(Vec<Point>, bool, u32),
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
            ObjectInfo::CatCrouch => 500,
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
            ObjectInfo::Portal(..) => -500,
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
            ObjectInfo::CatCrouch => {
                sprite!("house/cat_crouch", x = x + 5, y = y - 10);
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
                    sprite!("goal6", x = x, y = y - 16 - (anim - 8) * 12)
                }
            }
            // TRAPS
            ObjectInfo::Trap => sprite!(
                "trap2",
                x = x + ((tick() as i32 / 12 + y) % 6 - 3).abs(),
                y = y + (tick() as i32 / 15 + x) % 4 / 2 - 1,
                flip_x = ((tick() + x as usize) % (75 + x as usize % 10)) < 25
            ),
            ObjectInfo::Death => {
                if tick() % 30 < 15 {
                    sprite!("factory/acid", x = x, y = y, frame = 0)
                } else {
                    sprite!("factory/acid", x = x, y = y, frame = 1)
                }
            }
            ObjectInfo::Fire => {
                sprite!(
                    "factory/better_fire",
                    frame = ((tick() as f32 + y as f32 * 10.0 + x as f32 * 5.0 + x as f32 % 51.0)
                        / 8.0) as usize,
                    x = x + 5,
                    y = y - 36
                )
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
            ObjectInfo::WallBack(false) => sprite!("factory/back_wall2", x = x - 2, y = (y - 32)),
            ObjectInfo::WallFront => sprite!("factory/front_wall", x = x + 14, y = y + 25),
            ObjectInfo::WallLeft(true) => sprite!("factory/left_wall", x = x, y = y - 32),
            ObjectInfo::WallLeft(false) => sprite!("factory/right_wall", x = x - 4, y = y - 27),
            ObjectInfo::WallRight(true) => {
                sprite!("factory/right_wall_short", x = x + 37, y = y - 34)
            }
            ObjectInfo::WallRight(false) => sprite!("factory/right_wall", x = x + 38, y = y - 27),

            // BUTTONS
            ObjectInfo::PushButton(..) => {
                if anim == 0 {
                    sprite!("house/push2_up", x = x, y = y - 1)
                } else if anim == 1 {
                    sprite!("house/push2_middle", x = x, y = y - 1)
                } else {
                    sprite!("house/push2_down", x = x, y = y - 1)
                }
            }
            ObjectInfo::ToggleButton(..) => {
                if anim == 0 {
                    sprite!("house/toggle2_up", x = x, y = y)
                } else if anim == 1 {
                    sprite!("house/toggle2_middle", x = x, y = y)
                } else {
                    sprite!("house/toggle2_down", x = x, y = y)
                }
            }

            // DOORS
            ObjectInfo::Door(Direction::South | Direction::North, _) => {
                sprite!(
                    "factory/door_vertical",
                    x = x + 18,
                    y = y - 19,
                    frame = anim as usize,
                );
            }
            ObjectInfo::Door(Direction::East | Direction::West, _) => {
                sprite!(
                    "factory/door_horizontal",
                    x = x + 5,
                    y = y - 6,
                    frame = anim as usize,
                );
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

            //PORTALS
            ObjectInfo::Portal(_, false, _) => sprite!("factory/portal_closed", x = x, y = y),
            ObjectInfo::Portal(_, true, color) => {
                sprite!("factory/portal_open", color = color, x = x, y = y)
            }
        }
    }
    pub fn draw_wires(&self, world: &World) {
        let half_point = (
            World::to_screen_space((1, 1).into()).0 / 2,
            World::to_screen_space((1, 1).into()).1 / 2,
        );
        match self.obj_type {
            ObjectInfo::PushButton(point, idx) | ObjectInfo::ToggleButton(point, idx) => {
                let active = world.get_wiring(point, idx);
                let color = if active { 0xcfb538FF } else { 0xCCCCCCFF };
                let start = World::to_screen_space(point);
                let end = World::to_screen_space(self.position);
                path!(
                    start = (start.0 + half_point.0, start.1 + half_point.1),
                    end = (end.0 + half_point.0, end.1 + half_point.1),
                    rounded = true,
                    width = 2,
                    color = color,
                );
            }
            ObjectInfo::Portal(ref dests, _active, color) => {
                for dest in dests.iter() {
                    let start = World::to_screen_space(self.position);
                    let end = World::to_screen_space(dest.clone() + self.position);
                    path!(
                        start = (start.0 + half_point.0, start.1 + half_point.1),
                        end = (end.0 / 2 + half_point.0, end.1 / 2 + half_point.1),
                        width = 2,
                        color = color,
                    );
                }
            }
            _ => {}
        }
    }
    pub fn test_push_by(&self, pusher: &ObjectInfo) -> MoveType {
        match self.obj_type {
            ObjectInfo::RotateableConveyor(..) => MoveType::MoveOver,
            ObjectInfo::ToggleableConveyor(..) => MoveType::MoveOver,
            ObjectInfo::Trap => MoveType::MoveOver,
            ObjectInfo::Box => MoveType::Push,
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
            ObjectInfo::CatCrouch => MoveType::Push,
            ObjectInfo::PushButton(..) => MoveType::MoveOver,
            ObjectInfo::ToggleButton(..) => MoveType::MoveOver,
            ObjectInfo::Death => MoveType::MoveOver,
            ObjectInfo::Door(_, true) => MoveType::MoveOver,
            ObjectInfo::Door(_, false) => MoveType::NotAllowed,
            ObjectInfo::Water => MoveType::Push,
            ObjectInfo::Fire => MoveType::MoveOver,
            ObjectInfo::BurntBox => MoveType::MoveOver,
            ObjectInfo::FireOut => MoveType::MoveOver,
            ObjectInfo::Portal(..) => MoveType::MoveOver,
        }
    }
    pub fn does_move(&self, world: &World) -> bool {
        if self.obj_type == ObjectInfo::Cat {
            return true;
        }
        false
    }
}
