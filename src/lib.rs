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
            menu_world: LevelBuilder::get_template(
                ["menu1", "menu2", "menu3", "menu4"][(random::u8() % 4) as usize],
            ),
        }
    }
    fn update(&mut self) {
        let (new_menu, world_name) = self.menu.run(&self.solved_maps);
        self.menu = new_menu;
        if world_name.len() != 0 {
            if world_name == "Credits" {
                self.menu = Menu::Credits;
            } else if world_name == "Links" {
                self.menu = Menu::Links;
            } else {
                self.world = LevelBuilder::get_template(world_name);
            }
        }
        if let Menu::World(page_id, puzzle_id) = self.menu {
            self.world.draw();
            let center = World::to_screen_space(
                (self.world.width as i32 - 1, self.world.height as i32 - 1).into(),
            );
            camera::set_xy(center.0 / 2 + 20, center.1 / 2 + 10);
            self.world.check_win();
            if keyboard::get().key_e().just_pressed() || gamepad::get(0).x.just_pressed() {
                self.world.undo();
            }
            if keyboard::get().key_r().just_pressed() || gamepad::get(0).y.just_pressed() {
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
                    || turbo::keyboard::get().enter().just_pressed()
                    || turbo::gamepad::get(0).a.just_pressed()
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
                    || turbo::keyboard::get().enter().just_pressed()
                    || turbo::gamepad::get(0).a.just_pressed()
                {
                    self.world = LevelBuilder::get_template(PUZZLE_PAGES[page_id][puzzle_id].1);
                }
                return;
            } else {
                self.world.convey();
                if self.world.conveyance == 0 {
                    if turbo::gamepad::get(0).left.just_pressed() {
                        self.world.movement(Direction::West)
                    } else if turbo::gamepad::get(0).right.just_pressed() {
                        self.world.movement(Direction::East)
                    } else if turbo::gamepad::get(0).up.just_pressed() {
                        self.world.movement(Direction::North)
                    } else if turbo::gamepad::get(0).down.just_pressed() {
                        self.world.movement(Direction::South)
                    }
                }
                if self.world.conveyance == 1 {
                    self.world.convey();
                }
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
            if tick() % 600 == 0 && random::u8() < 128 {
                self.menu_world = LevelBuilder::get_template(
                    ["menu1", "menu2", "menu3", "menu4"][(random::u8() % 4) as usize],
                );
            }
            self.menu_world.convey();
            if (tick() % 20 == 0 || tick() % 90 == 0) && self.menu_world.conveyance == 0 {
                self.menu_world
                    .movement(Direction::array_all()[random::between(0, 3) as usize])
            }
            if self.menu_world.conveyance == 1 {
                self.menu_world.convey();
            }
            self.menu_world.win_state = WinState::Alive;
            self.menu_world.draw();
        }
    }
}
