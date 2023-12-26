use std::path::Path;

use ggez::{glam::IVec2, GameResult};

use super::{
    instances::{
        collectible::{CollectibleType, Collectible},
        floor::{FloorType, Floor},
        object::{Object, ObjectType},
        wall::{WallData, WallType, WallOrientation},
        ActivatingColor, LayerData,
    },
    level_data::LevelData,
};

fn write_line(
    content: &mut String,
    name: impl AsRef<str>,
    pos: &IVec2,
    suffix: impl AsRef<str>,
    properties: &[(impl AsRef<str>, impl AsRef<str>)],
) {
    content.push_str(name.as_ref());
    content.push(' ');

    content.push_str(&pos.x.to_string());
    content.push(',');
    content.push_str(&pos.y.to_string());
    content.push(' ');

    content.push_str(suffix.as_ref());

    for (key, val) in properties {
        content.push(' ');
        content.push_str(key.as_ref());
        content.push(':');
        content.push_str(val.as_ref());
    }

    content.push('\n');
}

pub fn write_wall(
    contents: &mut String,
    pos: &IVec2,
    wall_data: &WallData,
    suffix: impl AsRef<str>,
) {
    let name = match wall_data.wall_type {
        WallType::Normal => "wall",
    };

    let properties = vec![
        ("c", wall_data.color.to_string()),
        ("closed", (!wall_data.opened).to_string()),
        ("inputbased", wall_data.input_dependent.to_string()),
    ];

    write_line(contents, name, pos, suffix, &properties);
}

pub fn save(level_data: &LevelData, path: &Path) -> GameResult {
    let mut contents = String::new();
    let mut dimentions = IVec2::new(0, 0);

    for (pos, floor) in &level_data.floors {
        if pos.x > dimentions.x {
            dimentions.x = pos.x;
        }
        if pos.y > dimentions.y {
            dimentions.y = pos.y;
        }

        let name = match floor.floor_type {
            FloorType::Normal => "floor",
            FloorType::Button => "button",
            FloorType::Teleport => "teleport",
        };

        let suffix = "";

        let properties = vec![
            ("dur", floor.durability.to_string()),
            ("c", floor.color.to_string()),
        ];

        write_line(&mut contents, name, pos, suffix, &properties);
    }

    contents.push('\n');
    contents.push('\n');

    for (pos, object) in &level_data.objects {
        if pos.x > dimentions.x {
            dimentions.x = pos.x;
        }
        if pos.y > dimentions.y {
            dimentions.y = pos.y;
        }

        let name = match object.object_type {
            ObjectType::Player => "player",
            ObjectType::Box => "box",
            ObjectType::TeleBox => "telebox",
        };

        let suffix = "";

        let properties = vec![("c", object.color.to_string())];

        write_line(&mut contents, name, pos, suffix, &properties);
    }

    contents.push('\n');
    contents.push('\n');

    for (pos, collectible) in &level_data.collectibles {
        if pos.x > dimentions.x {
            dimentions.x = pos.x;
        }
        if pos.y > dimentions.y {
            dimentions.y = pos.y;
        }

        let name = match collectible.collectible_type {
            CollectibleType::Win => "win",
        };

        let suffix = "";

        let properties: Vec<(&str, &str)> = vec![];

        write_line(&mut contents, name, pos, suffix, &properties);
    }

    contents.push('\n');
    contents.push('\n');

    for (pos, wall) in &level_data.walls {
        if pos.x > dimentions.x {
            dimentions.x = pos.x;
        }
        if pos.y > dimentions.y {
            dimentions.y = pos.y;
        }

        if let Some(wall_data) = &wall.down {
            write_wall(&mut contents, pos, wall_data, "d");
        }

        if let Some(wall_data) = &wall.right {
            write_wall(&mut contents, pos, wall_data, "r");
        }
    }

    contents.insert_str(0, &format!("{},{}\n", dimentions.x, dimentions.y));

    std::fs::write(path, contents)?;

    Ok(())
}


