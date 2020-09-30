use ggez::*;
use ggez::input::mouse::MouseButton;
use nalgebra as na;

// variables
const CLEAR_COLOR: graphics::Color = graphics::Color::new(0.9, 0.87, 0.72, 1.0); // background-color

type Point2 = na::Point2<f32>;

#[derive(Debug)]
struct Line{
    points: Vec<Point2>,
}

impl Line {
    fn new() -> Self{
        Line {
            points: vec![]
        }
    }
}

struct State {
    mouse_down: bool,
    current_line: Option<Line>,
    lines: Vec<graphics::Mesh>
}

fn distance (p1: &Point2, p2: &Point2) -> f64{
    ((p1[0] - p2[1]).powf(2.) as f64 + (p1.y - p2.y).powf(2.) as f64).sqrt()
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, CLEAR_COLOR);
        for line in &self.lines {
            graphics::draw(ctx, line, graphics::DrawParam::default())?; 
        }
        graphics::set_window_title(ctx, &format!("{:.0} FPS", timer::fps(ctx)));
        graphics::present(ctx)?;
        Ok(())
        
    }
    
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32){
        self.mouse_down = true;
        self.current_line = Some(Line::new());

        // push twice because one point is the origin and the second one gets replaced with the mouse position
        self.current_line.as_mut().unwrap().points.push(Point2::new(x, y));
        self.current_line.as_mut().unwrap().points.push(Point2::new(x + 1., y)); // +1. because they have to be unique


    }
    fn mouse_button_up_event(&mut self, ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32){
        self.mouse_down = false;
        let line = graphics::Mesh::new_line(ctx, &self.current_line.take().unwrap().points, 3.0, graphics::BLACK).unwrap();
        self.lines.push(line);
    }
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32){
        if self.mouse_down{
            let line = self.current_line.as_mut().unwrap();
            line.points.pop();
            let last_point = line.points.last().unwrap().clone();
            let current_position = Point2::new(x, y);
            if distance(&last_point, &current_position) > 2. {
                line.points.push(current_position)
            }
            if &current_position == &last_point{
                line.points.push(Point2::new(x, y))
            }
            else {
                line.points.push(Point2::new(x, y))
            }
        }
    }
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height)).unwrap();
    }
}

fn main() {
    let state = &mut State {
        mouse_down: false,
        current_line: None,
        lines: vec![]
    };

    let cb =
        ggez::ContextBuilder::new("Back to the drawing board", "Sebastian motherfucking klähn")
        .window_setup(conf::WindowSetup::default().title("Drawing bord"))
        .window_mode(conf::WindowMode::default().resizable(true)); //.borderless(true)
    let (ref mut ctx, ref mut event_loop) = &mut cb.build().unwrap();
    event::run(ctx, event_loop, state).unwrap();
}