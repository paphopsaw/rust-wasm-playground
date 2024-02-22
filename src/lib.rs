use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Particle {
    pos_x: f32,
    pos_y: f32
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    particles: Vec<Particle>
}


#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn size(&self) -> usize {
        self.particles.len()
    }

    pub fn new() -> Universe {
        Universe {
            width: 960,
            height: 720,
            particles: Vec::new()
        }
    }

    pub fn particles(&self) -> *const Particle {
        self.particles.as_ptr()
    }

    pub fn add_particle(&mut self, pos_x: f32, pos_y: f32) {
        let new_particle = Particle{ pos_x, pos_y };
        self.particles.push(new_particle)
    }
}