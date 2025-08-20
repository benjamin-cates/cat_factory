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
    // 2: Fires extinguished
    |world: &World| {
        for p in world.cells_iterator() {
            if world[p].iter().any(|v| v.obj_type == ObjectInfo::Fire) {
                return false;
            }
        }
        return true;
    },
];

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub enum WinState {
    Won,
    Alive,
    Burnt,
    Acid,
    ConstructingLevel,
}
pub const PUZZLE_PAGES: &'static [&'static [&'static str]] = &[
    &[
        "Movement",
        "Traps",
        "Buttons",
        "Box Bridge",
        "Conveyor Alley",
    ],
    &["Easy Box", "Pushing My Buttons", "Acid River", "Box Maze"],
    &[
        "Cat Coordination",
        "Help Me Out!",
        "Parking Lot",
        "Pushing My Boxes",
        "One-way Door",
    ],
    &["Playing with Fire"],
    &["one", "two", "three", "Conveyance Test", "Fire test"],
];
pub const PAGE_NAMES: &'static [&'static str] = &[
    "Tutorial",
    "Pushing boxes",
    "Two Cat Conundrum",
    "Factory Emergency",
    "Junk levels (don't play)",
];

pub struct LevelBuilder {
    world: World,
}

impl LevelBuilder {
    /// Make a new world using the floors as a 2d bool array of where the floors will be.
    fn make_level(
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
            world: World {
                win_function,
                width,
                height,
                inner: vec![vec![]; width * height],
                wiring: vec![[false; 4]; width * height],
                move_id: 0,
                edit_history: vec![],
                win_state: WinState::ConstructingLevel,
                caption: "".to_string(),
                hint: "".to_string(),
                conveyance: 0,
            },
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
                    out.world.summon_object((x, y).into(), ObjectInfo::Barrier);
                    continue;
                }
                if y == height - 1 || floors[y + 1][x] == false {
                    out.world
                        .summon_object((x, y).into(), ObjectInfo::WallFront);
                }
                if y == 0 {
                    out.world
                        .summon_object((x, y).into(), ObjectInfo::WallBack(false));
                } else if floors[y - 1][x] == false {
                    // If any floors are above, draw this wall short
                    let is_short = (0..(y - 1)).any(|i| floors[i][x]);
                    out.world
                        .summon_object((x, y).into(), ObjectInfo::WallBack(is_short));
                }
                if x == width - 1 {
                    out.world
                        .summon_object((x, y).into(), ObjectInfo::WallRight(false));
                } else if floors[y][x + 1] == false {
                    // If any floors are to the right, draw this wall short
                    let is_short = (0..y).any(|i| floors[i][x + 1]);
                    out.world
                        .summon_object((x, y).into(), ObjectInfo::WallRight(is_short));
                }
                if x == 0 {
                    out.world
                        .summon_object((x, y).into(), ObjectInfo::WallLeft(true));
                } else if floors[y][x - 1] == false {
                    // Is short if it is on the far left, or there is no "back" wall below it
                    let is_short = (0..y).any(|i| floors[i][x - 1]);
                    out.world
                        .summon_object((x, y).into(), ObjectInfo::WallLeft(is_short));
                }
            }
        }
        out
    }
    fn finish(mut self) -> World {
        self.world.edit_history.clear();
        self.world.move_id = 0;
        self.world.win_state = WinState::Alive;
        self.world
    }
    /// Adds a caption and returns the self
    fn with_caption(mut self, str: &'static str) -> Self {
        self.world.caption = String::from(str);
        self
    }
    /// Adds a hint and returns the self
    fn with_hint(mut self, str: &'static str) -> Self {
        self.world.hint = String::from(str);
        self
    }
    /// Add an object to the world
    fn with_obj(mut self, point: (i32, i32), obj: ObjectInfo) -> Self {
        self.world.summon_object(point.into(), obj);
        self
    }
    /// Add an object to the world with a starting animation
    fn with_obj_anim(mut self, point: (i32, i32), obj: ObjectInfo, anim: i32) -> Self {
        self.world.summon_object(point.into(), obj);
        self.world[point.into()]
            .last_mut()
            .unwrap()
            .animation
            .set(anim);
        self
    }
    /// Set wiring in certain location
    fn with_wiring(mut self, point: (i32, i32), idx: usize, active: bool) -> Self {
        self.world.set_wiring(point.into(), idx, active);
        self
    }
    /// Returns a template world based on the given name
    pub fn get_template(name: &'static str) -> World {
        const T: bool = true;
        const F: bool = false;
        match name {
            "menu1" => Self::make_level(
                5,
                5,
                &[
                    &[T, T, T, T, T],
                    &[T, T, T, F, T],
                    &[T, T, T, F, T],
                    &[T, T, T, F, F],
                    &[T, T, T, T, T],
                ],
                0,
            )
            .with_obj((4, 4), ObjectInfo::Cat)
            .with_obj((0, 4), ObjectInfo::ToggleButton((3, 2).into(), 0))
            .with_obj((4, 1), ObjectInfo::Door(Direction::East, false))
            .with_obj((4, 2), ObjectInfo::Goal)
            .with_obj((2, 2), ObjectInfo::Death)
            .with_obj((0, 0), ObjectInfo::Trap)
            .with_obj((2, 3), ObjectInfo::ToggleableConveyor(Direction::West, true))
            .with_obj((1, 3), ObjectInfo::ToggleableConveyor(Direction::North, true))
            .with_obj((1, 2), ObjectInfo::ToggleableConveyor(Direction::North, true))
            .finish(),
            "menu2" => Self::make_level(
                5,
                5,
                &[
                    &[T, T, T, T, T],
                    &[F, T, T, T, T],
                    &[T, T, T, T, T],
                    &[T, F, F, T, F],
                    &[T, T, T, T, T],
                ],
                0,
            )
            .with_obj((0, 0), ObjectInfo::Cat)
            .with_obj((3, 4), ObjectInfo::Goal)
            .with_obj((4, 4), ObjectInfo::Death)
            .with_obj((1, 1), ObjectInfo::Box)
            .with_obj((3, 3), ObjectInfo::Box)
            .with_obj((1, 2), ObjectInfo::RotateableConveyor(Direction::West, Direction::North, false))
            .with_obj((3, 1), ObjectInfo::PushButton((1,2).into(), 0))
            .finish(),
            "menu3" => Self::make_level(
                5,
                5,
                &[
                    &[T, F, T, T, T],
                    &[T, F, T, F, T],
                    &[T, T, T, F, T],
                    &[T, T, T, F, T],
                    &[F, F, T, F, T],
                ],
                0,
            )
            .with_obj((0, 0), ObjectInfo::Cat)
            .with_obj((2, 4), ObjectInfo::Death)
            .with_obj((0, 2), ObjectInfo::PushButton((2,1).into(), 0))
            .with_obj((2, 1), ObjectInfo::Door(Direction::East, false))
            .with_obj((1, 2), ObjectInfo::Box)
            .with_obj((4, 4), ObjectInfo::Goal)
            .finish(),
            "menu4" => Self::make_level(
                5,
                5,
                &[
                    &[T, T, T, T, T],
                    &[T, F, T, F, T],
                    &[T, F, T, T, T],
                    &[T, F, T, F, T],
                    &[T, T, T, F, T],
                ],
                0,
            )
            .with_obj((1, 4), ObjectInfo::Cat)
            .with_obj((0, 0), ObjectInfo::ToggleButton((4, 3).into(), 0))
            .with_obj((4, 3), ObjectInfo::Door(Direction::East, false))
            .with_obj((2, 2), ObjectInfo::RotateableConveyor(Direction::North,Direction::East, false))
            .with_obj((2, 3), ObjectInfo::ToggleableConveyor(Direction::North, true))
            .with_obj((2, 1), ObjectInfo::ToggleableConveyor(Direction::North, true))
            .with_obj((4, 4), ObjectInfo::Goal)
            .with_obj((3, 2), ObjectInfo::Death)
            .with_obj((0, 4), ObjectInfo::ToggleButton((2,2).into(),0))
            .finish(),
            "Movement" => Self::make_level(
                6,
                3,
                &[
                    &[T, T, T, T, T, T],
                    &[T, T, T, T, T, T],
                    &[T, T, T, T, T, T],
                ],
                1,
            )
            .with_obj((0, 1), ObjectInfo::Goal)
            .with_obj((2, 0), ObjectInfo::Box)
            .with_obj((3, 1), ObjectInfo::Box)
            .with_obj((4, 2), ObjectInfo::Box)
            .with_obj((5, 1), ObjectInfo::Cat)
            .with_caption(
                "It looks like this cat has escaped his box at the Cat Factory! \
                Can you help guide him back? Press the WASD or arrow keys to move.",
            )
            .finish(),
            "Traps" => Self::make_level(
                5,
                3,
                &[&[T, T, T, T, T], &[T, T, T, T, T], &[T, T, T, T, T]],
                1,
            )
            .with_obj((0, 0), ObjectInfo::Goal)
            .with_obj((1, 0), ObjectInfo::Trap)
            .with_obj((1, 1), ObjectInfo::Trap)
            .with_obj((3, 1), ObjectInfo::Death)
            .with_obj((3, 2), ObjectInfo::Death)
            .with_obj((4, 2), ObjectInfo::Cat)
            .with_caption(
                "ACID kills cats immediately and the MOUSE distract cats (so don't get too close!)\
                . Press E to undo or press R to reset the level.",
            )
            .finish(),
            "Buttons" => Self::make_level(
                7,
                4,
                &[
                    &[T, F, T, T, T, T, T],
                    &[T, F, T, T, T, T, T],
                    &[T, F, T, T, T, T, T],
                    &[T, T, T, T, T, T, T],
                ],
                1,
            )
            .with_obj((0, 0), ObjectInfo::Goal)
            .with_obj((0, 1), ObjectInfo::Door(Direction::East, false))
            .with_obj((0, 2), ObjectInfo::Door(Direction::East, false))
            .with_obj((3, 0), ObjectInfo::ToggleButton((0, 1).into(), 0))
            .with_obj((5, 0), ObjectInfo::PushButton((0, 2).into(), 0))
            .with_obj((4, 1), ObjectInfo::Box)
            .with_obj((3, 3), ObjectInfo::Cat)
            .with_caption(
                "Buttons can open doors. Square buttons toggle on and off. \
                Circle buttons must be held down.",
            )
            .finish(),
            "Acid River" => Self::make_level(
                5,
                5,
                &[
                    &[T, T, T, T, T],
                    &[T, T, T, T, T],
                    &[T, T, T, T, T],
                    &[T, T, T, T, T],
                    &[T, T, T, T, T],
                ],
                1,
            )
            .with_obj((0, 4), ObjectInfo::Goal)
            .with_obj((1, 0), ObjectInfo::Death)
            .with_obj((1, 1), ObjectInfo::Death)
            .with_obj((1, 2), ObjectInfo::Door(Direction::North, false))
            .with_obj((0, 3), ObjectInfo::Door(Direction::East, false))
            .with_obj((1, 3), ObjectInfo::Death)
            .with_obj((1, 4), ObjectInfo::Death)
            .with_obj((0, 0), ObjectInfo::PushButton((0, 3).into(), 0))
            .with_obj((0, 1), ObjectInfo::PushButton((1, 2).into(), 0))
            .with_obj((4, 2), ObjectInfo::Cat)
            .with_obj((3, 1), ObjectInfo::Box)
            .with_obj((3, 3), ObjectInfo::Box)
            .finish(),
            "Box Bridge" => Self::make_level(
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
            .with_obj((0, 0), ObjectInfo::Goal)
            .with_obj((0, 1), ObjectInfo::Door(Direction::East, false))
            .with_obj((3, 0), ObjectInfo::PushButton((0, 1).into(), 0))
            .with_obj((3, 1), ObjectInfo::Death)
            .with_obj((3, 2), ObjectInfo::Box)
            .with_obj((1, 2), ObjectInfo::Box)
            .with_obj((4, 2), ObjectInfo::Cat)
            .with_caption("Boxes can be pushed over acid. Try to get back to your box!!")
            .finish(),
            "Pushing My Buttons" => Self::make_level(
                6,
                5,
                &[
                    &[T, T, T, T, T, T],
                    &[T, T, T, F, F, F],
                    &[T, T, T, T, T, T],
                    &[T, T, T, T, F, F],
                    &[T, T, T, T, T, T],
                ],
                1,
            )
            .with_obj((1, 0), ObjectInfo::Cat)
            .with_obj((3, 0), ObjectInfo::Door(Direction::North, false))
            .with_obj((4, 0), ObjectInfo::Door(Direction::North, false))
            .with_obj((4, 2), ObjectInfo::Door(Direction::North, false))
            .with_obj((5, 0), ObjectInfo::Goal)
            .with_obj((5, 4), ObjectInfo::PushButton((3, 0).into(), 0))
            .with_obj((5, 2), ObjectInfo::ToggleButton((4, 0).into(), 0))
            .with_obj((1, 4), ObjectInfo::PushButton((4, 2).into(), 0))
            .with_obj((2, 1), ObjectInfo::Box)
            .with_hint("Square buttons can be toggled")
            .finish(),
            "Conveyor Alley" => Self::make_level(
                6,
                5,
                &[
                    &[T, T, F, F, F, F],
                    &[T, T, F, F, F, F],
                    &[T, T, F, T, F, T],
                    &[T, T, T, T, T, T],
                    &[T, T, F, F, T, F],
                ],
                1,
            )
            .with_obj((1, 1), ObjectInfo::Box)
            .with_obj((0, 2), ObjectInfo::Cat)
            .with_obj(
                (1, 3),
                ObjectInfo::ToggleableConveyor(Direction::East, false),
            )
            .with_obj(
                (2, 3),
                ObjectInfo::ToggleableConveyor(Direction::East, true),
            )
            .with_obj(
                (3, 3),
                ObjectInfo::ToggleableConveyor(Direction::North, false),
            )
            .with_obj(
                (4, 3),
                ObjectInfo::RotateableConveyor(Direction::South, Direction::East, false),
            )
            .with_obj(
                (5, 3),
                ObjectInfo::ToggleableConveyor(Direction::North, true),
            )
            .with_obj((4, 4), ObjectInfo::Death)
            .with_obj((3, 2), ObjectInfo::Death)
            .with_obj((5, 2), ObjectInfo::Goal)
            .with_obj((1, 4), ObjectInfo::PushButton((3, 3).into(), 0))
            .with_obj_anim((1, 0), ObjectInfo::ToggleButton((1, 3).into(), 0), 1)
            .with_obj_anim((0, 0), ObjectInfo::ToggleButton((4, 3).into(), 0), 1)
            .with_wiring((1, 3), 0, true)
            .with_wiring((4, 3), 0, true)
            .with_wiring((3, 3), 1, true)
            .with_caption(
                "Conveyor belts move things! \
                Some toggle on and off; \
                some change directions when activated. \
                Get back to your box without being thrown into acid!",
            )
            .with_hint("Determine which buttons go to which conveyors. Stop them from pushing you into acid.")
            .finish(),
            "One-way Door" => Self::make_level(
                7,
                5,
                &[
                    &[F, T, F, T, T, T, T],
                    &[F, T, F, T, T, T, T],
                    &[T, T, T, T, T, T, T],
                    &[T, T, F, T, T, T, T],
                    &[F, T, F, F, F, F, F],
                ],
                1,
            )
            .with_obj(
                (2, 2),
                ObjectInfo::ToggleableConveyor(Direction::West, false),
            )
            .with_wiring((2, 2), 1, true)
            .with_obj(
                (3, 2),
                ObjectInfo::ToggleableConveyor(Direction::West, false),
            )
            .with_wiring((3, 2), 1, true)
            .with_obj((5, 1), ObjectInfo::Box)
            .with_obj(
                (4, 0),
                ObjectInfo::ToggleableConveyor(Direction::West, false),
            )
            .with_wiring((4, 0), 1, true)
            .with_obj((6, 2), ObjectInfo::ToggleButton((4, 0).into(), 0))
            .with_obj((6, 1), ObjectInfo::Goal)
            .with_obj((5, 0), ObjectInfo::Goal)
            .with_obj((6, 0), ObjectInfo::Death)
            .with_obj((5, 2), ObjectInfo::Cat)
            .with_obj((1, 2), ObjectInfo::Cat)
            .with_obj((1, 0), ObjectInfo::ToggleButton((3, 2).into(), 0))
            .with_obj((1, 0), ObjectInfo::ToggleButton((1, 3).into(), 0))
            .with_obj(
                (1, 1),
                ObjectInfo::ToggleableConveyor(Direction::North, true),
            )
            .with_obj((1, 4), ObjectInfo::ToggleButton((2, 2).into(), 0))
            .with_obj(
                (1, 3),
                ObjectInfo::ToggleableConveyor(Direction::North, false),
            )
            .with_wiring((1, 3), 1, true)
            .with_obj((3, 3), ObjectInfo::Death)
            .with_obj((4, 3), ObjectInfo::Death)
            .with_obj((5, 3), ObjectInfo::Death)
            .with_obj((6, 3), ObjectInfo::Death)
            .finish(),
            "Box Maze" => Self::make_level(
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
            .with_obj((1, 0), ObjectInfo::Goal)
            .with_obj((1, 1), ObjectInfo::Door(Direction::East, false))
            .with_obj((4, 5), ObjectInfo::Door(Direction::North, false))
            .with_obj((3, 1), ObjectInfo::Box)
            .with_obj((3, 5), ObjectInfo::Box)
            .with_obj((6, 4), ObjectInfo::Box)
            .with_obj((6, 5), ObjectInfo::Cat)
            .with_obj((3, 2), ObjectInfo::Death)
            .with_obj((3, 3), ObjectInfo::PushButton((4, 5).into(), 0))
            .with_obj((4, 2), ObjectInfo::PushButton((1, 1).into(), 0))
            .finish(),
            "Conveyance Test" => Self::make_level(
                5,
                5,
                &[
                    &[T, T, T, T, T],
                    &[T, T, T, T, T],
                    &[T, T, T, T, T],
                    &[T, T, T, T, T],
                    &[T, T, T, T, T],
                ],
                1,
            )
            .with_obj(
                (0, 0),
                ObjectInfo::RotateableConveyor(Direction::South, Direction::West, false),
            )
            .with_obj(
                (0, 1),
                ObjectInfo::RotateableConveyor(Direction::South, Direction::East, false),
            )
            .with_obj(
                (0, 2),
                ObjectInfo::RotateableConveyor(Direction::South, Direction::North, false),
            )
            .with_obj(
                (0, 3),
                ObjectInfo::ToggleableConveyor(Direction::South, true),
            )
            .with_obj(
                (0, 4),
                ObjectInfo::ToggleableConveyor(Direction::South, false),
            )
            .with_obj(
                (1, 0),
                ObjectInfo::RotateableConveyor(Direction::East, Direction::North, false),
            )
            .with_obj(
                (1, 1),
                ObjectInfo::RotateableConveyor(Direction::East, Direction::South, false),
            )
            .with_obj(
                (1, 2),
                ObjectInfo::RotateableConveyor(Direction::East, Direction::West, false),
            )
            .with_obj(
                (1, 3),
                ObjectInfo::ToggleableConveyor(Direction::East, true),
            )
            .with_obj(
                (1, 4),
                ObjectInfo::ToggleableConveyor(Direction::East, false),
            )
            .with_obj(
                (2, 0),
                ObjectInfo::RotateableConveyor(Direction::West, Direction::North, false),
            )
            .with_obj(
                (2, 1),
                ObjectInfo::RotateableConveyor(Direction::West, Direction::South, false),
            )
            .with_obj(
                (2, 2),
                ObjectInfo::RotateableConveyor(Direction::West, Direction::East, false),
            )
            .with_obj(
                (2, 3),
                ObjectInfo::ToggleableConveyor(Direction::West, true),
            )
            .with_obj(
                (2, 4),
                ObjectInfo::ToggleableConveyor(Direction::West, false),
            )
            .with_obj(
                (3, 0),
                ObjectInfo::RotateableConveyor(Direction::North, Direction::East, false),
            )
            .with_obj(
                (3, 1),
                ObjectInfo::RotateableConveyor(Direction::North, Direction::West, false),
            )
            .with_obj(
                (3, 2),
                ObjectInfo::RotateableConveyor(Direction::North, Direction::South, false),
            )
            .with_obj(
                (3, 3),
                ObjectInfo::ToggleableConveyor(Direction::North, true),
            )
            .with_obj(
                (3, 4),
                ObjectInfo::ToggleableConveyor(Direction::North, false),
            )
            .with_obj((4, 4), ObjectInfo::Cat)
            .with_obj((4, 2), ObjectInfo::Goal)
            .finish(),
            "Cat Coordination" => Self::make_level(
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
            .with_obj((6, 1), ObjectInfo::Cat)
            .with_obj((5, 0), ObjectInfo::Door(Direction::North, false))
            .with_obj((4, 0), ObjectInfo::Goal)
            .with_obj((4, 2), ObjectInfo::Door(Direction::North, false))
            .with_obj((5, 2), ObjectInfo::Goal)
            .with_obj((1, 0), ObjectInfo::Cat)
            .with_obj((1, 1), ObjectInfo::Box)
            .with_obj((0, 2), ObjectInfo::Box)
            .with_obj((1, 3), ObjectInfo::PushButton((5, 0).into(), 0))
            .with_obj((2, 3), ObjectInfo::PushButton((4, 2).into(), 0))
            .with_caption(
                "Uh oh! It looks like two cats escaped their boxes. \
                They both move in the same direction at your command. \
                Both cats have to be in their boxes to ship them away.",
            )
            .with_hint("Trap the top right kitty in his box")
            .finish(),
            "Parking Lot" => Self::make_level(
                7,
                6,
                &[
                    &[T, T, T, F, T, T, T],
                    &[T, F, T, F, T, T, T],
                    &[T, T, T, T, T, T, T],
                    &[T, T, T, F, T, T, T],
                    &[T, T, T, F, T, F, T],
                    &[T, T, T, F, T, T, F],
                ],
                1,
            )
            .with_obj((1, 3), ObjectInfo::Cat)
            .with_obj((5, 3), ObjectInfo::Cat)
            .with_obj((0, 5), ObjectInfo::Death)
            .with_obj((1, 5), ObjectInfo::Death)
            .with_obj((2, 5), ObjectInfo::Death)
            .with_obj((4, 0), ObjectInfo::Death)
            .with_obj((5, 0), ObjectInfo::Death)
            .with_obj((6, 0), ObjectInfo::Death)
            .with_obj((6, 0), ObjectInfo::Death)
            .with_obj((1, 2), ObjectInfo::Box)
            .with_obj((5, 2), ObjectInfo::Box)
            .with_obj((1, 0), ObjectInfo::Goal)
            .with_obj((3, 2), ObjectInfo::Trap)
            .with_obj((5, 5), ObjectInfo::Goal)
            .with_obj((4, 4), ObjectInfo::Door(Direction::East, false))
            .with_obj((0, 1), ObjectInfo::Door(Direction::East, false))
            .with_obj((2, 1), ObjectInfo::Door(Direction::East, false))
            .with_obj((6, 4), ObjectInfo::ToggleButton((0, 1).into(), 0))
            .with_obj((2, 0), ObjectInfo::ToggleButton((4, 4).into(), 0))
            .with_obj((6, 1), ObjectInfo::PushButton((2, 1).into(), 0))
            .with_hint("You might have to put the kitties on the trap to stop them from moving")
            .finish(),
            "Help Me Out!" => Self::make_level(
                7,
                4,
                &[
                    &[T, T, T, T, T, T, T],
                    &[F, F, T, F, F, F, T],
                    &[T, T, T, F, T, T, T],
                    &[T, T, T, F, T, T, T],
                ],
                1,
            )
            .with_obj((1, 0), ObjectInfo::Door(Direction::North, false))
            .with_obj((4, 0), ObjectInfo::Door(Direction::North, false))
            .with_obj((1, 2), ObjectInfo::Door(Direction::North, false))
            .with_obj((1, 3), ObjectInfo::Door(Direction::North, false))
            .with_obj((0, 0), ObjectInfo::Goal)
            .with_obj((3, 0), ObjectInfo::Goal)
            .with_obj((2, 3), ObjectInfo::Cat)
            .with_obj((4, 3), ObjectInfo::Cat)
            .with_obj((0, 2), ObjectInfo::ToggleButton((1, 0).into(), 0))
            .with_obj((0, 3), ObjectInfo::ToggleButton((4, 0).into(), 0))
            .with_obj((4, 2), ObjectInfo::PushButton((1, 3).into(), 0))
            .with_obj((5, 2), ObjectInfo::PushButton((1, 2).into(), 0))
            .finish(),
            "Pushing My Boxes" => Self::make_level(
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
            .with_obj((0, 1), ObjectInfo::Cat)
            .with_obj((5, 0), ObjectInfo::Cat)
            .with_obj((6, 1), ObjectInfo::Box)
            .with_obj((4, 2), ObjectInfo::Death)
            .with_obj((2, 1), ObjectInfo::Box)
            .with_obj((0, 5), ObjectInfo::Goal)
            .with_obj((2, 3), ObjectInfo::Goal)
            .with_hint("Get the goal to the bottom left corner of the right half")
            .finish(),
            "Playing with Fire" => Self::make_level(7,5,&[
                &[F,T,T,T,T,T,T],
                &[F,T,T,T,T,T,T],
                &[F,T,F,T,T,T,F],
                &[T,T,T,T,T,T,T],
                &[T,T,T,T,T,T,T],
            ],
            2
            )
            .with_obj((1,1), ObjectInfo::Water)
            .with_obj((1,2), ObjectInfo::Door(Direction::East, false))
            .with_obj((0,4), ObjectInfo::Cat)
            .with_obj((5,2), ObjectInfo::Fire)
            .with_obj((2,1), ObjectInfo::Fire)
            .with_obj((5,3), ObjectInfo::Box)
            .with_obj((6,0), ObjectInfo::PushButton((1,2).into(), 0))
            .with_caption("FIRE ALERT! Extinguish both fires with the water bucket before it's too late! \
            Boxes and cats will burn if placed on the fire")
            .finish(),
            "Fire test" => Self::make_level(7,7,&[
                &[T,T,T,T,T,T,T],
                &[T,T,T,T,T,T,T],
                &[T,T,T,T,T,T,T],
                &[T,T,T,T,T,T,T],
                &[T,T,T,T,T,T,T],
                &[T,T,T,T,T,T,T],
                &[T,T,T,T,T,T,T],
            ],
            2)
            .with_obj((0,0), ObjectInfo::Cat)
            .with_obj((3,3),ObjectInfo::Fire)
            .with_obj((2,3),ObjectInfo::Fire)
            .with_obj((3,2),ObjectInfo::Fire)
            .with_obj((4,3),ObjectInfo::Fire)
            .with_obj((3,4),ObjectInfo::Fire)
            .with_obj((1,1), ObjectInfo::Box)
            .with_obj((5,5), ObjectInfo::Water)
            .with_obj((2,2), ObjectInfo::Goal)
            .finish(),
            "one" => Self::make_level(5, 1, &[&[true, true, true, true, true]], 0)
                .with_obj((4, 0), ObjectInfo::Cat)
                .finish(),
            "two" => Self::make_level(
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
            .with_obj((2, 0), ObjectInfo::Cat)
            .finish(),
            "three" => Self::make_level(
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
            .with_obj((0, 1), ObjectInfo::Goal)
            .with_obj((2, 1), ObjectInfo::Cat)
            .with_obj((1, 3), ObjectInfo::Box)
            .with_obj((2, 3), ObjectInfo::Box)
            .with_obj((3, 4), ObjectInfo::Box)
            //((8, 1), ObjectInfo::Cat),
            .with_obj((9, 2), ObjectInfo::Box)
            .with_obj((9, 4), ObjectInfo::Box)
            .with_obj((8, 4), ObjectInfo::Box)
            .finish(),
            "Easy Box" => Self::make_level(
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
            .with_obj((0, 1), ObjectInfo::Goal)
            .with_obj((2, 1), ObjectInfo::Box)
            .with_obj((4, 4), ObjectInfo::Cat)
            .with_obj((2, 2), ObjectInfo::PushButton((1, 1).into(), 0))
            .with_obj((1, 1), ObjectInfo::Door(Direction::North, false))
            .finish(),
            _ => Self::make_level(1, 1, &[&[true]], 0).finish(),
        }
    }
}
