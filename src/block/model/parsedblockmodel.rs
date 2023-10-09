use cgmath::Vector3;
use cgmath::Zero;
use glfw::Error;
use json::JsonValue;
use json::object;
use json::object::Object;
use json::parse;
use std::collections::HashMap;
use std::env;
use std::fs;

use crate::block::block::Block;
use crate::block::blockface::BlockFace;
use crate::block::blockmodel::BlockModel;
use crate::block::model::deserialize::JsonDeserializeError::*;
use crate::block::model::deserialize::MaybeDeserialize;
use super::deserialize::Deserialize;
use super::deserialize::JsonDeserializeError;
use super::deserialize::JsonType;

#[derive(Debug)]
#[derive(Clone)]
pub struct ParsedBlockModel {
    pub parent: Option<String>,
    pub ambientocclusion: Option<bool>,
    pub display: Option<Display>,
    pub textures: Option<HashMap<String, Texture>>,
    pub elements: Option<Vec<Element>>
}

impl ParsedBlockModel {
    pub fn from_json(json: &JsonValue) -> Result<ParsedBlockModel, JsonDeserializeError> {
        return ParsedBlockModel::deserialize(json);
    }
}

impl Deserialize for ParsedBlockModel {
    fn deserialize(value: &JsonValue) -> Result<Self, super::deserialize::JsonDeserializeError> { unsafe {
        if let JsonValue::Object(object) = value {
            let parent = String::maybe_deserialize_field(object, "parent");
            let ambientocclusion = bool::maybe_deserialize_field(object, "ambientocclusion");
            let display = Display::maybe_deserialize_field(object, "display");
            let textures = HashMap::maybe_deserialize_field(object, "textures");
            let elements = Vec::<Element>::maybe_deserialize_field(object, "elements");

            let Ok(parent) = parent else { return Err(parent.unwrap_err_unchecked()); };
            let Ok(ambientocclusion) = ambientocclusion else { return Err(ambientocclusion.unwrap_err_unchecked()); };
            let Ok(display) = display else { return Err(display.unwrap_err_unchecked()); };
            let Ok(textures) = textures else { return Err(textures.unwrap_err_unchecked()); };
            let Ok(elements) = elements else { return Err(elements.unwrap_err_unchecked()); };
            
            return Ok(ParsedBlockModel {
                parent: parent,
                ambientocclusion: ambientocclusion,
                display: display,
                textures: textures,
                elements: elements,
            });
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    } }
}

#[derive(Default, Debug, Clone)]
pub struct Display {
    thirdperson_righthand: Option<Position>,
    thirdperson_lefthand: Option<Position>,
    firstperson_righthand: Option<Position>,
    firstperson_lefthand: Option<Position>,
    gui: Option<Position>,
    head: Option<Position>,
    ground: Option<Position>,
    fixed: Option<Position>
}

impl Deserialize for Display {
    fn deserialize(value: &JsonValue) -> Result<Self, super::deserialize::JsonDeserializeError> { unsafe {
        if let JsonValue::Object(object) = value {
            let thirdperson_righthand = Position::maybe_deserialize_field(object, "thirdperson_righthand");
            let thirdperson_lefthand = Position::maybe_deserialize_field(object, "thirdperson_lefthand");
            let firstperson_righthand = Position::maybe_deserialize_field(object, "firstperson_righthand");
            let firstperson_lefthand = Position::maybe_deserialize_field(object, "firstperson_lefthand");
            let gui = Position::maybe_deserialize_field(object, "gui");
            let head = Position::maybe_deserialize_field(object, "head");
            let ground = Position::maybe_deserialize_field(object, "ground");
            let fixed = Position::maybe_deserialize_field(object, "fixed");

            let Ok(thirdperson_righthand) = thirdperson_righthand else { return Err(thirdperson_righthand.unwrap_err_unchecked()); };
            let Ok(thirdperson_lefthand) = thirdperson_lefthand else { return Err(thirdperson_lefthand.unwrap_err_unchecked()); };
            let Ok(firstperson_righthand) = firstperson_righthand else { return Err(firstperson_righthand.unwrap_err_unchecked()); };
            let Ok(firstperson_lefthand) = firstperson_lefthand else { return Err(firstperson_lefthand.unwrap_err_unchecked()); };
            let Ok(gui) = gui else { return Err(gui.unwrap_err_unchecked()); };
            let Ok(head) = head else { return Err(head.unwrap_err_unchecked()); };
            let Ok(ground) = ground else { return Err(ground.unwrap_err_unchecked()); };
            let Ok(fixed) = fixed else { return Err(fixed.unwrap_err_unchecked()); };
            
            return Ok(Display {
                thirdperson_righthand: thirdperson_righthand,
                thirdperson_lefthand: thirdperson_lefthand,
                firstperson_righthand: firstperson_righthand,
                firstperson_lefthand: firstperson_lefthand,
                gui: gui,
                head: head,
                ground: ground,
                fixed: fixed,
            });
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    } }
}

#[derive(Debug, Clone)]
pub struct Position {
    rotation: Vector3<f64>,
    translation: Vector3<f64>,
    scale: Vector3<f64>,
}

impl Deserialize for Position {
    fn deserialize(value: &JsonValue) -> Result<Self, super::deserialize::JsonDeserializeError> { unsafe {
        if let JsonValue::Object(object) = value {
            let rotation = Vector3::deserialize_field(object, "rotation");
            let translation = Vector3::deserialize_field(object, "translation");
            let scale = Vector3::deserialize_field(object, "scale");

            let Ok(rotation) = rotation else { return Err(rotation.unwrap_err_unchecked()); };
            let Ok(translation) = translation else { return Err(translation.unwrap_err_unchecked()); };
            let Ok(scale) = scale else { return Err(scale.unwrap_err_unchecked()); };

            return Ok(Position {
                rotation: rotation,
                translation: translation,
                scale: scale,
            });
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    } }
}

#[derive(Debug, Clone)]
pub struct Element {
    pub from: [f64; 3],
    pub to: [f64; 3],
    pub rotation: Option<Rotation>,
    pub shade: Option<bool>,
    pub faces: Faces
}

impl Deserialize for Element {
    fn deserialize(value: &JsonValue) -> Result<Self, super::deserialize::JsonDeserializeError> { unsafe {
        if let JsonValue::Object(object) = value {
            let from = <[f64; 3]>::deserialize_field(object, "from");
            let to = <[f64; 3]>::deserialize_field(object, "to");
            let rotation = Rotation::maybe_deserialize_field(object, "rotation");
            let shade = bool::maybe_deserialize_field(object, "shade");
            let faces = Faces::deserialize_field(object, "faces");

            let Ok(from) = from else { return Err(from.unwrap_err_unchecked()); };
            let Ok(to) = to else { return Err(to.unwrap_err_unchecked()); };
            let Ok(rotation) = rotation else { return Err(rotation.unwrap_err_unchecked()); };
            let Ok(shade) = shade else { return Err(shade.unwrap_err_unchecked()); };
            let Ok(faces) = faces else { return Err(faces.unwrap_err_unchecked()); };

            return Ok(Element {
                from: from,
                to: to,
                rotation: rotation,
                shade: shade,
                faces: faces,
            });
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    } }
}

#[derive(Debug, Clone)]
pub struct Rotation {
    pub origin: Vector3<f64>,
    pub axis: Axis,
    pub angle: f64,
    pub rescale: bool
}

impl Deserialize for Rotation {
    fn deserialize(value: &JsonValue) -> Result<Self, JsonDeserializeError> { unsafe {
        if let JsonValue::Object(object) = value {
            let origin = Vector3::deserialize_field(object, "origin");
            let axis = Axis::deserialize_field(object, "axis");
            let angle = f64::deserialize_field(object, "angle");
            let rescale = bool::deserialize_field(object, "rescale");

            let Ok(origin) = origin else { return Err(origin.unwrap_err_unchecked()); };
            let Ok(axis) = axis else { return Err(axis.unwrap_err_unchecked()); };
            let Ok(angle) = angle else { return Err(angle.unwrap_err_unchecked()); };
            let Ok(rescale) = rescale else { return Err(rescale.unwrap_err_unchecked()); };

            return Ok(Rotation {
                origin: origin,
                axis: axis,
                angle: angle,
                rescale: rescale,
            });
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    } }
}

#[derive(Debug, Clone)]
pub enum Axis {
    X,
    Y,
    Z
}

impl Deserialize for Axis {
    fn deserialize(value: &JsonValue) -> Result<Self, super::deserialize::JsonDeserializeError> {
        if let JsonValue::String(string) = value {
            match string.as_str() {
                "x" => { return Ok(Axis::X); }
                "y" => { return Ok(Axis::Y); }
                "z" => { return Ok(Axis::Z); }
                string => { return Err(BadStringEnum(string.to_owned())); }
            }
        } else if let JsonValue::Short(short) = value {
            match short.as_str() {
                "x" => { return Ok(Axis::X); }
                "y" => { return Ok(Axis::Y); }
                "z" => { return Ok(Axis::Z); }
                string => { return Err(BadStringEnum(string.to_owned())); }
            }
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    }
}

#[derive(Debug, Clone)]
pub struct Faces {
    pub down: Option<Face>,
    pub up: Option<Face>,
    pub north: Option<Face>,
    pub south: Option<Face>,
    pub east: Option<Face>,
    pub west: Option<Face>,
}

impl Deserialize for Faces {
    fn deserialize(value: &JsonValue) -> Result<Self, JsonDeserializeError> { unsafe {
        if let JsonValue::Object(object) = value {
            let down = Face::maybe_deserialize_field(object, "down");
            let up = Face::maybe_deserialize_field(object, "up");
            let north = Face::maybe_deserialize_field(object, "north");
            let south = Face::maybe_deserialize_field(object, "south");
            let east = Face::maybe_deserialize_field(object, "east");
            let west = Face::maybe_deserialize_field(object, "west");

            let Ok(down) = down else { return Err(down.unwrap_err_unchecked()); };
            let Ok(up) = up else { return Err(up.unwrap_err_unchecked()); };
            let Ok(north) = north else { return Err(north.unwrap_err_unchecked()); };
            let Ok(south) = south else { return Err(south.unwrap_err_unchecked()); };
            let Ok(east) = east else { return Err(east.unwrap_err_unchecked()); };
            let Ok(west) = west else { return Err(west.unwrap_err_unchecked()); };

            return Ok(Faces {
                down: down,
                up: up,
                north: north,
                south: south,
                east: east,
                west: west,
            });
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    } }
}

#[derive(Debug, Clone)]
pub struct Face {
    pub uv: Option<[f64; 4]>,
    pub texture: Texture,
    pub cullface: Option<CullFace>,
    pub rotation: Option<f64>,
    pub tintindex: Option<u64>
}

impl Deserialize for Face {
    fn deserialize(value: &JsonValue) -> Result<Self, JsonDeserializeError> { unsafe {
        if let JsonValue::Object(object) = value {
            let uv = <[f64; 4]>::maybe_deserialize_field(object, "uv");
            let texture = Texture::deserialize_field(object, "texture");
            let cullface = CullFace::maybe_deserialize_field(object, "cullface");
            let rotation = f64::maybe_deserialize_field(object, "rotation");
            let tintindex = u64::maybe_deserialize_field(object, "tintindex");

            let Ok(uv) = uv else { return Err(uv.unwrap_err_unchecked()); };
            let Ok(texture) = texture else { return Err(texture.unwrap_err_unchecked()); };
            let Ok(cullface) = cullface else { return Err(cullface.unwrap_err_unchecked()); };
            let Ok(rotation) = rotation else { return Err(rotation.unwrap_err_unchecked()); };
            let Ok(tintindex) = tintindex else { return Err(tintindex.unwrap_err_unchecked()); };

            return Ok(Face {
                uv: uv,
                texture: texture,
                cullface: cullface,
                rotation: rotation,
                tintindex: tintindex,
            });
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    } }
}

#[derive(Debug, Clone)]
pub enum CullFace {
    Down,
    Up,
    North,
    South,
    West,
    East
}

impl Deserialize for CullFace {
    fn deserialize(value: &JsonValue) -> Result<Self, super::deserialize::JsonDeserializeError> {
        if let JsonValue::String(string) = value {
            match string.as_str() {
                "down" => { return Ok(CullFace::Down); }
                "up" => { return Ok(CullFace::Up); }
                "north" => { return Ok(CullFace::North); }
                "south" => { return Ok(CullFace::South); }
                "west" => { return Ok(CullFace::West); }
                "east" => { return Ok(CullFace::East); }
                string => { return Err(BadStringEnum(string.to_owned())); }
            }
        } else if let JsonValue::Short(short) = value {
            match short.as_str() {
                "down" => { return Ok(CullFace::Down); }
                "up" => { return Ok(CullFace::Up); }
                "north" => { return Ok(CullFace::North); }
                "south" => { return Ok(CullFace::South); }
                "west" => { return Ok(CullFace::West); }
                "east" => { return Ok(CullFace::East); }
                string => { return Err(BadStringEnum(string.to_owned())); }
            }
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    }
}

#[derive(Debug, Clone)]
pub enum Texture {
    Path(String),
    Variable(String)
}

impl Deserialize for Texture {
    fn deserialize(value: &JsonValue) -> Result<Self, JsonDeserializeError> {
        if let JsonValue::String(string) = value {
            if string.starts_with('#') {
                return Ok(Texture::Variable(string.clone()));
            } else {
                return Ok(Texture::Path(string.clone()));
            }
        } else if let JsonValue::Short(short) = value {
            let string = short.to_string();
            if string.starts_with('#') {
                return Ok(Texture::Variable(string.clone()));
            } else {
                return Ok(Texture::Path(string.clone()));
            }
        } else {
            return Err(WrongType(JsonType::get_type(value)));
        }
    }
}