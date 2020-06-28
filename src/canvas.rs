use stdweb::traits::IParentNode;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    scaled_width: u32,
    scaled_height: u32,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(attr_id: &str, width: u32, height: u32) -> Self {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();
        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
        let scaled_width = canvas.width() / width;
        let scaled_height = canvas.height() / height;
        Self {
            canvas,
            ctx,
            scaled_width,
            scaled_height,
            width,
            height,
        }
    }

    pub fn draw(&self, x: u32, y: u32, color: &str) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.ctx.set_fill_style_color(color);

        let scaled_x = x * self.scaled_width;
        let scaled_y = y * self.scaled_height;

        self.ctx.fill_rect(
            scaled_x as f64,
            scaled_y as f64,
            self.scaled_width as f64,
            self.scaled_height as f64,
        );
    }

    pub fn clear_all(&self) {
        self.ctx.set_fill_style_color("white");
        self.ctx.fill_rect(
            0.0,
            0.0,
            (self.width * self.scaled_width) as f64,
            (self.height * self.scaled_height) as f64,
        );
    }
}
