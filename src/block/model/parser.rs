use cgmath::Vector3;
use json::JsonValue;
use json::parse;
use std::collections::HashMap;
use std::env;
use std::fs;

use crate::block::block::Block;
use crate::block::blockface::BlockFace;
use crate::block::blockmodel::BlockModel;

pub fn load() {
    let model = parse(include_str!("../../../assets/minecraft/models/block/cube.json"));
    if let Ok(model) = model {
        println!("{}", model["elements"]);
    }
}

pub fn parse_block_model(file: &str) -> ParsedBlockModel {
    let json_model = parse(&fs::read_to_string("../../../assets/minecraft/models/block/".to_owned() + file).unwrap()).expect(&("could not find file ".to_owned() + file));
    if json_model["parent"] != JsonValue::Null {
        todo!();
    } else {
        let JsonValue::Array(elements) = &json_model["elements"] else { panic!("could not find elements field"); };
        for element in elements {
            let JsonValue::Array(from) = &element["from"] else { panic!(); };
            let JsonValue::Array(to) = &element["to"] else { panic!() };
            let from = Vector3 {
                x: from[0].as_u8().expect("could not find x element of from"),
                y: from[1].as_u8().expect("could not find y element of from"),
                z: from[2].as_u8().expect("could not find z element of from"),
            };
            let to = Vector3 {
                x: to[0].as_u8().expect("could not find x element of to"),
                y: to[1].as_u8().expect("could not find y element of to"),
                z: to[2].as_u8().expect("could not find z element of to"),
            };
            let faces = &element["faces"];
            if let JsonValue::Object(down) = &faces["down"] {
                let JsonValue::String(texture) = &down["texture"] else { panic!("could not find texture")};
            }
        }
    }
    todo!();
}

pub struct ParsedBlockModel {
    pub parent: Option<String>,
    pub ambientocclusion: Option<bool>,
    pub display: Option<Positions>,
    pub textures: Option<HashMap<String, String>>,
    pub elements: Vec<Element>
}

pub struct Positions {
    thirdperson_righthand: Option<Position>,
    thirdperson_lefthand: Option<Position>,
    firstperson_righthand: Option<Position>,
    firstperson_lefthand: Option<Position>,
    gui: Option<Position>,
    head: Option<Position>,
    ground: Option<Position>,
    fixed: Option<Position>
}

pub struct Position {
    rotation: Vector3<f64>,
    translation: Vector3<f64>,
    scale: Vector3<f64>,
}

pub struct Element {
    pub from: Vector3<f64>,
    pub to: Vector3<f64>,
    pub rotation: Option<Rotation>,
    pub shade: Option<bool>,
    pub faces: Faces
}

pub struct Rotation {
    pub origin: Vector3<f64>,
    pub axis: Axis,
    pub angle: f64,
    pub rescale: bool
}

pub enum Axis {
    X,
    Y,
    Z
}

pub struct Faces {
    pub down: Option<Face>
}

pub struct Face {
    pub uv: Option<Uv>,
    pub texture: Texture,
    pub cullface: Option<CullFace>,
    pub rotation: f64,
    pub tintindex: u64
}

pub struct Uv {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

pub enum CullFace {
    Down,
    Up,
    North,
    South,
    West,
    East
}

pub enum Texture {
    Path(String),
    Variable(String)
}