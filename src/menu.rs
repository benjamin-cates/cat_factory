use crate::levels::{PAGE_NAMES, PUZZLE_PAGES};
use turbo::*;

#[derive(Copy, PartialEq)]
#[turbo::serialize]
pub enum Menu {
    PuzzlePage(usize),
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
            Menu::PuzzlePage(page_id) => {
                let display_bounds = Bounds::with_size(100, 20)
                    .anchor_center(&turbo::screen())
                    .translate_x(-150)
                    .translate_y(-100);
                sprite!(
                    "logo",
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
                    let (color_a, color_b) = if completed[*page_id][i] {
                        (0x29D227FF, 0x23B221FF)
                    } else {
                        (0x777777FF, 0x888888FF)
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
                    button("Next", right_bounds, 0x666666FF, 0x666666FF);
                } else {
                    if button("Next", right_bounds, 0x777777FF, 0x888888FF) {
                        return (Menu::PuzzlePage(*page_id + 1), "");
                    }
                }
                if *page_id == 0 {
                    button("Prev", left_bounds, 0x666666FF, 0x666666FF);
                } else {
                    if button("Prev", left_bounds, 0x777777FF, 0x888888FF) {
                        return (Menu::PuzzlePage(*page_id - 1), "");
                    }
                }
            }
            Menu::World(page_id, _) => {
                if button("Exit", Bounds::new(2, 2, 30, 20), 0x777777FF, 0x888888FF) {
                    return (Menu::PuzzlePage(*page_id), "");
                }
            }
        }
        return (*self, "");
    }
}
