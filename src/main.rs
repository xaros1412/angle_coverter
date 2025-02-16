use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{self, ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

fn to_low(num: f64) -> f64 {
    num * 60.0
}
fn to_more(num: f64) -> f64 {
    num / 60.0
}

#[derive(Default)]
struct App {
    window: Option<Box<dyn Window>>,
}

impl ApplicationHandler for App {
    fn can_create_surfaces(&mut self, event_loop: &dyn event_loop::ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(WindowAttributes::default())
                .unwrap(),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close. stop");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    println!("It's ok");
    Ok(())
}

#[test]
fn degree_to_minute_is_60() {
    let degree: f64 = 1.0;
    let minute: f64 = to_low(degree);
    assert_eq!(minute, 60.0)
}
#[test]
fn minute_to_second_is_60() {
    let minute: f64 = 1.0;
    let second: f64 = to_low(minute);
    assert_eq!(second, 60.0)
}
#[test]
fn degree_to_second_is_3600() {
    let degree: f64 = 1.0;
    let minute: f64 = to_low(degree);
    let second: f64 = to_low(minute);
    assert_eq!(second, 3600.0)
}
#[test]
fn second_to_minute_is_one() {
    let second: f64 = 60.0;
    let minute: f64 = to_more(second);
    assert_eq!(minute, 1.0)
}
#[test]
fn minute_to_degree_is_one() {
    let minute: f64 = 60.0;
    let degree: f64 = to_more(minute);
    assert_eq!(degree, 1.0)
}
#[test]
fn second_to_degree_is_1() {
    let second: f64 = 3600.0;
    let minute: f64 = to_more(second);
    let degree: f64 = to_more(minute);
    assert_eq!(degree, 1.0)
}
