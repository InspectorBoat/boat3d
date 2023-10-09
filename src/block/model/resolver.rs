use std::fs;

use cgmath::{Vector3, Vector2};

use crate::block::model::parsedblockmodel::Texture;

use super::parsedblockmodel::{ParsedBlockModel, Rotation};

pub struct ResolvedBlockModel {
    pub elements: Vec<Element>
}

impl ResolvedBlockModel {
    pub fn resolve(model: ParsedBlockModel) -> ResolvedBlockModel { unsafe {
        if model.elements.is_none() && model.parent.is_none() {
            panic!("no elements and no parent");
        }
        let mut resolved_model = ResolvedBlockModel { elements: Vec::new() };
        // model has parent
        if model.parent.is_some() {
            let models = ResolvedBlockModel::find_parents(&model);
            let elements = models.last().unwrap_unchecked().elements.as_ref().expect("missing elements in base parent model");
            for element in elements {
                let from = Vector3::from(element.from);
                let to = Vector3::from(element.to);
                let mut faces = Faces {
                    down: None,
                    up: None,
                    north: None,
                    south: None,
                    east: None,
                    west: None,
                };
                if let Some(down) = &element.faces.down {
                    let mut texture = &down.texture;
                    let mut furthest_index = models.len() - 1;
                    while let Texture::Variable(variable) = texture {
                        // find binding for that variable, child overriding parent
                        for i in 0..furthest_index {
                            let mut found_match = false;
                            models[i].textures.as_ref().inspect(|textures| {
                                if let Some(binding) = textures.get(variable) {
                                    
                                    texture = binding;
                                    found_match = true;
                                }
                            });
                        }
                    };
                    // faces.down = Some(Face { uv: None, texture: path.to_string(), rotation: down.rotation.unwrap_or(0.0) });
                    if let Some(uv) = down.uv {
                        faces.down.unwrap().uv = Some(Uv {
                            min: Vector2 { x: uv[0], y: uv[1] },
                            max: Vector2 { x: uv[2], y: uv[3] },
                        });
                    }
                }
                resolved_model.elements.push(Element {
                    from: from,
                    to: to,
                    rotation: element.rotation.clone(),
                    shade: element.shade.unwrap_or(true),
                    faces: todo!(),
                })
            }
        }
        // model has no parent
        else {
            let Some(elements) = model.elements else { panic!("missing elements in model"); };
            for element in elements {
                let from = Vector3::from(element.from);
                let to = Vector3::from(element.to);
                let mut faces = Faces {
                    down: None,
                    up: None,
                    north: None,
                    south: None,
                    east: None,
                    west: None,
                };
                if let Some(down) = &element.faces.down {
                    let Texture::Path(path) = &down.texture else { panic!("found variable instead of path"); };
                    faces.down = Some(Face { uv: None, texture: path.to_string(), rotation: down.rotation.unwrap_or(0.0) });
                    if let Some(uv) = down.uv {
                        faces.down.unwrap().uv = Some(Uv {
                            min: Vector2 { x: uv[0], y: uv[1] },
                            max: Vector2 { x: uv[2], y: uv[3] },
                        });
                    }
                }
                if let Some(up) = &element.faces.up {
                    let Texture::Path(path) = &up.texture else { panic!("found variable instead of path"); };
                    faces.up = Some(Face { uv: None, texture: path.to_string(), rotation: up.rotation.unwrap_or(0.0) });
                    if let Some(uv) = up.uv {
                        faces.up.unwrap().uv = Some(Uv {
                            min: Vector2 { x: uv[0], y: uv[1] },
                            max: Vector2 { x: uv[2], y: uv[3] },
                        });
                    }
                }
                if let Some(north) = &element.faces.north {
                    let Texture::Path(path) = &north.texture else { panic!("found variable instead of path"); };
                    faces.north = Some(Face { uv: None, texture: path.to_string(), rotation: north.rotation.unwrap_or(0.0) });
                    if let Some(uv) = north.uv {
                        faces.north.unwrap().uv = Some(Uv {
                            min: Vector2 { x: uv[0], y: uv[1] },
                            max: Vector2 { x: uv[2], y: uv[3] },
                        });
                    }
                }
                if let Some(south) = &element.faces.south {
                    let Texture::Path(path) = &south.texture else { panic!("found variable instead of path"); };
                    faces.south = Some(Face { uv: None, texture: path.to_string(), rotation: south.rotation.unwrap_or(0.0) });
                    if let Some(uv) = south.uv {
                        faces.south.unwrap().uv = Some(Uv {
                            min: Vector2 { x: uv[0], y: uv[1] },
                            max: Vector2 { x: uv[2], y: uv[3] },
                        });
                    }
                }
                if let Some(east) = &element.faces.east {
                    let Texture::Path(path) = &east.texture else { panic!("found variable instead of path"); };
                    faces.east = Some(Face { uv: None, texture: path.to_string(), rotation: east.rotation.unwrap_or(0.0) });
                    if let Some(uv) = east.uv {
                        faces.east.unwrap().uv = Some(Uv {
                            min: Vector2 { x: uv[0], y: uv[1] },
                            max: Vector2 { x: uv[2], y: uv[3] },
                        });
                    }
                }
                if let Some(west) = &element.faces.west {
                    let Texture::Path(path) = &west.texture else { panic!("found variable instead of path"); };
                    faces.west = Some(Face { uv: None, texture: path.to_string(), rotation: west.rotation.unwrap_or(0.0) });
                    if let Some(uv) = west.uv {
                        faces.west.unwrap().uv = Some(Uv {
                            min: Vector2 { x: uv[0], y: uv[1] },
                            max: Vector2 { x: uv[2], y: uv[3] },
                        });
                    }
                }
                resolved_model.elements.push(Element {
                    from: from,
                    to: to,
                    rotation: element.rotation,
                    shade: element.shade.unwrap_or(true),
                    faces: todo!(),
                })
            }
        }
        todo!();
    } }
    
    pub fn find_parents(model: &ParsedBlockModel) -> Vec<ParsedBlockModel> { unsafe {
        let mut models: Vec<ParsedBlockModel> = Vec::new();
        models.push(model.clone());
        while let Some(parent) = &models.last().unwrap_unchecked().parent {
            let parent = fs::read_to_string(&("C:/Users/inspe/Documents/Code/boat3d/assets/minecraft/models".to_string() + &parent)).expect("failed to read file");
            let parent = json::parse(&parent).expect("failed to parse parent");
            if let Ok(parent) = ParsedBlockModel::from_json(&parent) {
                models.push(parent);
            }
        }
        return models;
    } }
}

pub struct Element {
    pub from: Vector3<f64>,
    pub to: Vector3<f64>,
    pub rotation: Option<Rotation>,
    pub shade: bool,
    pub faces: Faces
}

#[derive(Debug)]
pub struct Faces {
    pub down: Option<Face>,
    pub up: Option<Face>,
    pub north: Option<Face>,
    pub south: Option<Face>,
    pub east: Option<Face>,
    pub west: Option<Face>,
}

#[derive(Debug)]
pub struct Face {
    pub uv: Option<Uv>,
    pub texture: String,
    pub rotation: f64,
}

#[derive(Debug)]
pub struct Uv {
    pub min: Vector2<f64>,
    pub max: Vector2<f64>,
}