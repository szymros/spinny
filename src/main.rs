mod camera;
mod instance;
mod layouts;
mod light;
mod model;
mod state;
mod texture;
mod binding;
mod vertex;

use std::sync::Arc;
use std::time::Instant;

use state::State;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::Key;
use winit::window::{self, WindowAttributes};

#[derive(Default)]
struct App {
    state: Option<State>,
    last_updated: Option<std::time::Instant>,
    delta_time: Option<f32>,
    focused: Option<bool>,
}

impl App {
    fn input_handler(&mut self, key: Key, delta_time: f32) {
        let state = self.state.as_mut().unwrap();
        match key.as_ref() {
            Key::Character("w") => {
                state.camera.position += state.camera.direction * state.camera.speed * delta_time;
            }
            Key::Character("a") => {
                state.camera.position +=
                    glam::Vec3::Y.cross(state.camera.direction) * state.camera.speed * delta_time;
            }
            Key::Character("s") => {
                state.camera.position -= state.camera.direction * state.camera.speed * delta_time;
            }

            Key::Character("d") => {
                state.camera.position -=
                    glam::Vec3::Y.cross(state.camera.direction) * state.camera.speed * delta_time;
            }
            _ => (),
        }
    }

    fn mouse_hanlder(&mut self, delta_mouse: (f64, f64), delta_time: f32) {
        let state = self.state.as_mut().unwrap();
        state.camera.yaw += delta_mouse.0 as f32 * state.camera.speed * delta_time * 0.05;
        state.camera.pitch -= delta_mouse.1 as f32 * state.camera.speed * delta_time * 0.05;
    }
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default())
                .unwrap(),
        );
        let state = pollster::block_on(State::new(window.clone()));
        self.state = Some(state);
        self.last_updated = Some(Instant::now());
        self.delta_time = Some(0.0);
        self.focused = Some(true);
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _cause: winit::event::StartCause,
    ) {
        let now = std::time::Instant::now();
        let delta_time = match self.last_updated {
            Some(delta) => now.duration_since(delta).as_secs_f32(),
            None => 0.0,
        };
        self.last_updated = Some(now);
        self.delta_time = Some(delta_time);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => {
                state.spin_teapots();
                state.camera.update_view_matrix();
                state.render();
                state.window.request_redraw();
            }
            winit::event::WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => self.input_handler(logical_key, self.delta_time.unwrap()),
            winit::event::WindowEvent::Focused(focused) => {
                self.focused = Some(focused);
            }
            _ => (),
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                if self.focused.unwrap(){
                    self.mouse_hanlder(delta, self.delta_time.unwrap())
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}
