use nannou::prelude::*;

mod vertical_line;
// register new projects here, then add a Scene variant + match arm below
// mod another_project;

/// Each variant is one visual project, switchable at runtime via number keys.
enum Scene {
    VerticalLine(vertical_line::VerticalLines),
    // AnotherProject(another_project::AnotherProject),
}

fn main() {
    nannou::app(model)
        .size(1920, 216) // 80:9 ratio G05 9600x1080
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    scene: Scene,
}

fn model(app: &App) -> Model {
    Model {
        scene: Scene::VerticalLine(vertical_line::setup(app)),
    }
}

fn update(app: &App, model: &mut Model) {
    let keys = app.keys();
    if keys.just_pressed(KeyCode::Digit1) {
        model.scene = Scene::VerticalLine(vertical_line::setup(app));
    }
    // if keys.just_pressed(KeyCode::Digit2) {
    //     model.scene = Scene::AnotherProject(another_project::setup(app));
    // }

    match &mut model.scene {
        Scene::VerticalLine(lines) => vertical_line::update(app, lines),
    }
}

fn view(app: &App, model: &Model, _window: Entity) {
    let draw = app.draw();
    draw.background().color(BLACK);

    match &model.scene {
        Scene::VerticalLine(lines) => vertical_line::draw(app, &draw, lines),
    }
}
