use ggez::*;
use ggez::input::mouse::MouseButton;
use mint::{Point2};

#[derive(Debug)]
struct Line{
    points: Vec<Point2<f32>>,
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

fn distance (p1: &Point2<f32>, p2: &Point2<f32>) -> f32{
    ((p1.x - p2.x).powf(2.) + (p1.y - p2.y).powf(2.)).sqrt()
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        for line in &self.lines {
            graphics::draw(ctx, line, graphics::DrawParam::default())?; 
        }
        graphics::present(ctx)?;
        Ok(())
        
    }
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32){
        self.mouse_down = true;
        self.current_line = Some(Line::new());
        self.current_line.as_mut().unwrap().points.push(Point2{x: _x, y: _y})

    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32){
        self.mouse_down = false;
        let line = graphics::Mesh::new_line(_ctx, &self.current_line.take().unwrap().points, 1.0, graphics::WHITE).unwrap();
        self.lines.push(line);
    }
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32){
        if self.mouse_down{
            let line = self.current_line.as_mut().unwrap();
            let last_point = line.points.last().unwrap();
            let current_position = mint::Point2{x: _x, y: _y};
            if distance(last_point, &current_position) > 5. {
                line.points.push(current_position)
            }
        }
    }
}

fn main() {
    let state = &mut State {
        mouse_down: false,
        current_line: Some(Line::new()),
        lines: vec![]
    };
    let cb =
        ggez::ContextBuilder::new("Back to the drawing board", "Sebastian motherfucking klähn");
    let (ref mut ctx, ref mut event_loop) = &mut cb.build().unwrap();
    event::run(ctx, event_loop, state).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distance_check(){
        assert_eq!(distance(&Point2{x: 5., y: 0. }, &Point2{x: 0., y: 0.}), 5.);
        assert_eq!(distance(&Point2{x: 4., y: 3. }, &Point2{x: 0., y: 0.}), 5.);
    }
}