use stdweb;

mod canvas;
mod snake;

fn main() {
    stdweb::initialize();
    let canvas = canvas::Canvas::new("#canvas", 20, 20);

    canvas.draw(5, 5, "red");
    canvas.draw(10, 5, "red");
    canvas.draw(5, 10, "red");

    stdweb::event_loop();
}
