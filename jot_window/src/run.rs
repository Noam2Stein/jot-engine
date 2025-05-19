use std::{mem::transmute, sync::Arc, time::Instant};

use gilrs::Gilrs;
use wgpu::{PresentMode, Surface, SurfaceConfiguration};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use crate::*;

pub fn run<A: App>() {
    let event_loop = EventLoop::new().unwrap();

    let mut game_runner = AppRunner::<A>::Uninit;

    event_loop.run_app(&mut game_runner).unwrap();
}

enum AppRunner<'a, A: App> {
    Uninit,
    Init(InitAppRunner<'a, A>),
    Exited,
}
struct InitAppRunner<'a, A: App> {
    app: A,
    resources: AppContext,
    surface: Surface<'a>,
    surface_config: SurfaceConfiguration,
    gilrs: Gilrs,
    instant: Instant,
}

impl<'a, A: App> ApplicationHandler for AppRunner<'a, A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match self {
            AppRunner::Uninit => *self = AppRunner::Init(InitAppRunner::new(event_loop)),
            AppRunner::Init(_) => {}
            AppRunner::Exited => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let runner = if let Self::Init(runner) = self {
            runner
        } else {
            return;
        };

        match runner.poll_gilrs_events() {
            AppFlow::Continue => {}
            AppFlow::Exit => {
                event_loop.exit();
                *self = AppRunner::Exited;

                return;
            }
        };

        let new_instant = Instant::now();
        let frame_duration = new_instant - runner.instant;
        runner.instant = new_instant;
        let delta_time = frame_duration
            .as_secs_f64()
            .clamp(1.0 / 10_000.0, 1.0 / 30.0);

        match runner.app.update(delta_time, &runner.resources) {
            AppFlow::Continue => {
                runner.resources.window.request_redraw();
            }
            AppFlow::Exit => {
                event_loop.exit();
                *self = AppRunner::Exited;
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        window_event: WindowEvent,
    ) {
        let runner = if let Self::Init(runner) = self {
            runner
        } else {
            return;
        };

        match window_event {
            WindowEvent::RedrawRequested => {
                if let Ok(mut frame) = runner.surface.get_current_texture() {
                    runner.app.draw(&mut frame, &runner.resources);

                    frame.present();
                };
            }
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    runner.surface_config.width = size.width;
                    runner.surface_config.height = size.height;
                    runner
                        .surface
                        .configure(&runner.resources.device, &runner.surface_config);
                }
            }
            _ => {}
        };

        if let Some(event) = Event::from_winit(&window_event) {
            match runner.app.event(&event, &runner.resources) {
                AppFlow::Continue => {}
                AppFlow::Exit => {
                    event_loop.exit();
                    *self = AppRunner::Exited;
                }
            }
        }
    }
}

impl<'a, A: App> InitAppRunner<'a, A> {
    fn new(event_loop: &ActiveEventLoop) -> Self {
        let window = event_loop.create_window(A::window_attrs()).unwrap();

        let wgpu = wgpu::Instance::default();
        let adapter_options = wgpu::RequestAdapterOptions::default();
        let adapter = pollster::block_on(wgpu.request_adapter(&adapter_options))
            .expect("adapter request failed");

        let device_desc = wgpu::DeviceDescriptor::default();
        let (device, queue) =
            pollster::block_on(adapter.request_device(&device_desc, None)).unwrap();

        let window = Arc::new(window);
        let queue = Arc::new(queue);
        let device = Arc::new(device);

        let surface = wgpu
            .create_surface(unsafe { transmute::<&Window, &Window>(&window) })
            .unwrap();

        let mut surface_config = surface
            .get_default_config(
                &adapter,
                window.inner_size().width,
                window.inner_size().height,
            )
            .unwrap();

        surface_config.present_mode = PresentMode::AutoNoVsync;

        surface.configure(&device, &surface_config);

        let resources = AppContext {
            device,
            queue,
            window,
            surface_format: surface_config.format,
        };

        let gilrs = Gilrs::new().unwrap();

        let app = A::new(&resources);

        let instant = Instant::now();

        Self {
            app,
            resources,
            surface,
            surface_config,
            gilrs,
            instant,
        }
    }

