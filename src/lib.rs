use std::str::FromStr;

use crate::{
    levels::{LevelBuilder, PUZZLE_PAGES, WinState},
    menu::{Menu, button},
    util::Direction,
    world::World,
};
use turbo::{time::tick, *};

mod levels;
mod menu;
mod object;
mod serde;
mod util;
mod world;

#[turbo::game]
struct GameState {
    world: World,
    menu: Menu,
    menu_world: World,
    solved_maps: Vec<Vec<bool>>,
}

impl GameState {
    fn new() -> Self {
        Self {
            solved_maps: vec![vec![false; 8]; 10],
            world: LevelBuilder::get_template("double_cat"),
            menu: Menu::PuzzlePage(0, 0),
            menu_world: LevelBuilder::get_menu_world(random::u8()),
        }
    }
    fn update(&mut self) {
        let keyboard = keyboard::get();
        let gamepad = gamepad::get(0);
        if let Some((new_menu, world_name)) = self.menu.run(&self.solved_maps) {
            self.menu = new_menu;
            if world_name.len() != 0 {
                if world_name == "Credits" {
                    self.menu = Menu::Credits;
                } else if world_name == "Links" {
                    self.menu = Menu::Links;
                } else if world_name == "Custom" {
                    self.menu = Menu::CustomLevel(String::new());
                } else {
                    self.world = LevelBuilder::get_template(world_name);
                }
            }
        }
        if let Menu::World(page_id, puzzle_id) = self.menu {
            self.world.draw();
            self.world.check_win();
            self.world.process_input(&keyboard, &gamepad);
            if keyboard.key_r().just_pressed() || gamepad.y.just_pressed() {
                self.world = LevelBuilder::get_template(PUZZLE_PAGES[page_id][puzzle_id].1);
            }
            let action_bounds = Bounds::with_size(100, 20).anchor_center(&turbo::screen());
            let action_background_bounds = action_bounds.above_self().adjust_height(20);
            if self.world.win_state == WinState::Won {
                rect!(
                    bounds = action_background_bounds.expand(3),
                    color = 0x222222FF,
                    fixed = true,
                    border_radius = 2
                );
                text_box!(
                    "You won!",
                    bounds = action_background_bounds.translate_y(5),
                    fixed = true,
                    align = "center"
                );
                if button("Main Menu", action_bounds, 0x777777FF, 0x888888FF)
                    || keyboard.enter().just_pressed()
                    || gamepad.a.just_pressed()
                {
                    self.menu = Menu::PuzzlePage(page_id, puzzle_id);
                }
                self.solved_maps[page_id][puzzle_id] = true;
            }
            // If user died
            else if self.world.win_state == WinState::Acid
                || self.world.win_state == WinState::Burnt
            {
                rect!(
                    bounds = action_background_bounds.expand(3),
                    color = 0x222222FF,
                    fixed = true,
                    border_radius = 2
                );
                text_box!(
                    "You died! E to undo",
                    bounds = action_background_bounds.translate_y(5),
                    fixed = true,
                    align = "center"
                );
                if button("Restart...", action_bounds, 0x777777FF, 0x888888FF)
                    || keyboard.enter().just_pressed()
                    || gamepad.a.just_pressed()
                {
                    self.world = LevelBuilder::get_template(PUZZLE_PAGES[page_id][puzzle_id].1);
                }
                return;
            }
        } else if let Menu::PuzzlePage(_page, _selection) = self.menu {
            let center = World::to_screen_space(
                (
                    self.menu_world.width as i32 / 2,
                    self.menu_world.height as i32 / 2,
                )
                    .into(),
            );
            camera::set_xy(center.0 + 95, center.1 - 30);
            if (tick() % 600 == 0 && random::u8() < 128) || keyboard.key_r().just_pressed() {
                self.menu_world = LevelBuilder::get_menu_world(random::u8());
            }
            self.menu_world.convey();
            if (tick() % 20 == 0 || tick() % 90 == 0) && self.menu_world.conveyance == 0 {
                self.menu_world
                    .movement(Direction::array_all()[(random::u8() % 4) as usize])
            }
            if self.menu_world.conveyance == 1 {
                self.menu_world.convey();
            }
            self.menu_world.win_state = WinState::ConstructingLevel;
            self.menu_world.draw();
        } else if let Menu::CustomLevel(ref mut str) = self.menu {
            if button(
                "Play!",
                Bounds::with_size(40, 20)
                    .anchor_top(&turbo::screen())
                    .anchor_right(&turbo::screen())
                    .translate_x(-2)
                    .translate_y(2),
                0x777777FF,
                0x888888FF,
            ) {
                match LevelBuilder::from_str(str.as_str()) {
                    Ok(builder) => {
                        self.world = builder.finish();
                        self.menu = Menu::World(0, 0);
                        return;
                    }
                    Err(mut code) => {
                        code.push_str(" backspace to clear");
                        *str = code;
                    }
                }
            }
            str.extend(keyboard.chars());
            if keyboard.backspace().just_pressed() {
                str.pop();
            }
            let mut to_present = String::new();
            for (i, x) in str.chars().enumerate() {
                if i % 75 == 74 {
                    to_present.push('\n');
                }
                to_present.push(x);
            }
            text_box!(
                "Type custom game code here",
                bounds = turbo::new(200, 20)
                    .anchor_top(&turbo::screen())
                    .anchor_center_x(&turbo::screen())
                    .translate_y(6),
                fixed = true,
                align = "center"
            );
            text_box!(
                to_present.as_str(),
                align = "center",
                bounds = turbo::new(500, 75).anchor_center(&turbo::screen()),
                fixed = true,
            );
        }
    }
}
