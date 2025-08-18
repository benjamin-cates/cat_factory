use std::ops::Mul;

use crate::levels::{PAGE_NAMES, PUZZLE_PAGES};
use turbo::*;

#[derive(Copy, PartialEq)]
#[turbo::serialize]
pub enum Menu {
    PuzzlePage(usize, usize),
    World(usize, usize),
}

pub fn button(text: &'static str, bounds: Bounds, color_a: u32, color_b: u32) -> bool {
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
    return pointer::screen().just_pressed() && pointer::screen().intersects_bounds(bounds);
}

impl Menu {
    pub fn run(&self, completed: &Vec<Vec<bool>>) -> (Menu, &'static str) {
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
                            (0x23B221FF, 0x23B221FF)
                        } else {
                            (0x23B221FF, 0x29D227FF)
                        }
                    } else {
                        if *selected == i {
                            (0x777777FF, 0x777777FF)
                        } else {
                            (0x777777FF, 0x888888FF)
                        }
                    };
                    if button(puzzle_names[i], bounds, color_a, color_b) {
                        return (Menu::World(*page_id, i), PUZZLE_PAGES[*page_id][i]);
                    }
                }
                let right_bounds = Bounds::with_size(45, 20)
                    .anchor_center(&turbo::screen())
                    .translate_x(180)
                    .translate_y(4 * 30);
                let left_bounds = right_bounds.translate_x(-60);
                if *page_id == PUZZLE_PAGES.len() - 1 {
                    button("Next", right_bounds, 0x444444FF, 0x444444FF);
                } else {
                    if button("Next", right_bounds, 0x777777FF, 0x888888FF) {
                        return (Menu::PuzzlePage(*page_id + 1, 0), "");
                    }
                }
                if *page_id == 0 {
                    button("Prev", left_bounds, 0x444444FF, 0x444444FF);
                } else {
                    if button("Prev", left_bounds, 0x777777FF, 0x888888FF) {
                        return (Menu::PuzzlePage(*page_id - 1, 0), "");
                    }
                }
                if turbo::gamepad::get(0).up.just_pressed() {
                    if *selected != 0 {
                        return (Menu::PuzzlePage(*page_id, (*selected).max(1) - 1), "");
                    }
                }
                if turbo::gamepad::get(0).down.just_pressed() {
                    return (
                        Menu::PuzzlePage(*page_id, (*selected + 1).min(puzzle_names.len() - 1)),
                        "",
                    );
                }
                if turbo::gamepad::get(0).right.just_pressed() {
                    return (
                        Menu::PuzzlePage((*page_id + 1).min(PUZZLE_PAGES.len() - 1), 0),
                        "",
                    );
                }
                if turbo::gamepad::get(0).left.just_pressed() {
                    return (Menu::PuzzlePage((*page_id).max(1) - 1, 0), "");
                }
                if turbo::gamepad::get(0).a.just_pressed()
                    || turbo::keyboard::get().enter().just_pressed()
                    || turbo::keyboard::get().key_e().just_pressed()
                {
                    return (
                        Menu::World(*page_id, *selected as usize),
                        PUZZLE_PAGES[*page_id][*selected],
                    );
                }
            }
            Menu::World(page_id, world_id) => {
                if button("Exit", Bounds::new(2, 2, 30, 20), 0x777777FF, 0x888888FF)
                    || turbo::keyboard::get().escape().just_pressed()
                    || gamepad::get(0).start.just_pressed()
                {
                    return (Menu::PuzzlePage(*page_id, *world_id), "");
                }
            }
        }
        return (*self, "");
    }
}
