use std::{fmt::Write, str::FromStr};

use crate::{
    levels::{LevelBuilder, WinRequirement},
    object::ObjectInfo,
    util::Point,
};

impl ToString for WinRequirement {
    fn to_string(&self) -> String {
        match self {
            Self::CatsInGoals(num) => format!("cg{}", num),
            Self::FiresExtinguished(num) => format!("fe{}", num),
            Self::MaxMoves(num) => format!("mm{}", num),
            Self::Never => format!("imp"),
        }
    }
}

impl FromStr for WinRequirement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("cg") {
            Ok(Self::CatsInGoals(s[2..].parse().ok().ok_or(())?))
        } else if s.starts_with("fe") {
            Ok(Self::FiresExtinguished(s[2..].parse().ok().ok_or(())?))
        } else if s.starts_with("mm") {
            Ok(Self::MaxMoves(s[2..].parse().ok().ok_or(())?))
        } else {
            Ok(Self::Never)
        }
    }
}

impl ToString for ObjectInfo {
    fn to_string(&self) -> String {
        match self {
            Self::Barrier => "b",
            Self::Box => "box",
            Self::Cat => "c",
            Self::CatCrouch => "cc",
            Self::Goal => "g",
            Self::WallLeft(_) => "",
            Self::WallRight(_) => "",
            Self::WallBack(_) => "",
            Self::WallFront => "",
            Self::PushButton(point, idx) => return format!("pb;{};{}", point, idx),
            Self::ToggleButton(point, idx) => return format!("tb;{};{}", point, idx),
            Self::Door(dir, open) => return format!("d;{};{}", dir.short_code(), open),
            Self::Trap => "t",
            Self::Death => "death",
            Self::ToggleableConveyor(dir, on) => return format!("tc;{};{}", dir.short_code(), on),
            Self::RotateableConveyor(dir, alt, on) => {
                return format!("rc;{};{};{}", dir.short_code(), alt.short_code(), on);
            }
            Self::BurntBox => "bb",
            Self::Fire => "f",
            Self::FireOut => "fo",
            Self::Water => "w",
            Self::Portal(dsts, on, color) => {
                let mut out = String::from("p;");
                for dst in dsts.iter() {
                    out.write_fmt(format_args!("{};", dst)).unwrap();
                }
                out.write_fmt(format_args!("{};{}", on, color)).unwrap();
                return out;
            }
        }
        .to_string()
    }
}

impl FromStr for ObjectInfo {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "b" => Self::Barrier,
            "box" => Self::Box,
            "c" => Self::Cat,
            "cc" => Self::CatCrouch,
            "g" => Self::Goal,
            "t" => Self::Trap,
            "death" => Self::Death,
            "bb" => Self::BurntBox,
            "f" => Self::Fire,
            "fo" => Self::FireOut,
            "w" => Self::Water,
            s => {
                let arg = |i: usize| s.split(";").skip(i).next().ok_or(());
                if s.starts_with("pb;") {
                    Self::PushButton(arg(1)?.parse()?, arg(2)?.parse().ok().ok_or(())?)
                } else if s.starts_with("tb;") {
                    Self::ToggleButton(arg(1)?.parse()?, arg(2)?.parse().ok().ok_or(())?)
                } else if s.starts_with("tc;") {
                    Self::ToggleableConveyor(arg(1)?.parse()?, arg(2)?.parse().ok().ok_or(())?)
                } else if s.starts_with("rc;") {
                    Self::RotateableConveyor(
                        arg(1)?.parse()?,
                        arg(2)?.parse()?,
                        arg(3)?.parse().ok().ok_or(())?,
                    )
                } else if s.starts_with("d;") {
                    Self::Door(arg(1)?.parse()?, arg(2)?.parse().ok().ok_or(())?)
                } else if s.starts_with("p;") {
                    let mut args = vec![];
                    for i in 1.. {
                        if let Ok(s) = arg(i) {
                            args.push(s);
                        } else {
                            break;
                        }
                    }
                    let mut points: Vec<Point> = vec![];
                    for i in 0..(args.len() - 2) {
                        points.push(args[i].parse()?);
                    }
                    Self::Portal(
                        points,
                        args[args.len() - 2].parse().ok().ok_or(())?,
                        args[args.len() - 1].parse().ok().ok_or(())?,
                    )
                } else {
                    return Err(());
                }
            }
        })
    }
}

