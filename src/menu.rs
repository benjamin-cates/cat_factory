use std::ops::Mul;

use crate::{
    levels::{
        Difficulty, LevelBuilder, PAGE_NAMES, PORTAL_BLUE, PORTAL_GREEN, PORTAL_ORANGE,
        PORTAL_PURPLE, PUZZLE_PAGES, WinRequirement,
    },
    object::ObjectInfo,
    util::Direction,
    world::World,
};
use turbo::*;

#[derive(PartialEq)]
#[turbo::serialize]
pub enum Menu {
    PuzzlePage(usize, usize),
    World(usize, usize),
    Credits,
    Links,
    CustomLevel(String),
}
pub fn button_held(text: &'static str, bounds: Bounds, color_a: u32, color_b: u32) -> bool {
    let play_color = if pointer::screen().intersects_bounds(bounds) {
        color_a
    } else {
        color_b
    };
    rect!(
        bounds = bounds,
        color = play_color,
        fixed = true,
        border_radius = 2,
    );
    let text_bounds = bounds.height(12).anchor_center(&bounds);
    text_box!(
        text,
        bounds = text_bounds,
        align = "center",
        fixed = true,
        color = 0xFFFFFFFF,
    );
    return pointer::screen().intersects_bounds(bounds) && pointer::screen().pressed();
}
pub fn button(text: &'static str, bounds: Bounds, color_a: u32, color_b: u32) -> bool {
    button_held(text, bounds, color_a, color_b) && pointer::screen().just_pressed()
}

