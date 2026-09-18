use nannou::prelude::*;

mod vertical_line;

fn main() {
    nannou::app(model)
        .size(1920, 216) // 80:9 ratio G05 9600x1080
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    vertical_lines: vertical_line::VerticalLines,
}

fn model(app: &App) -> Model {
    Model {
        vertical_lines: vertical_line::setup(app),
    }
}

fn update(app: &App, model: &mut Model) {
    vertical_line::update(app, &mut model.vertical_lines);
}

fn view(app: &App, model: &Model, _window: Entity) {
    let draw = app.draw();
    draw.background().color(BLACK);
    vertical_line::draw(app, &draw, &model.vertical_lines);
}
