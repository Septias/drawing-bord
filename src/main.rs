use ggez::graphics::DrawMode;
use ggez::input::mouse::MouseButton;
use ggez::*;
use nalgebra as na;
use std::collections::HashSet;

// variables
const CLEAR_COLOR: graphics::Color = graphics::Color::new(0.9, 0.87, 0.72, 1.0); // background-color
type Point2 = na::Point2<f32>;

trait CreateRender {
    fn notify_mouse_move(&mut self, ctx: &mut Context, pos: Point2);
    fn draw(&self, _ctx: &mut Context, magnification: &f32);
}

#[derive(Debug)]
struct DrawnLine {
    font_size: f32,
    magnification: f32,
    mesh: Option<graphics::Mesh>,
    points: HashSet<Point2>,
    position: Point2,
    draw_params: graphics::DrawParam,
}

impl DrawnLine {
    fn new(font_size: f32, magnification: f32, point0: Point2) -> Self {
        let hashset = HashSet::new();
        hashset.insert(point0);
        DrawnLine {
            font_size,
            magnification,
            points: hashset,
            position: point0,
            mesh: None,
            draw_params: graphics::DrawParam::default(),
        }
    }
}

impl CreateRender for DrawnLine {
    fn draw(&self, ctx: &mut Context, magnification: &f32) {
        if let Some(mesh) = &self.mesh {
            graphics::draw(ctx, mesh, self.draw_params).unwrap();
        }
    }
    fn notify_mouse_move(&mut self, ctx: &mut Context, pos: Point2) {
        let last_point = &self.points[self.points.len() - 1];
        if na::distance(last_point, &pos) > 2. {
            self.points.push(pos);
        }
        self.mesh = match self.points.len() {
            1 if self.points[0] != pos => {
                let mut mouse_and_point = self.points.clone();
                mouse_and_point.push(pos);
                assert_eq!(mouse_and_point.len(), 2);
                Some(graphics::Mesh::new_line(ctx, &mouse_and_point, 4.0, graphics::BLACK).unwrap())
            }
            1 => Some(
                graphics::Mesh::new_circle(ctx, DrawMode::fill(), pos, 4.0, 1.0, graphics::BLACK)
                    .unwrap(),
            ),
            _ => Some(graphics::Mesh::new_line(ctx, &self.points, 4.0, graphics::BLACK).unwrap()),
        }
    }
}

struct BoundingRect {
    points: [Point2; 2],
}

impl BoundingRect {
    fn new(points: [Point2; 2]) -> Self {
        BoundingRect { points }
    }
    fn get_rect(&self, ctx: &mut Context) -> graphics::Mesh {
        let w = self.points[1][0] - self.points[0][0];
        let h = self.points[1][1] - self.points[0][1];
        graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.),
            graphics::Rect::new(self.points[0][0], self.points[0][1], w, h),
            graphics::BLACK,
        )
        .unwrap()
    }
}

struct State {
    mouse_down: bool,
    current_struct: Option<Box<dyn CreateRender>>,
    structures: Vec<Box<dyn CreateRender>>,
    magnification: f32,
}

fn create_bounding_rect(points: &Vec<Point2>) -> BoundingRect {
    let point0 = &points[0];
    let (mut min_x, mut max_y) = (point0[0], point0[1]);
    let (mut max_x, mut min_y) = (point0[0], point0[1]);
    for point in points {
        min_x = if point[0] < min_x { point[0] } else { min_x };
        max_x = if point[0] > max_x { point[0] } else { max_x };
        min_y = if point[1] < min_y { point[1] } else { min_y };
        max_y = if point[1] > max_y { point[1] } else { max_y };
    }
    return BoundingRect::new([Point2::new(min_x, min_y), Point2::new(max_x, max_y)]);
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, CLEAR_COLOR);
        for structure in &self.structures {
            structure.draw(ctx, &4.0);
        }
        if let Some(structure) = &self.current_struct {
            structure.draw(ctx, &self.magnification);
        }
        graphics::set_window_title(ctx, &format!("{:.0} FPS", timer::fps(ctx)));
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.mouse_down = true;
        let structure = Box::new(DrawnLine::new(5.0, 1.0, Point2::new(x, y)));
        self.current_struct = Some(structure);
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.mouse_down = false;
        self.structures.push(self.current_struct.take().unwrap());
    }
    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        if self.mouse_down {
            if let Some(current_struct) = &mut self.current_struct {
                current_struct.notify_mouse_move(ctx, Point2::new(x, y));
            };
        }
    }
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
    }
}

fn main() {
    let state = &mut State {
        mouse_down: false,
        current_struct: None,
        structures: vec![],
        magnification: 1.,
    };

    let cb =
        ggez::ContextBuilder::new("Back to the drawing board", "Sebastian motherfucking klähn")
            .window_setup(conf::WindowSetup::default().title("Drawing bord"))
            .window_mode(conf::WindowMode::default().resizable(true)); //.borderless(true)
    let (ref mut ctx, ref mut event_loop) = &mut cb.build().unwrap();
    event::run(ctx, event_loop, state).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounding_box_test() {
        let points = vec![
            Point2::new(0., 2.),
            Point2::new(2., 1.),
            Point2::new(3., 0.),
            Point2::new(2., 3.),
        ];
        let rect = create_bounding_rect(&points);
        assert_eq!(rect, [Point2::new(0., 0.), Point2::new(3., 3.)]);
    }
}
