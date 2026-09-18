use nannou::prelude::*;

pub struct VerticalLine {
    x: f32,
    speed: f32,
    weight: f32,
}

pub struct VerticalLines {
    lines: Vec<VerticalLine>,
}

const LINE_COUNT: usize = 40;
const MIN_SPEED: f32 = 1.0;
const MAX_SPEED: f32 = 4.0;
const GLOW_LAYERS_NUM: usize = 10;

pub fn setup(app: &App) -> VerticalLines {
    let win = app.window_rect();
    let mut lines = Vec::with_capacity(LINE_COUNT);

    for _ in 0..LINE_COUNT {
        // spawn near the center, moving outward to the left or right
        let going_right = random::<bool>();
        let x = random_range(win.left() * 0.1, win.right() * 0.1);
        let speed = random_range(MIN_SPEED, MAX_SPEED) * if going_right { 1.0 } else { -1.0 };
        let weight = win.w() as f32 * 0.0005;
        lines.push(VerticalLine { x, speed, weight });
    }

    VerticalLines { lines }
}

pub fn update(app: &App, model: &mut VerticalLines) {
    let win = app.window_rect();

    for line in model.lines.iter_mut() {
        line.x += line.speed;
    }

    // once a line moves off screen, respawn it near the center
    for line in model.lines.iter_mut() {
        if line.x < win.left() || line.x > win.right() {
            let going_right = line.speed > 0.0;
            line.x = random_range(win.left() * 0.1, win.right() * 0.1);
            line.speed = random_range(MIN_SPEED, MAX_SPEED) * if going_right { 1.0 } else { -1.0 };
        }
    }
}


pub fn draw(app: &App, draw: &Draw, model: &VerticalLines) {
    let win = app.window_rect();

    for line in model.lines.iter() {
        let start = pt2(line.x, win.bottom());
        let end = pt2(line.x, win.top());

        for i in 0..GLOW_LAYERS_NUM {
            let weight = line.weight * (GLOW_LAYERS_NUM - i) as f32;
            let alpha: f32 = 0.01 * (i + 1) as f32;
            draw.line()
                .start(start)
                .end(end)
                .weight(weight)
                .linear_rgba(1.0, 1.0, 1.0, alpha);
            // .color(rgba(1.0, 1.0, 1.0, *alpha));
        }

        draw.line().start(start).end(end).weight(line.weight).color(WHITE);
    }
}