impl Menu {
    pub fn run(&self, completed: &Vec<Vec<bool>>) -> Option<(Menu, &'static str)> {
        match self {
            Menu::PuzzlePage(page_id, selected) => {
                let display_bounds = Bounds::with_size(100, 20)
                    .anchor_center(&turbo::screen())
                    .translate_x(-150)
                    .translate_y(-100);
                sprite!(
                    "logo2",
                    x = display_bounds.x(),
                    y = display_bounds.y(),
                    fixed = true
                );
                let top_bounds = Bounds::with_size(100, 20)
                    .anchor_center(&turbo::screen())
                    .translate_x(150)
                    .translate_y(-60);
                let puzzle_names = PUZZLE_PAGES[*page_id];
                text_box!(
                    PAGE_NAMES[*page_id],
                    bounds = top_bounds.translate_y(-30),
                    align = "center",
                    fixed = true,
                );
                for i in 0..puzzle_names.len() {
                    let bounds = top_bounds.translate_y(i * 30);
                    if *selected == i {
                        let color = 0x282828FF
                            + 0x01010100
                                * (turbo::time::tick() as f64 / 5.0).sin().mul(25.0).floor() as u32;
                        rect!(
                            bounds = bounds.expand(2),
                            color = color,
                            fixed = true,
                            border_radius = 2
                        );
                    }
                    let (color_a, color_b) = if completed[*page_id][i] {
                        if *selected == i {
                            (0x3fb84aff, 0x3fb84aff)
                        } else {
                            (0x36b248ff, 0x3fb84aff)
                        }
                    } else {
                        if *selected == i {
                            (0x777777FF, 0x777777FF)
                        } else {
                            (0x777777FF, 0x888888FF)
                        }
                    };
                    let (diff_color, difficulty_char) = match puzzle_names[i].0 {
                        Difficulty::Easy => (0x3fb84aFF, "Easy"),
                        Difficulty::Medium => (0xcbb41cFF, "Med"),
                        Difficulty::Hard => (0xbc4040FF, "Hard"),
                        Difficulty::Tutorial => (0x00000000, " "),
                    };
                    rect!(
                        bounds = bounds.right_of_self().width(30).translate_x(5),
                        color = diff_color,
                        fixed = true,
                        border_radius = 6,
                    );
                    text_box!(
                        difficulty_char,
                        bounds = bounds
                            .right_of_self()
                            .width(30)
                            .translate_x(6)
                            .translate_y(6),
                        fixed = true,
                        align = "center"
                    );
                    if button(puzzle_names[i].1, bounds, color_a, color_b) {
                        return Some((Menu::World(*page_id, i), PUZZLE_PAGES[*page_id][i].1));
                    }
                }
                let right_bounds = Bounds::with_size(45, 20)
                    .anchor_center(&turbo::screen())
                    .translate_x(180)
                    .translate_y(4 * 30);
                let left_bounds = right_bounds.translate_x(-60);
                let mut out = (Menu::PuzzlePage(1000, 1000), "");
                if *page_id == PUZZLE_PAGES.len() - 1 {
                    button("Next", right_bounds, 0x444444FF, 0x444444FF);
                } else if button("Next", right_bounds, 0x777777FF, 0x888888FF) {
                    out = (Menu::PuzzlePage(*page_id + 1, 0), "");
                }
                if *page_id == 0 {
                    button("Prev", left_bounds, 0x444444FF, 0x444444FF);
                } else if button("Prev", left_bounds, 0x777777FF, 0x888888FF) {
                    out = (Menu::PuzzlePage(*page_id - 1, 0), "");
                }
                if out != (Menu::PuzzlePage(1000, 1000), "") {
                    return Some(out);
                }
                if turbo::gamepad::get(0).up.just_pressed() {
                    if *selected != 0 {
                        return Some((Menu::PuzzlePage(*page_id, (*selected).max(1) - 1), ""));
                    }
                }
                if turbo::gamepad::get(0).down.just_pressed() {
                    return Some((
                        Menu::PuzzlePage(*page_id, (*selected + 1).min(puzzle_names.len() - 1)),
                        "",
                    ));
                }
                if turbo::gamepad::get(0).right.just_pressed() {
                    return Some((
                        Menu::PuzzlePage((*page_id + 1).min(PUZZLE_PAGES.len() - 1), 0),
                        "",
                    ));
                }
                if turbo::gamepad::get(0).left.just_pressed() {
                    return Some((Menu::PuzzlePage((*page_id).max(1) - 1, 0), ""));
                }
                if turbo::gamepad::get(0).a.just_pressed()
                    || turbo::keyboard::get().enter().just_pressed()
                    || turbo::keyboard::get().key_e().just_pressed()
                {
                    return Some((
                        Menu::World(*page_id, *selected as usize),
                        PUZZLE_PAGES[*page_id][*selected].1,
                    ));
                }
            }
            Menu::World(page_id, world_id) => {
                if button("Exit", Bounds::new(2, 2, 30, 20), 0x777777FF, 0x888888FF)
                    || turbo::keyboard::get().escape().just_pressed()
                    || (gamepad::get(0).start.just_pressed()
                        && !keyboard::get().space().just_pressed())
                {
                    return Some((Menu::PuzzlePage(*page_id, *world_id), ""));
                }
            }
            Menu::Credits => {
                if button("Exit", Bounds::new(2, 2, 30, 20), 0x777777FF, 0x888888FF)
                    || turbo::keyboard::get().escape().just_pressed()
                    || gamepad::get(0).start.just_pressed()
                {
                    return Some((Menu::PuzzlePage(PUZZLE_PAGES.len() - 1, 0), ""));
                }
                text_box!(
                    "Credits\n\nBenjamin Cates --> Lead programmer, level designer, artist\
                    \n\n\nSound effects via FreeSound.org\n\
                    Thanks to SecureSubset, SilentStrikeZ, Aiyumi, MLaudio, \
                    SilverIllusionist, jbdelgado, and ThomasMillar, Ragnar59\n\n\
                    Built with Rust and Turbo",
                    align = "center",
                    bounds = turbo::new(300, 120).anchor_center(&turbo::screen()),
                    fixed = true,
                );
            }
            Menu::Links => {
                if button("Exit", Bounds::new(2, 2, 30, 20), 0x777777FF, 0x888888FF)
                    || turbo::keyboard::get().escape().just_pressed()
                    || gamepad::get(0).start.just_pressed()
                {
                    return Some((Menu::PuzzlePage(PUZZLE_PAGES.len() - 1, 0), ""));
                }
                text_box!(
                    "GitHub: https://github.com/benjamin-cates/cat_factory\n\n\
                     Find the playtest form and license on the GitHub page\n\n\
                     Turbo: https://turbo.computer",
                    align = "center",
                    bounds = turbo::new(300, 75).anchor_center(&turbo::screen()),
                    fixed = true,
                );
            }
            Menu::CustomLevel(string) => {
                if button("Exit", Bounds::new(2, 2, 30, 20), 0x777777FF, 0x888888FF)
                    || turbo::keyboard::get().escape().just_pressed()
                    || gamepad::get(0).start.just_pressed()
                {
                    return Some((Menu::PuzzlePage(PUZZLE_PAGES.len() - 1, 0), ""));
                }
            }
        }
        return None;
    }
}

