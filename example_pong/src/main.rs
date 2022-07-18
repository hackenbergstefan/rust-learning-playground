use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500, 500)
        .run();
}

struct Model {
    ball: Ball,
    bar: Bar,
}

struct Ball {
    position: Point2,
    velocity: Vec2,
}

impl Ball {
    fn new() -> Self {
        let position = pt2(0.0, 0.0);
        let velocity = vec2(2.5, 5.0);
        Ball { position, velocity }
    }

    fn update(&mut self, rect: Rect<f32>, bar: &Bar) {
        // Add the current speed to the position.
        self.position += self.velocity;

        if self.position.x > rect.right() || self.position.x < rect.left() {
            self.velocity.x = self.velocity.x * -1.0;
        }
        if self.position.y > rect.top() || self.position.y < rect.bottom() {
            self.velocity.y = self.velocity.y * -1.0;
        }

        if bar.contains(self.position) {
            self.velocity.x *= -1.0;
        }
    }

    fn display(&self, draw: &Draw) {
        // Display circle at x position
        draw.ellipse()
            .xy(self.position)
            .w_h(16.0, 16.0)
            .gray(0.5)
            .stroke(BLACK);
    }
}

struct Bar {
    length: f32,
    position: Vec2,
    weight: f32,
}

impl Bar {
    fn new(rect: Rect<f32>) -> Self {
        Bar {
            length: 50.0,
            position: rect.mid_left() + pt2(30.0, 0.0),
            weight: 20.0,
        }
    }

    fn update(&mut self, app: &App) {
        self.position.x = -app.window_rect().w() / 2.0 + 30.0;

        let diff = || -> f32 {
            for key in app.keys.down.iter() {
                return match key {
                    Key::Up => 10.0,
                    Key::Down => -10.0,
                    _ => 0.0,
                };
            }
            0.0
        }();
        self.position.y += diff;

        // Keep in bounding box
        let window_height = app.window_rect().h() / 2.0;

        if self.position.y + self.length > window_height {
            self.position.y = window_height - self.length;
        } else if self.position.y - self.length < -window_height {
            self.position.y = -window_height + self.length;
        }
    }

    fn display(&self, draw: &Draw) {
        // Display circle at x position
        draw.line()
            .start(self.position - pt2(0.0, self.length))
            .end(self.position + pt2(0.0, self.length))
            .weight(self.weight);
    }

    fn contains(&self, point: Vec2) -> bool {
        Rect::<f32> {
            x: geom::Range::<f32> {
                start: self.position.x - self.weight / 2.0,
                end: self.position.x + self.weight / 2.0,
            },
            y: geom::Range::<f32> {
                start: self.position.y - self.length,
                end: self.position.y + self.length,
            },
        }
        .contains(point)
    }
}

fn model(app: &App) -> Model {
    Model {
        ball: Ball::new(),
        bar: Bar::new(app.window_rect()),
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    m.ball.update(app.window_rect(), &m.bar);
    m.bar.update(app);
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.rect().wh(app.window_rect().wh()).rgb(1.0, 1.0, 1.0);

    m.ball.display(&draw);
    m.bar.display(&draw);

    draw.to_frame(app, &frame).unwrap();
}