pub fn load(path: &Path) -> GameResult<LevelData> {
    let mut level_data = LevelData::new();

    let contents = std::fs::read_to_string(path)?;

    let mut lines = contents.lines();
    lines.next();

    for line in lines {
        if let Some(0) = line.find('#') {
            continue;
        }

        let mut properties = line.split(' ');

        let Some(name) = properties.next() else {
            continue;
        };

        let mut name = match name {
            "player" => LayerData::Object(Object::default(ObjectType::Player)),
            "box" => LayerData::Object(Object::default(ObjectType::Box)),
            "telebox" => LayerData::Object(Object::default(ObjectType::TeleBox)),
            
            "floor" => LayerData::Floor(Floor::default(FloorType::Normal)),
            "button" => LayerData::Floor(Floor::default(FloorType::Button)),
            "teleport" => LayerData::Floor(Floor::default(FloorType::Teleport)),

            "wall" => LayerData::Wall(WallData::default(WallType::Normal)),

            "win" => LayerData::Collectible(Collectible::default(CollectibleType::Win)),

            _ => continue,
        };

        let Some(pos) = properties.next() else {
            continue;
        };
        let Some(pos) = pos.split_once(',') else {continue;};
        let (Ok(pos_x), Ok(pos_y)) = (pos.0.parse(), pos.1.parse()) else {continue;};

        let pos = IVec2 { x: pos_x, y: pos_y };

        let mut orientation = WallOrientation::Down;

        if let LayerData::Wall(_) = name {
            let Some(s) = properties.next() else {continue;};
            if s == "r" {
                orientation = WallOrientation::Right;
            }
        }


        match name {
            LayerData::Object(ref mut object) => init_object(object, properties),
            LayerData::Floor(ref mut floor) => init_floor(floor, properties),
            LayerData::Wall(ref mut wall) => init_wall(wall, properties),
            LayerData::Collectible(ref mut collectible) => init_collectible(collectible, properties),
        }
        
        level_data.insert(pos, name, orientation);
    }

    Ok(level_data)
}

fn parse_color(color: &str) -> ActivatingColor {
    match color {
        "r" => ActivatingColor::Red,
        "g" => ActivatingColor::Green,
        "b" => ActivatingColor::Blue,
        "y" => ActivatingColor::Yellow,
        "c" => ActivatingColor::Cyan,
        "p" => ActivatingColor::Pink,

        _ => ActivatingColor::None,
    }
}

fn parse_bool(b: &str) -> bool{
    b == "true"
}

fn init_object<'a>(object: &mut Object, props: impl std::iter::Iterator<Item = &'a str>) {
     for prop in props {
         let Some((word, val)) = prop.split_once(',') else {continue;};
         match word {
             "c" => object.color = parse_color(val),

             _ => (),
         }
     }
}


fn init_wall<'a>(wall: &mut WallData, props: impl std::iter::Iterator<Item = &'a str>) {
     for prop in props {
         let Some((word, val)) = prop.split_once(',') else {continue;};
         match word {
             "c" => wall.color = parse_color(val),
             "closed" => wall.opened = parse_bool(val),
             "inputbased" => wall.input_dependent = parse_bool(val),
            
             _ => (),
         }
     }
}

fn init_floor<'a>(floor: &mut Floor, props: impl std::iter::Iterator<Item = &'a str>) {
     for prop in props {
         let Some((word, val)) = prop.split_once(',') else {continue;};
         match word {
             "c" => floor.color = parse_color(val),
             "dur" => floor.durability = if let Ok(ival) = val.parse() {ival} else {continue;},
            
             _ => (),
         }
     }
}

fn init_collectible<'a>(collectible: &mut Collectible, props: impl std::iter::Iterator<Item = &'a str>) {
     for prop in props {
         let Some((word, val)) = prop.split_once(',') else {continue;};
         match word {
             _ => (),
         }
     }
}