impl LevelBuilder {
    pub fn get_menu_world(idx: u8) -> World {
        const T: bool = true;
        const F: bool = false;
        match idx % 8 {
            0 => Self::make_level(
                5,
                5,
                &[
                    &[T, T, T, T, T],
                    &[T, T, T, F, T],
                    &[T, T, T, F, T],
                    &[T, T, T, F, F],
                    &[T, T, T, T, T],
                ],
                WinRequirement::Never,
            )
            .with_obj((4, 4), ObjectInfo::Cat)
            .with_obj((0, 4), ObjectInfo::ToggleButton((3, 2).into(), 0))
            .with_obj((4, 1), ObjectInfo::Door(Direction::East, false))
            .with_obj((4, 2), ObjectInfo::Goal)
            .with_obj((2, 2), ObjectInfo::Death)
            .with_obj((0, 0), ObjectInfo::Trap)
            .with_obj(
                (2, 3),
                ObjectInfo::ToggleableConveyor(Direction::West, true),
            )
            .with_obj(
                (1, 3),
                ObjectInfo::ToggleableConveyor(Direction::North, true),
            )
            .with_obj(
                (1, 2),
                ObjectInfo::ToggleableConveyor(Direction::North, true),
            )
            .finish(),
            1 => LevelBuilder::make_level(
                5,
                5,
                &[
                    &[T, T, T, T, T],
                    &[F, T, T, T, T],
                    &[T, T, T, T, T],
                    &[T, F, F, T, F],
                    &[T, T, T, T, T],
                ],
                WinRequirement::Never,
            )
            .with_obj((0, 0), ObjectInfo::Cat)
            .with_obj((3, 4), ObjectInfo::Goal)
            .with_obj((4, 4), ObjectInfo::Death)
            .with_obj((1, 1), ObjectInfo::Box)
            .with_obj((3, 3), ObjectInfo::Box)
            .with_obj(
                (1, 2),
                ObjectInfo::RotateableConveyor(Direction::West, Direction::North, false),
            )
            .with_obj((3, 1), ObjectInfo::PushButton((1, 2).into(), 0))
            .finish(),
            2 => LevelBuilder::make_level(
                5,
                5,
                &[
                    &[T, F, T, T, T],
                    &[T, F, T, F, T],
                    &[T, T, T, F, T],
                    &[T, T, T, F, T],
                    &[F, F, T, F, T],
                ],
                WinRequirement::Never,
            )
            .with_obj((0, 0), ObjectInfo::Cat)
            .with_obj((2, 4), ObjectInfo::Death)
            .with_obj((0, 2), ObjectInfo::PushButton((2, 1).into(), 0))
            .with_obj((2, 1), ObjectInfo::Door(Direction::East, false))
            .with_obj((1, 2), ObjectInfo::Box)
            .with_obj((4, 4), ObjectInfo::Goal)
            .finish(),
            3 => LevelBuilder::make_level(
                5,
                5,
                &[
                    &[T, T, T, T, T],
                    &[T, F, T, F, T],
                    &[T, F, T, T, T],
                    &[T, F, T, F, T],
                    &[T, T, T, F, T],
                ],
                WinRequirement::Never,
            )
            .with_obj((1, 4), ObjectInfo::Cat)
            .with_obj((0, 0), ObjectInfo::ToggleButton((4, 3).into(), 0))
            .with_obj((4, 3), ObjectInfo::Door(Direction::East, false))
            .with_obj(
                (2, 2),
                ObjectInfo::RotateableConveyor(Direction::North, Direction::East, false),
            )
            .with_obj(
                (2, 3),
                ObjectInfo::ToggleableConveyor(Direction::North, true),
            )
            .with_obj(
                (2, 1),
                ObjectInfo::ToggleableConveyor(Direction::North, true),
            )
            .with_obj((4, 4), ObjectInfo::Goal)
            .with_obj((3, 2), ObjectInfo::Death)
            .with_obj((0, 4), ObjectInfo::ToggleButton((2, 2).into(), 0))
            .finish(),
            4 => LevelBuilder::make_level(
                5,
                5,
                &[
                    &[F, F, F, T, T],
                    &[F, T, T, T, T],
                    &[F, T, T, T, F],
                    &[T, T, T, T, F],
                    &[T, T, F, F, F],
                ],
                WinRequirement::Never,
            )
            .with_obj((1, 1), ObjectInfo::Cat)
            .with_obj((4, 0), ObjectInfo::Fire)
            .with_obj((3, 1), ObjectInfo::Water)
            .with_obj(
                (3, 0),
                ObjectInfo::ToggleableConveyor(Direction::East, true),
            )
            .with_obj(
                (4, 1),
                ObjectInfo::ToggleableConveyor(Direction::North, true),
            )
            .with_obj((2, 2), ObjectInfo::Goal)
            .with_obj((0, 4), ObjectInfo::Fire)
            .with_obj(
                (0, 3),
                ObjectInfo::ToggleableConveyor(Direction::South, true),
            )
            .with_obj(
                (1, 4),
                ObjectInfo::ToggleableConveyor(Direction::West, true),
            )
            .finish(),
            5 => LevelBuilder::make_level(
                5,
                5,
                &[
                    &[T, T, T, T, T],
                    &[F, F, F, F, T],
                    &[T, F, T, F, T],
                    &[T, F, T, F, T],
                    &[T, T, T, F, T],
                ],
                WinRequirement::Never,
            )
            .with_obj(
                (2, 2),
                ObjectInfo::Portal(vec![(0, 0).into()], true, PORTAL_BLUE),
            )
            .with_obj(
                (0, 0),
                ObjectInfo::Portal(vec![(2, 2).into()], true, PORTAL_ORANGE),
            )
            .with_obj((0, 2), ObjectInfo::Cat)
            .with_obj((1, 0), ObjectInfo::PushButton((2, 0).into(), 0))
            .with_obj((2, 0), ObjectInfo::Door(Direction::North, false))
            .with_obj((0, 4), ObjectInfo::PushButton((1, 4).into(), 0))
            .with_obj((1, 4), ObjectInfo::Door(Direction::North, false))
            .with_obj((4, 4), ObjectInfo::Goal)
            .finish(),
            6 => LevelBuilder::make_level(
                5,
                5,
                &[
                    &[T, T, T, T, T],
                    &[F, F, F, F, F],
                    &[T, T, T, T, T],
                    &[F, F, F, F, F],
                    &[T, T, T, T, T],
                ],
                WinRequirement::Never,
            )
            .with_obj((0, 2), ObjectInfo::Cat)
            .with_obj((1, 2), ObjectInfo::Water)
            .with_obj((3, 2), ObjectInfo::Fire)
            .with_obj(
                (4, 2),
                ObjectInfo::Portal(vec![(0, 0).into()], true, PORTAL_BLUE),
            )
            .with_obj(
                (0, 0),
                ObjectInfo::Portal(vec![(4, 2).into()], true, PORTAL_ORANGE),
            )
            .with_obj((2, 0), ObjectInfo::Fire)
            .with_obj(
                (0, 4),
                ObjectInfo::Portal(vec![(4, 0).into()], true, PORTAL_GREEN),
            )
            .with_obj(
                (4, 0),
                ObjectInfo::Portal(vec![(0, 4).into()], true, PORTAL_PURPLE),
            )
            .with_obj((1, 4), ObjectInfo::Fire)
            .with_obj((2, 4), ObjectInfo::Fire)
            .with_obj((3, 4), ObjectInfo::Fire)
            .with_obj((4, 4), ObjectInfo::Fire)
            .finish(),
            7 => LevelBuilder::make_level(
                5,
                5,
                &[
                    &[T, T, T, T, T],
                    &[T, T, T, T, T],
                    &[F, F, F, F, F],
                    &[T, T, T, T, T],
                    &[T, T, T, T, T],
                ],
                WinRequirement::Never,
            )
            .with_obj((2, 1), ObjectInfo::Cat)
            .with_obj((2, 3), ObjectInfo::Cat)
            .with_obj((4, 0), ObjectInfo::Goal)
            .with_obj((0, 3), ObjectInfo::Goal)
            .with_obj(
                (1, 3),
                ObjectInfo::ToggleableConveyor(Direction::East, true),
            )
            .with_obj(
                (1, 4),
                ObjectInfo::ToggleableConveyor(Direction::West, true),
            )
            .finish(),
            _ => Self::make_level(1, 1, &[&[T]], WinRequirement::Never).finish(),
        }
    }
}