impl ToString for LevelBuilder {
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.write_fmt(format_args!("{}:{}:", self.world.width, self.world.height))
            .unwrap();
        for j in 0..self.world.height {
            if j != 0 {
                out.write_str(",").unwrap();
            }
            for i in 0..self.world.width {
                if self.world[(i, j).into()]
                    .iter()
                    .any(|v| v.obj_type == ObjectInfo::Barrier)
                {
                    out.write_str("F").unwrap();
                } else {
                    out.write_str("T").unwrap();
                }
            }
        }
        out.write_str(":").unwrap();
        let mut first = true;
        for (i, wire) in self.world.wiring.iter().enumerate() {
            if wire.iter().any(|v| *v) {
                let point: Point = (i % self.world.width, i / self.world.width).into();
                if first {
                    out.write_str("&").unwrap();
                    first = false;
                }
                out.write_fmt(format_args!(
                    "{};{};{};{};{}",
                    point, wire[0], wire[1], wire[2], wire[3]
                ))
                .unwrap();
            }
        }
        out.write_str(":").unwrap();
        for i in 0..self.world.requirements.len() {
            out.write_fmt(format_args!("{}", self.world.requirements[i].to_string()))
                .unwrap();
            if i != self.world.requirements.len() - 1 {
                out.write_str(",").unwrap();
            }
        }
        out.write_str(":").unwrap();
        let mut first = true;
        for j in 0..self.world.height {
            for i in 0..self.world.width {
                for obj in self.world[(i, j).into()].iter() {
                    if match obj.obj_type {
                        ObjectInfo::Barrier => false,
                        ObjectInfo::WallBack(_) => false,
                        ObjectInfo::WallRight(_) => false,
                        ObjectInfo::WallLeft(_) => false,
                        ObjectInfo::WallFront => false,
                        _ => true,
                    } {
                        if first {
                            out.write_str("&").unwrap();
                            first = false;
                        }
                        out.write_fmt(format_args!(
                            "{};{}",
                            obj.position.to_string(),
                            obj.obj_type.to_string()
                        ))
                        .unwrap();
                    }
                }
            }
        }

        out
    }
}
impl FromStr for LevelBuilder {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon_section = |arg: usize| {
            s.split(":")
                .skip(arg)
                .next()
                .ok_or_else(|| String::from("Invalid encoding"))
        };
        // Flooring
        let width: usize = colon_section(0)?
            .parse::<usize>()
            .ok()
            .ok_or_else(|| String::from("Invalid width"))?;
        let height: usize = colon_section(1)?
            .parse::<usize>()
            .ok()
            .ok_or_else(|| String::from("Invalid width"))?;
        let floor = colon_section(2)?;
        if floor.chars().filter(|v| *v != ',').count() != width * height {
            return Err(String::from("Invalid floor size"));
        }
        let mut floor_vec: Vec<Vec<bool>> = vec![];
        let mut index = 0;
        for j in 0..height {
            floor_vec.push(vec![]);
            for _ in 0..width {
                while floor.as_bytes()[index] == b',' {
                    index += 1;
                }
                floor_vec[j].push(floor.as_bytes()[index] == b'T');
                index += 1;
            }
        }
        // Wires
        let mut wires: Vec<(Point, [bool; 4])> = vec![];
        for wire in colon_section(3)?.split("&") {
            if wire.len() == 0 {
                continue;
            }
            let point: Point = wire
                .split(";")
                .next()
                .ok_or_else(|| String::from("Wire encoding error"))?
                .parse()
                .ok()
                .ok_or_else(|| String::from("Wire point encoding error"))?;
            let wire = wire
                .split(";")
                .skip(1)
                .next()
                .ok_or_else(|| String::from("Wire encoding error"))?
                .as_bytes();
            if wire.len() != 4 {
                return Err(String::from("Wire encoding error"));
            }
            let wire = [
                wire[0] == b'T',
                wire[1] == b'T',
                wire[2] == b'T',
                wire[3] == b'T',
            ];
            wires.push((point, wire));
        }
        turbo::log!("Wires: {:?}", wires);
        // Requirements
        let requirements: Vec<WinRequirement> = colon_section(4)?
            .split(",")
            .map(|v| {
                v.parse::<WinRequirement>()
                    .ok()
                    .ok_or_else(|| String::from("Invalid requirement"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        turbo::log!("Requirements: {:?}", requirements);
        // Objects
        let mut objects: Vec<(Point, ObjectInfo)> = vec![];
        for obj in colon_section(5)?.split("&") {
            if obj.len() == 0 {
                continue;
            }
            let semicolon = obj.find(';').ok_or_else(|| String::from("No semicolon"))?;
            let point: Point = obj[0..semicolon]
                .parse()
                .ok()
                .ok_or_else(|| String::from("Invalid point"))?;
            let obj: ObjectInfo = obj[semicolon + 1..]
                .parse()
                .ok()
                .ok_or_else(|| format!("Invalid object {}", obj))?;
            objects.push((point, obj));
        }
        turbo::log!("Objects: {:?}", objects);

        // Check integrity
        if requirements.len() == 0 {
            return Err(String::from("World must have win requirement"));
        }
        let mut builder = Self::make_level(
            width,
            height,
            floor_vec
                .iter()
                .map(|v| v.as_slice())
                .collect::<Vec<&[bool]>>()
                .as_slice(),
            requirements[0],
        );
        for wire in wires.iter() {
            if !builder.world.point_inside(wire.0) {
                return Err(format!("{} is outside the world", wire.0));
            }
            for i in 0..4 {
                if wire.1[i] {
                    builder = builder.with_wiring(wire.0.into(), i, true);
                }
            }
        }
        for object in objects.iter() {
            if !builder.world.point_inside(object.0) {
                return Err(format!("{} is outside the world", object.0));
            }
            builder = builder.with_obj(object.0.into(), object.1.clone());
        }
        Ok(builder)
    }
}
