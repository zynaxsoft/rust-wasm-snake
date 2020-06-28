use super::canvas::Canvas;
use stdweb::js;
use stdweb::unstable::TryInto;

pub struct Board {
    pub canvas: Canvas,
    width: u32,
    height: u32,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        let canvas = Canvas::new("#canvas", width, height);
        Self {
            canvas,
            width,
            height,
        }
    }

    pub fn update(&self) {
        self.canvas.draw(random_u32(20), random_u32(20), "green");
    }
}

fn random_u32(max: u32) -> u32 {
    let random = js! { return Math.floor(Math.random() * @{max}) }
        .try_into()
        .unwrap();
    random
}
