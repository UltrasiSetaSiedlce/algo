use std::collections::HashMap;

use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PackingArea {
    pub boxes: Vec<Box>,
    pub palett_size: (usize, usize, usize),
    pub palettes_n: usize,
}

#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Box {
    pub id: usize,
    pub dx: usize,
    pub dz: usize,
    pub weight: usize,
}

impl Box {
    pub fn new(id: usize, width: usize, height: usize) -> Box {
        Box {
            id,
            dx: width,
            dz: height,
            weight: 5,
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PackingPlan {
    pub palettes: Vec<FilledPalett>
}

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct FilledPalett {
    pub boxes: HashMap<usize, (usize, usize, usize)>,
    #[serde(skip_serializing)]
    pub dy: usize
}

impl FilledPalett {
    pub fn new() -> FilledPalett {
        FilledPalett {
            boxes: HashMap::new(),
            dy: 0
        }
    }
}
