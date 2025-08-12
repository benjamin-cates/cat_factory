use crate::object::ObjectInfo;
use crate::util::Direction;
use crate::world::World;

pub const WIN_FUNCTIONS: &'static [fn(&World) -> bool] = &[
    // 0: Never win
    |_world: &World| false,
    // 1: Every cat is in a goal
    |world: &World| {
        for p in world.cells_iterator() {
            if world[p].iter().any(|v| v.obj_type == ObjectInfo::Cat)
                && !world[p].iter().any(|v| v.obj_type == ObjectInfo::Goal)
            {
                return false;
            }
        }
        true
    },
];
pub const PUZZLE_PAGES: &'static [&'static [&'static str]] = &[
    &["Movement", "Traps", "Buttons", "Box Bridge"],
    &["Easy Box", "Pushing My Buttons", "Acid River", "Box Maze"],
    &["Cat Coordination", "Pushing My Boxes"],
    &["one", "two", "three"],
];
pub const PAGE_NAMES: &'static [&'static str] = &[
    "Tutorial",
    "Pushing boxes",
    "Double cat world",
    "Junk levels (don't play)",
];

impl World {
    /// Make a new world using the floors as a 2d bool array of where the floors will be.
    pub fn make_world(
        width: usize,
        height: usize,
        floors: &'static [&'static [bool]],
        win_function: usize,
    ) -> Self {
        assert_eq!(
            height,
            floors.len(),
            "Height {} does not match floors",
            height
        );
        let mut out = Self {
            win_function,
            width,
            height,
            inner: vec![vec![]; width * height],
            wiring: vec![[false; 4]; width * height],
            move_id: 0,
            edit_history: vec![],
            dead: false,
            won: false,
            caption: "".to_string(),
            hint: "".to_string(),
        };
        for y in 0..height {
            assert_eq!(
                width,
                floors[y].len(),
                "Width {} does not match floors on index {}",
                width,
                y
            );
            for x in 0..width {
                if !floors[y][x] {
                    out.summon_object((x, y).into(), ObjectInfo::Barrier);
                    continue;
                }
                if y == height - 1 || floors[y + 1][x] == false {
                    out.summon_object((x, y).into(), ObjectInfo::WallFront);
                }
                if y == 0 {
                    out.summon_object((x, y).into(), ObjectInfo::WallBack(false));
                } else if floors[y - 1][x] == false {
                    // If any floors are above, draw this wall short
                    let is_short = (0..(y - 1)).any(|i| floors[i][x]);
                    out.summon_object((x, y).into(), ObjectInfo::WallBack(is_short));
                }
                if x == width - 1 {
                    out.summon_object((x, y).into(), ObjectInfo::WallRight(false));
                } else if floors[y][x + 1] == false {
                    // If any floors are to the right, draw this wall short
                    let is_short = (0..y).any(|i| floors[i][x + 1]);
                    out.summon_object((x, y).into(), ObjectInfo::WallRight(is_short));
                }
                if x == 0 {
                    out.summon_object((x, y).into(), ObjectInfo::WallLeft(true));
                } else if floors[y][x - 1] == false {
                    // Is short if it is on the far left, or there is no "back" wall below it
                    let is_short = (0..y).any(|i| floors[i][x - 1]);
                    out.summon_object((x, y).into(), ObjectInfo::WallLeft(is_short));
                }
            }
        }
        out
    }
    /// Adds a caption and returns the self
    pub fn add_caption(mut self, str: &'static str) -> Self {
        self.caption = String::from(str);
        self
    }
    /// Adds a hint and returns the self
    pub fn add_hint(mut self, str: &'static str) -> Self {
        self.hint = String::from(str);
        self
    }
    /// Returns a template world based on the given name
    pub fn get_template(name: &'static str) -> Self {
        const T: bool = true;
        const F: bool = false;
        match name {
            "menu1" => Self::make_world(
                5,
                5,
                &[
                    &[T, T, T, T, T],
                    &[T, F, F, F, T],
                    &[T, F, T, T, T],
                    &[T, F, F, F, F],
                    &[T, T, T, T, T],
                ],
                0,
            )
            .add_objects(vec![
                ((4, 4).into(), ObjectInfo::Cat),
                ((0, 4).into(), ObjectInfo::ToggleButton((3, 2).into(), 0)),
                ((3, 2).into(), ObjectInfo::Door(Direction::North, false)),
                ((2, 2).into(), ObjectInfo::Goal),
            ]),
            "menu2" => Self::make_world(
                5,
                5,
                &[
                    &[T, T, T, T, T],
                    &[F, F, F, F, T],
                    &[T, T, T, T, T],
                    &[T, F, F, F, F],
                    &[T, T, T, T, T],
                ],
                0,
            )
            .add_objects(vec![
                ((0, 0).into(), ObjectInfo::Cat),
                ((4, 4).into(), ObjectInfo::Goal),
            ]),
            "menu3" => Self::make_world(
                5,
                5,
                &[
                    &[T, F, T, T, T],
                    &[T, F, T, F, T],
                    &[T, F, T, F, T],
                    &[T, F, T, F, T],
                    &[T, T, T, F, T],
                ],
                0,
            )
            .add_objects(vec![
                ((0, 0).into(), ObjectInfo::Cat),
                ((4, 4).into(), ObjectInfo::Goal),
            ]),
            "menu4" => Self::make_world(
                5,
                5,
                &[
                    &[T, T, T, T, T],
                    &[T, F, F, F, T],
                    &[T, F, F, F, T],
                    &[T, F, F, F, T],
                    &[T, T, T, F, T],
                ],
                0,
            )
            .add_objects(vec![
                ((0, 0).into(), ObjectInfo::Cat),
                ((2, 4).into(), ObjectInfo::ToggleButton((4, 3).into(), 0)),
                ((4, 3).into(), ObjectInfo::Door(Direction::East, false)),
                ((4, 4).into(), ObjectInfo::Goal),
            ]),
            "Movement" => Self::make_world(
                6,
                3,
                &[
                    &[T, T, T, T, T, T],
                    &[T, T, T, T, T, T],
                    &[T, T, T, T, T, T],
                ],
                1,
            )
            .add_objects(vec![
                ((0, 1).into(), ObjectInfo::Goal),
                ((2, 0).into(), ObjectInfo::Box),
                ((3, 1).into(), ObjectInfo::Box),
                ((4, 2).into(), ObjectInfo::Box),
                ((5, 1).into(), ObjectInfo::Cat),
            ])
            .add_caption("It looks like this cat has escaped his box at the Cat Factory! Can you help guide him back? Press the WASD or arrow keys to move."),
            "Traps" => Self::make_world(
                5,
                3,
                &[
                    &[T,T,T,T,T],
                    &[T,T,T,T,T],
                    &[T,T,T,T,T],
                ],
                1,
            )
            .add_objects(vec![
                ((0,0).into(), ObjectInfo::Goal),
                ((1,0).into(), ObjectInfo::Death),
                ((1,1).into(), ObjectInfo::Death),
                ((3,1).into(), ObjectInfo::Trap),
                ((3,2).into(), ObjectInfo::Trap),
                ((4,2).into(), ObjectInfo::Cat),
            ])
            .add_caption(
                "Avoid the traps! ACID kills cats immediately and the X prevents cats from moving. Press E to undo or press R to reset the level."
            ),
            "Buttons" => Self::make_world(
                7,
                4,
                &[
                    &[T,F,T,T,T,T,T],
                    &[T,F,T,T,T,T,T],
                    &[T,F,T,T,T,T,T],
                    &[T,T,T,T,T,T,T],
                ],
                1,
            )
            .add_objects(vec![
                ((0,0).into(), ObjectInfo::Goal),
                ((0,1).into(), ObjectInfo::Door(Direction::East, false)),
                ((0,2).into(), ObjectInfo::Door(Direction::East, false)),
                ((3,0).into(), ObjectInfo::ToggleButton((0,1).into(), 0)),
                ((5,0).into(), ObjectInfo::PushButton((0,2).into(), 0)),
                ((4,1).into(), ObjectInfo::Box),
                ((3,3).into(), ObjectInfo::Cat),
            ])
            .add_caption("Buttons can open doors. Square buttons toggle on and off. Circle buttons must be held down."),
            "Acid River" => Self::make_world(
                5,
                5,
                &[
                    &[T,T,T,T,T],
                    &[T,T,T,T,T],
                    &[T,T,T,T,T],
                    &[T,T,T,T,T],
                    &[T,T,T,T,T],
                ],
                1,
            )
            .add_objects(vec![
                ((0,4).into(), ObjectInfo::Goal),
                ((1,0).into(), ObjectInfo::Death),
                ((1,1).into(), ObjectInfo::Death),
                ((1,2).into(), ObjectInfo::Door(Direction::North, false)),
                ((0,3).into(), ObjectInfo::Door(Direction::East, false)),
                ((1,3).into(), ObjectInfo::Death),
                ((1,4).into(), ObjectInfo::Death),
                ((0,0).into(), ObjectInfo::PushButton((0,3).into(),0)),
                ((0,1).into(), ObjectInfo::PushButton((1,2).into(),0)),
                ((4,2).into(), ObjectInfo::Cat),
                ((3,1).into(), ObjectInfo::Box),
                ((3,3).into(), ObjectInfo::Box),
            ]),
            "Box Bridge" => Self::make_world(
                5,
                4,
                &[
                    &[T, F, F, T, F],
                    &[T, F, F, T, F],
                    &[T, T, T, T, T],
                    &[T, T, T, T, T],
                ],
                1,
            )
            .add_objects(vec![
                ((0, 0).into(), ObjectInfo::Goal),
                ((0, 1).into(), ObjectInfo::Door(Direction::East, false)),
                ((3, 0).into(), ObjectInfo::PushButton((0, 1).into(), 0)),
                ((3, 1).into(), ObjectInfo::Death),
                ((3, 2).into(), ObjectInfo::Box),
                ((1, 2).into(), ObjectInfo::Box),
                ((4, 2).into(), ObjectInfo::Cat),
            ])
            .add_caption("Boxes can be pushed over acid. Try to get back to your box!!"),
            "Pushing My Buttons" => Self::make_world(
                6,5,
                &[
                    &[T,T,T,T,T,T],
                    &[T,T,T,F,F,F],
                    &[T,T,T,T,T,T],
                    &[T,T,T,T,F,F],
                    &[T,T,T,T,T,T],
                ],
                1,
            )
            .add_objects(vec![
                ((1,0).into(), ObjectInfo::Cat),
                ((3,0).into(), ObjectInfo::Door(Direction::North, false)),
                ((4,0).into(), ObjectInfo::Door(Direction::North, false)),
                ((4,2).into(), ObjectInfo::Door(Direction::North, false)),
                ((5,0).into(), ObjectInfo::Goal),
                ((5,4).into(), ObjectInfo::PushButton((3,0).into(),0)),
                ((5,2).into(), ObjectInfo::ToggleButton((4,0).into(),0)),
                ((1,4).into(), ObjectInfo::PushButton((4,2).into(),0)),
                ((2,1).into(), ObjectInfo::Box),
            ]),
            "Box Maze" => Self::make_world(
                8,
                7,
                &[
                    &[F, T, F, T, T, T, T, T],
                    &[F, T, F, T, F, F, T, T],
                    &[T, T, T, T, T, T, T, T],
                    &[T, T, F, T, F, F, T, F],
                    &[T, T, F, F, F, F, T, F],
                    &[T, T, T, T, T, T, T, F],
                    &[F, T, T, T, F, F, F, F],
                ],
                1,
            )
            .add_objects(vec![
                ((1, 0).into(), ObjectInfo::Goal),
                ((1, 1).into(), ObjectInfo::Door(Direction::East, false)),
                ((4, 5).into(), ObjectInfo::Door(Direction::North, false)),
                ((3, 1).into(), ObjectInfo::Box),
                ((3, 5).into(), ObjectInfo::Box),
                ((6, 4).into(), ObjectInfo::Box),
                ((6, 5).into(), ObjectInfo::Cat),
                ((3, 2).into(), ObjectInfo::Death),
                ((3, 3).into(), ObjectInfo::PushButton((4, 5).into(), 0)),
                ((4, 2).into(), ObjectInfo::PushButton((1, 1).into(), 0)),
            ]),
            "Cat Coordination" => Self::make_world(
                7,
                4,
                &[
                    &[T, T, T, F, T, T, T],
                    &[T, T, T, T, F, F, T],
                    &[T, T, F, T, T, T, F],
                    &[T, T, T, T, F, F, F],
                ],
                1,
            )
            .add_objects(vec![
                ((6, 1).into(), ObjectInfo::Cat),
                ((5, 0).into(), ObjectInfo::Door(Direction::North, false)),
                ((4, 0).into(), ObjectInfo::Goal),
                ((4, 2).into(), ObjectInfo::Door(Direction::North, false)),
                ((5, 2).into(), ObjectInfo::Goal),
                ((1, 0).into(), ObjectInfo::Cat),
                ((1, 1).into(), ObjectInfo::Box),
                ((0, 2).into(), ObjectInfo::Box),
                ((1, 3).into(), ObjectInfo::PushButton((5, 0).into(), 0)),
                ((2, 3).into(), ObjectInfo::PushButton((4, 2).into(), 0)),
            ])
            .add_caption("Uh oh! It looks like two cats escaped their boxes. They both move in the same direction at your command. Both cats have to be in their boxes to ship them away."),
            "Pushing My Boxes" => Self::make_world(
                8,
                6,
                &[
                    &[T, T, T, T, F, T, T, T],
                    &[T, T, T, T, F, T, T, T],
                    &[T, T, T, T, T, T, T, T],
                    &[T, T, T, T, F, T, T, T],
                    &[T, T, T, T, F, T, T, T],
                    &[T, T, T, T, F, T, T, T],
                ],
                1,
            )
            .add_objects(vec![
                ((0, 1).into(), ObjectInfo::Cat),
                ((5, 0).into(), ObjectInfo::Cat),
                ((6, 1).into(), ObjectInfo::Box),
                ((4, 2).into(), ObjectInfo::Trap),
                ((2, 1).into(), ObjectInfo::Box),
                ((0, 5).into(), ObjectInfo::Goal),
                ((2, 3).into(), ObjectInfo::Goal),
            ]),
            "one" => Self::make_world(5, 1, &[&[true, true, true, true, true]], 0)
                .add_objects(vec![((4, 0).into(), ObjectInfo::Cat)]),
            "two" => Self::make_world(
                5,
                5,
                &[
                    &[T, F, T, T, T],
                    &[T, F, T, T, T],
                    &[T, T, T, F, T],
                    &[T, T, T, T, F],
                    &[F, F, T, T, T],
                ],
                0,
            )
            .add_objects(vec![((2, 0).into(), ObjectInfo::Cat)]),
            "three" => Self::make_world(
                11,
                5,
                &[
                    &[T, T, T, T, F, F, F, T, T, T, T],
                    &[T, T, T, T, T, T, T, T, T, T, T],
                    &[T, T, T, T, F, F, F, T, T, T, T],
                    &[T, T, T, T, F, F, F, T, T, T, T],
                    &[T, T, T, T, T, T, T, T, T, T, T],
                ],
                1,
            )
            .add_objects(vec![
                ((0, 1).into(), ObjectInfo::Goal),
                ((2, 1).into(), ObjectInfo::Cat),
                ((1, 3).into(), ObjectInfo::Box),
                ((2, 3).into(), ObjectInfo::Box),
                ((3, 4).into(), ObjectInfo::Box),
                //((8, 1).into(), ObjectInfo::Cat),
                ((9, 2).into(), ObjectInfo::Box),
                ((9, 4).into(), ObjectInfo::Box),
                ((8, 4).into(), ObjectInfo::Box),
            ]),
            "Easy Box" => Self::make_world(
                5,
                5,
                &[
                    &[F, F, T, T, F],
                    &[T, T, T, T, T],
                    &[F, T, T, T, T],
                    &[F, F, F, F, T],
                    &[F, F, F, F, T],
                ],
                1,
            )
            .add_objects(vec![
                ((0, 1).into(), ObjectInfo::Goal),
                ((2, 1).into(), ObjectInfo::Box),
                ((4, 4).into(), ObjectInfo::Cat),
                ((2, 2).into(), ObjectInfo::PushButton((1, 1).into(), 0)),
                ((1, 1).into(), ObjectInfo::Door(Direction::North, false)),
            ]),
            _ => Self::make_world(1, 1, &[&[true]], 0),
        }
    }
}
