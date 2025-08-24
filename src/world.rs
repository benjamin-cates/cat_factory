use std::{
    fmt::Write,
    ops::{Index, IndexMut},
};

use crate::{
    levels::{WinRequirement, WinState},
    menu::button_held,
    object::{MoveType, Object, ObjectInfo},
    util::{Direction, Point},
};
use turbo::*;

#[turbo::serialize]
pub enum Edit {
    /// Contains (point, index, old_activity)
    Wiring(Point, usize, bool),
    /// Contains (old_location, old_index, new_location)
    MoveObject(Point, usize, Point),
    /// Contains (point, index, old_info)
    ChangeObjInfo(Point, usize, ObjectInfo),
    /// Contains (point, index, old_animation_tween)
    SetAnimation(Point, usize, i32),
}

#[turbo::serialize]
pub struct World {
    /// Width of the world
    pub width: usize,
    /// Height of the world
    pub height: usize,
    /// List of cells in the world, each with a list of objects
    pub inner: Vec<Vec<Object>>,
    /// Which function will be used to score winning
    pub requirements: Vec<WinRequirement>,
    /// List of wires, each have four inputs
    pub wiring: Vec<[bool; 4]>,
    /// How many moves have been done
    pub move_id: usize,
    /// List of (move_id, edit)
    pub edit_history: Vec<(usize, Edit)>,
    /// Current win state (win, dead, or playing)
    pub win_state: WinState,
    /// Caption to always display
    pub caption: String,
    /// Hint to only show when the user clicks the button
    pub hint: String,
    /// Conveyor belt timer
    pub conveyance: u32,
}