    fn poll_gilrs_events(&mut self) -> AppFlow {
        while let Some(event) = self.gilrs.next_event() {
            macro_rules! event {
                ($input_event:expr) => {
                    match self.app.event(
                        &Event::InputDevice {
                            event: $input_event,
                            device: InputDeviceId::Gilrs(event.id),
                        },
                        &self.resources,
                    ) {
                        AppFlow::Continue => {}
                        AppFlow::Exit => {
                            return AppFlow::Exit;
                        }
                    }
                };
            }

            match event.event {
                gilrs::EventType::Connected => event!(InputDeviceEvent::Connect),
                gilrs::EventType::Disconnected => event!(InputDeviceEvent::Disconnect),
                gilrs::EventType::ButtonPressed(button, _code) => {
                    if let Some(button_code) = gilrs_to_button(button) {
                        event!(InputDeviceEvent::Gamepad(GamepadEvent::Button {
                            button: button_code,
                            state: ElementState::Pressed
                        }))
                    }
                }
                gilrs::EventType::ButtonReleased(button, _code) => {
                    if let Some(button_code) = gilrs_to_button(button) {
                        event!(InputDeviceEvent::Gamepad(GamepadEvent::Button {
                            button: button_code,
                            state: ElementState::Released
                        }))
                    }
                }
                gilrs::EventType::ButtonChanged(button, value, _code) => {
                    if let Some(value_code) = gilrs_to_value(button) {
                        event!(InputDeviceEvent::Gamepad(GamepadEvent::Value {
                            code: value_code,
                            value: (value * 255.0) as u8
                        }))
                    }
                }
                gilrs::EventType::AxisChanged(axis, value, _code) => {
                    if let Some((positive_value_code, negative_value_code)) = gilrs_to_axis(axis) {
                        event!(InputDeviceEvent::Gamepad(GamepadEvent::Value {
                            code: positive_value_code,
                            value: (value * 255.0).max(0.0) as u8
                        }));
                        event!(InputDeviceEvent::Gamepad(GamepadEvent::Value {
                            code: negative_value_code,
                            value: (value * -255.0).max(0.0) as u8
                        }));
                    };
                }
                gilrs::EventType::ButtonRepeated(_, _) => {}
                gilrs::EventType::Dropped => {}
                _ => {}
            };
        }

        AppFlow::Continue
    }
}

fn gilrs_to_button(button: gilrs::Button) -> Option<ButtonCode> {
    match button {
        gilrs::Button::DPadRight => Some(ButtonCode::DpadRight),
        gilrs::Button::DPadLeft => Some(ButtonCode::DpadLeft),
        gilrs::Button::DPadUp => Some(ButtonCode::DpadUp),
        gilrs::Button::DPadDown => Some(ButtonCode::DpadDown),
        gilrs::Button::LeftThumb => Some(ButtonCode::LeftThumb),
        gilrs::Button::RightThumb => Some(ButtonCode::RightThumb),
        gilrs::Button::East => Some(ButtonCode::East),
        gilrs::Button::West => Some(ButtonCode::West),
        gilrs::Button::North => Some(ButtonCode::North),
        gilrs::Button::South => Some(ButtonCode::South),
        gilrs::Button::C => None,
        gilrs::Button::Z => None,
        gilrs::Button::RightTrigger => None,
        gilrs::Button::LeftTrigger => None,
        gilrs::Button::RightTrigger2 => None,
        gilrs::Button::LeftTrigger2 => None,
        gilrs::Button::Start => Some(ButtonCode::Start),
        gilrs::Button::Select => Some(ButtonCode::Select),
        gilrs::Button::Mode => None,
        gilrs::Button::Unknown => None,
    }
}

fn gilrs_to_value(button: gilrs::Button) -> Option<ValueCode> {
    match button {
        gilrs::Button::DPadRight => None,
        gilrs::Button::DPadLeft => None,
        gilrs::Button::DPadUp => None,
        gilrs::Button::DPadDown => None,
        gilrs::Button::LeftThumb => None,
        gilrs::Button::RightThumb => None,
        gilrs::Button::East => None,
        gilrs::Button::West => None,
        gilrs::Button::North => None,
        gilrs::Button::South => None,
        gilrs::Button::C => None,
        gilrs::Button::Z => None,
        gilrs::Button::RightTrigger => Some(ValueCode::RightTrigger1),
        gilrs::Button::LeftTrigger => Some(ValueCode::LeftTrigger1),
        gilrs::Button::RightTrigger2 => Some(ValueCode::RightTrigger2),
        gilrs::Button::LeftTrigger2 => Some(ValueCode::LeftTrigger2),
        gilrs::Button::Start => None,
        gilrs::Button::Select => None,
        gilrs::Button::Mode => None,
        gilrs::Button::Unknown => None,
    }
}

fn gilrs_to_axis(axis: gilrs::Axis) -> Option<(ValueCode, ValueCode)> {
    match axis {
        gilrs::Axis::DPadX => None,
        gilrs::Axis::DPadY => None,
        gilrs::Axis::LeftStickX => Some((ValueCode::LeftStickRight, ValueCode::LeftStickLeft)),
        gilrs::Axis::LeftStickY => Some((ValueCode::LeftStickUp, ValueCode::LeftStickDown)),
        gilrs::Axis::RightStickX => Some((ValueCode::RightStickRight, ValueCode::RightStickLeft)),
        gilrs::Axis::RightStickY => Some((ValueCode::RightStickUp, ValueCode::RightStickDown)),
        gilrs::Axis::LeftZ => None,
        gilrs::Axis::RightZ => None,
        gilrs::Axis::Unknown => None,
    }
}