impl World {
    /// Convert world space point to screen space
    pub fn to_screen_space(point: Point) -> (i32, i32) {
        (point.x() * 38 + point.y() * 14, point.y() * 28)
    }
    /// Undo the previous move
    pub fn undo(&mut self) {
        self.win_state = WinState::Alive;
        if self.move_id == 0 {
            return;
        }
        self.move_id = self.move_id - 1;
        while let Some((_, edit)) = self.edit_history.pop_if(|v| v.0 == self.move_id) {
            match edit {
                Edit::ChangeObjInfo(point, idx, info) => {
                    if matches!(info, ObjectInfo::Door(..)) {
                        audio::play("door");
                    }
                    self[point][idx].obj_type = info;
                }
                Edit::MoveObject(old_point, idx, new_point) => {
                    let Some(mut obj) = self[new_point].pop() else {
                        panic!();
                    };
                    obj.position = old_point;
                    self[old_point].insert(idx, obj);
                    let new_pos = World::to_screen_space(old_point);
                    self[old_point][idx].draw_pos.0.set(new_pos.0);
                    self[old_point][idx].draw_pos.1.set(new_pos.1);
                }
                Edit::Wiring(point, idx, active) => {
                    self.wiring[point.x() as usize + point.y() as usize * self.width][idx] = active;
                }
                Edit::SetAnimation(point, idx, anim) => {
                    self[point][idx].animation.set(anim);
                }
            }
        }
    }
    /// Returns true if the world is in a win state
    pub fn check_win(&mut self) {
        let reqs = self.win_requirements();
        if reqs.iter().all(|v| v.0 == v.1) && self.win_state != WinState::Won {
            self.win_state = WinState::Won;
            audio::play("win");
            for point in self.cells_iterator() {
                for i in 0..self[point].len() {
                    if self[point][i].obj_type == ObjectInfo::Cat
                        || self[point][i].obj_type == ObjectInfo::Goal
                    {
                        self.set_animation(point, i, 30, 30);
                    }
                }
            }
        }
        for i in 0..reqs.len() {
            let text = format!("{}/{} {}", reqs[i].0, reqs[i].1, reqs[i].2);
            //let done = reqs[i].0 == reqs[i].1;
            text!(text.as_str(), x = 120 + i * 120, y = 6, fixed = true);
        }
    }
    /// Summons an object at that point
    pub fn summon_object(&mut self, point: Point, obj: ObjectInfo) {
        assert!(
            self.point_inside(point),
            "Point {:?} is outside the world",
            point
        );
        let draw_pos = World::to_screen_space(point);
        self[point].push(Object {
            obj_type: obj,
            draw_pos: (Tween::new(draw_pos.0), Tween::new(draw_pos.1)),
            facing: Direction::East,
            position: point,
            animation: Tween::new(0),
        });
    }
    /// Draw the whole world
    pub fn draw(&mut self) {
        // Draw floors
        for y in 0..self.height {
            for x in (0..self.width).rev() {
                if !self[(x, y).into()]
                    .iter()
                    .any(|v| v.obj_type == ObjectInfo::Barrier)
                {
                    let pos = World::to_screen_space((x, y).into());
                    sprite!(
                        ["factory/floor3", "factory/floor3_1"][(x + y) as usize % 2],
                        x = pos.0,
                        y = pos.1
                    );
                }
            }
        }
        // Tuples of (location, index, z-index)
        let mut draw_array: Vec<(Point, usize, i32)> = vec![];
        // Iterate over all grid cells and add to sprite list
        for y in 0..self.height {
            for x in (0..self.width).rev() {
                let pos = (x, y).into();
                for i in 0..self[pos].len() {
                    let mut world_pos = self[pos][i].draw_pos;
                    let z_index = self[pos][i].draw_height() + world_pos.1.get() * 50
                        - world_pos.0.get() * 25;
                    draw_array.push((pos, i, z_index));
                }
            }
        }
        // Sort sprite list by z-index
        draw_array.sort_by_key(|v| v.2);
        // Draw items in sprite array
        for (position, index, _) in draw_array {
            self[position][index].draw();
        }
        // Draw move count text
        let move_count = format!("Moves: {}", self.move_id);
        text!(move_count.as_str(), x = 35, y = 6, fixed = true);
        // Hint button
        let button_bounds = Bounds::with_size(50, 20)
            .anchor_right(&turbo::screen())
            .anchor_top(&turbo::screen())
            .translate_y(5)
            .translate_x(-5);
        if self.hint.len() != 0 && button_held("See hint", button_bounds, 0x888888FF, 0x777777FF) {
            text_box!(
                self.hint.as_str(),
                bounds = button_bounds
                    .left_of_self()
                    .adjust_width(200)
                    .translate_x(-205),
                align = "right",
                fixed = true,
            );
        }
        // Draw caption
        text_box!(
            self.caption.as_str(),
            bounds = Bounds::with_size(350, 40)
                .anchor_center_x(&turbo::screen())
                .anchor_bottom(&turbo::screen()),
            align = "center",
            fixed = true,
        );
    }
    pub fn cells_iterator<'a>(&'a self) -> impl Iterator<Item = Point> + use<> {
        let width = self.width;
        (0..(self.width * self.height))
            .map(move |v| Point::from(((v % width) as i32, (v / width) as i32)))
    }
    /// Iterate over which order the points should be pushed in if going in a certain direction.
    /// For example, if we are pushing West, we would want to start from the left and end with the right.
    pub fn push_order_points(&self, dir: Direction) -> impl Iterator<Item = Point> + use<> {
        let width = self.width.clone();
        let height = self.height.clone();
        match dir {
            Direction::East => Box::new(
                (0..(width - 1))
                    .rev()
                    .flat_map(move |x| std::iter::repeat(x).zip(0..height)),
            ) as Box<dyn Iterator<Item = (usize, usize)>>,
            Direction::West => {
                Box::new((0..width).flat_map(move |x| std::iter::repeat(x).zip(0..height)))
                    as Box<dyn Iterator<Item = (usize, usize)>>
            }
            Direction::North => {
                Box::new((0..width).flat_map(move |x| std::iter::repeat(x).zip(1..height)))
                    as Box<dyn Iterator<Item = (usize, usize)>>
            }
            Direction::South => Box::new(
                (0..width).flat_map(move |x| std::iter::repeat(x).zip((0..(height - 1)).rev())),
            ) as Box<dyn Iterator<Item = (usize, usize)>>,
        }
        .map(|point| (point.0 as i32, point.1 as i32).into())
    }
    /// Runs conveyor belt logic
    pub fn convey(&mut self) {
        if self.conveyance == 1 {
            self.move_id -= 1;
            let mut movements = vec![];
            for position in self.cells_iterator() {
                let mut push_proposal = [false; 8];
                let mut dir: Option<Direction> = None;
                for (i, cell) in self[position].iter().enumerate() {
                    if cell.obj_type == ObjectInfo::Cat
                        || cell.obj_type == ObjectInfo::Goal
                        || cell.obj_type == ObjectInfo::Box
                        || cell.obj_type == ObjectInfo::Water
                    {
                        push_proposal[i] = true;
                    }
                    match cell.obj_type {
                        ObjectInfo::RotateableConveyor(d, _, false)
                        | ObjectInfo::RotateableConveyor(_, d, true)
                        | ObjectInfo::ToggleableConveyor(d, true) => {
                            dir = Some(d);
                        }
                        _ => {}
                    }
                }
                if let Some(dir) = dir
                    && push_proposal != [false; 8]
                {
                    movements.push((dir, position, push_proposal));
                }
            }
            if movements.len() != 0 && self.win_state != WinState::ConstructingLevel {
                audio::play("conveyor")
            }
            for (dir, position, push_proposal) in movements {
                self.try_movement(dir, position, push_proposal);
            }
            self.move_id += 1;
        }
        self.conveyance = self.conveyance.max(1) - 1;
    }
    /// Run a movement command on the world
    pub fn movement(&mut self, dir: Direction) {
        let num_edits_before = self.edit_history.len();
        for position in self.push_order_points(dir) {
            let mut push_proposal = [false; 8];
            for (i, cell) in self[position].iter().enumerate() {
                if cell.does_move(self) {
                    push_proposal[i] = true;
                }
            }
            self.try_movement(dir, position, push_proposal);
        }
        if num_edits_before != self.edit_history.len() {
            self.move_id += 1;
        }
    }
    /// Try to move the cells at point in the bool array "push_proposal" in the direction dir.
    /// Moves as many of the attempted cells as possible
    pub fn try_movement(&mut self, dir: Direction, point: Point, mut push_proposal: [bool; 8]) {
        // Previous caller is requesting the objects in push_proposal at point to be pushed toward dir

        // If the place we are pushing to is outside the world, it will fail
        if !self.point_inside(point + dir) {
            return;
        }
        // If there is no request, end
        if push_proposal == [false; 8] {
            return;
        }
        // Determine who we must push
        let mut next_push = [false; 8];
        for (j, current_push) in self[point].iter().enumerate() {
            if !push_proposal[j] {
                continue;
            }
            let mut move_abilities = [MoveType::MoveOver; 8];
            self[point + dir]
                .iter()
                .map(|item| item.test_push_by(&current_push.obj_type))
                .enumerate()
                .for_each(|(i, v)| {
                    move_abilities[i] = v;
                });
            if move_abilities.contains(&MoveType::NotAllowed) {
                push_proposal[j] = false;
                continue;
            }
            for (i, ability) in move_abilities.iter().enumerate() {
                if ability == &MoveType::Push {
                    next_push[i] = true;
                }
            }
        }
        // Try pushing the next cell
        self.try_movement(dir, point + dir, next_push);
        for item in self[point + dir].iter() {
            for (j, current_push) in self[point].iter().enumerate() {
                match item.test_push_by(&current_push.obj_type) {
                    MoveType::MoveOver => {}
                    MoveType::NotAllowed | MoveType::Push => {
                        // Object j cannot be moved
                        push_proposal[j] = false;
                    }
                }
            }
        }
        // Move the cells
        let old_dst = self[point + dir].clone();
        let old_src = self[point].clone();
        for (i, _) in push_proposal.iter().enumerate().rev().filter(|(_, m)| **m) {
            self.move_to(point, i, dir);
        }
        self.update_cell(point + dir, &old_dst);
        self.update_cell(point, &old_src);
    }
    /// Returns true if a point is inside the Grid
    pub fn point_inside(&self, point: Point) -> bool {
        point.x() >= 0
            && point.x() < self.width as i32
            && point.y() >= 0
            && point.y() < self.height as i32
    }
    /// Move item at point old_location and index in the direction dir
    pub fn move_to(&mut self, old_location: Point, index: usize, dir: Direction) {
        let mut obj = self[old_location].remove(index);
        self.edit_history.push((
            self.move_id,
            Edit::MoveObject(old_location, index, old_location + dir),
        ));
        obj.facing = dir;
        obj.position = old_location + dir;
        let new_world_pos = World::to_screen_space(old_location + dir);
        obj.draw_pos.0.set_duration(10);
        obj.draw_pos.1.set_duration(10);
        obj.draw_pos.0.set(new_world_pos.0);
        obj.draw_pos.1.set(new_world_pos.1);
        self[old_location + dir].push(obj);
    }
    /// Set new items in a cell
    /// This function checks for button presses
    pub fn update_cell(&mut self, point: Point, old: &Vec<Object>) {
        if self[point] == *old {
            return;
        }
        let covered = self[point].iter().any(|v| {
            v.obj_type == ObjectInfo::Box
                || v.obj_type == ObjectInfo::Cat
                || v.obj_type == ObjectInfo::Goal
                || v.obj_type == ObjectInfo::Water
        });
        let has_acid = self[point].iter().any(|v| v.obj_type == ObjectInfo::Death);
        let has_fire = self[point].iter().any(|v| v.obj_type == ObjectInfo::Fire);
        let has_cat = self[point].iter().any(|v| v.obj_type == ObjectInfo::Cat);
        if has_acid && has_cat {
            audio::play("acid_bubbles");
            audio::play("meow");
            self.win_state = WinState::Acid;
        }
        if has_fire && has_cat {
            audio::play("meow");
            audio::play("fire");
            self.win_state = WinState::Burnt;
        }
        for i in 0..self[point].len() {
            match self[point][i].obj_type {
                ObjectInfo::PushButton(wire_dst, wiring_idx) => {
                    if self.set_wiring(wire_dst, wiring_idx, covered) {
                        self.set_animation(point, i, if covered { 1 } else { 0 }, 1);
                    }
                }
                ObjectInfo::ToggleButton(wire_dst, wiring_idx) => {
                    if covered {
                        let current = self.wiring
                            [(wire_dst.x() + wire_dst.y() * self.width as i32) as usize]
                            [wiring_idx];
                        if self.set_wiring(wire_dst, wiring_idx, !current) {
                            self.set_animation(point, i, if !current { 1 } else { 0 }, 1);
                        }
                    }
                }
                ObjectInfo::RotateableConveyor(_, _, _)
                | ObjectInfo::ToggleableConveyor(_, true) => {
                    if covered {
                        self.conveyance = 15;
                    }
                }
                ObjectInfo::Box | ObjectInfo::Goal => {
                    if has_fire {
                        self.edit_history.push((
                            self.move_id,
                            Edit::ChangeObjInfo(point, i, self[point][i].obj_type.clone()),
                        ));
                        self[point][i].obj_type = ObjectInfo::BurntBox;
                        audio::play("fire");
                        self.set_animation(point, i, 10, 30);
                    }
                }
                ObjectInfo::Water => {
                    if has_fire {
                        audio::play("fire_out");
                        for k in 0..self[point].len() {
                            if self[point][k].obj_type == ObjectInfo::Fire {
                                self[point][k].obj_type = ObjectInfo::FireOut;
                                self.edit_history.push((
                                    self.move_id,
                                    Edit::ChangeObjInfo(point, k, ObjectInfo::Fire),
                                ));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    /// Set the activity status of a wire.
    /// Will update wired objects like doors.
    pub fn set_wiring(&mut self, point: Point, wiring_idx: usize, active: bool) -> bool {
        let move_id = self.move_id;
        let wire_loc = (point.x() + point.y() * self.width as i32) as usize;
        if self.wiring[wire_loc][wiring_idx] == active {
            return false;
        }
        self.edit_history.push((
            move_id,
            Edit::Wiring(point, wiring_idx, self.wiring[wire_loc][wiring_idx]),
        ));
        self.wiring[wire_loc][wiring_idx] = active;
        let new_wiring = self.wiring[wire_loc];
        for i in 0..self[point].len() {
            match self[point][i].obj_type {
                ObjectInfo::Door(dir, ref mut open) => {
                    let old_open = *open;
                    *open = new_wiring.iter().fold(false, |a, b| a ^ b);
                    if *open != old_open {
                        self.edit_history.push((
                            move_id,
                            Edit::ChangeObjInfo(point, i, ObjectInfo::Door(dir, old_open)),
                        ));
                        if self.win_state != WinState::ConstructingLevel {
                            audio::play("door");
                        }
                        self.set_animation(point, i, if old_open { 0 } else { 2 }, 5);
                    }
                }
                ObjectInfo::RotateableConveyor(dir1, dir2, ref mut on) => {
                    let old_on = *on;
                    *on = new_wiring.iter().fold(false, |a, b| a ^ b);
                    if *on != old_on {
                        self.conveyance = self.conveyance.max(1);
                        self.edit_history.push((
                            move_id,
                            Edit::ChangeObjInfo(
                                point,
                                i,
                                ObjectInfo::RotateableConveyor(dir1, dir2, old_on),
                            ),
                        ));
                    }
                }
                ObjectInfo::ToggleableConveyor(dir, ref mut on) => {
                    let old_on = *on;
                    *on = new_wiring.iter().fold(false, |a, b| a ^ b);
                    if *on != old_on {
                        self.conveyance = self.conveyance.max(1);
                        self.edit_history.push((
                            move_id,
                            Edit::ChangeObjInfo(
                                point,
                                i,
                                ObjectInfo::ToggleableConveyor(dir, old_on),
                            ),
                        ));
                    }
                }
                _ => {}
            }
        }
        return true;
    }
    /// Set the animation to the given value with given duration and log animation in history
    pub fn set_animation(&mut self, point: Point, idx: usize, anim: i32, duration: usize) {
        let old = self[point][idx].animation.end;
        self.edit_history
            .push((self.move_id, Edit::SetAnimation(point, idx, old)));
        self[point][idx].animation.set_duration(duration);
        self[point][idx].animation.set(anim);
    }
    pub fn _print_state(&self) {
        for y in 0..self.height {
            let mut line: String = String::from("[");
            for x in 0..self.width {
                let vec: &Vec<Object> = &self[(x, y).into()];
                if vec.len() == 0 {
                    line.write_str("Void").unwrap();
                } else if vec.len() == 1 {
                    line.write_fmt(format_args!("{:?}", vec[0].obj_type))
                        .unwrap();
                } else {
                    line.write_str("[").unwrap();
                    for obj in vec.iter() {
                        line.write_fmt(format_args!("{:?},", obj.obj_type)).unwrap();
                    }
                    line.write_str("]").unwrap();
                }
                line.write_str(", ").unwrap();
            }
            line.write_str("]").unwrap();
            turbo::log!("{}", line);
        }
        turbo::log!("");
    }
}

impl IndexMut<Point> for World {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        if !self.point_inside(index) {
            turbo::log!("Error accessing point at {:?}", index);
        }
        &mut self.inner[index.y() as usize * self.width + index.x() as usize]
    }
}
impl Index<Point> for World {
    type Output = Vec<Object>;
    fn index(&self, index: Point) -> &Self::Output {
        if !self.point_inside(index) {
            turbo::log!("Error accessing point at {:?}", index);
        }
        &self.inner[index.y() as usize * self.width + index.x() as usize]
    }
}
