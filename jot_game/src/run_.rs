use std::{iter::once, mem::transmute, sync::Arc, time::Instant};

use super::*;

pub fn run<A: Game>() {
    let event_loop = EventLoop::new().unwrap();

    let mut game_runner = AppRunner::<A>::Uninit;

    event_loop.run_app(&mut game_runner).unwrap();
}

enum AppRunner<'window, A: Game> {
    Uninit,
    Init(InitAppRunner<'window, A>),
    Exited,
}
struct InitAppRunner<'window, A: Game> {
    app: A,
    window: Arc<Window>,
    surface: GPUSurface<'window>,
    surface_config: GPUSurfaceConfig,
    ctx: GPUContext,
    input: InputProvider,
    fs_switch: FullscreenSwitch,
    instant: Instant,
}

impl<'a, A: Game> ApplicationHandler for AppRunner<'a, A> {
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

        {
            let mut input_events = runner.input.poll_events();
            while let Some(input_event) = input_events.next() {
                match runner
                    .app
                    .event(&GameEvent::Input(input_event), &runner.ctx)
                {
                    GameFlow::Continue => {
                        runner.window.request_redraw();
                    }
                    GameFlow::Exit => {
                        drop(input_events);
                        event_loop.exit();
                        *self = AppRunner::Exited;
                        return;
                    }
                }
            }
        }

        let new_instant = Instant::now();
        let frame_duration = new_instant - runner.instant;
        runner.instant = new_instant;
        let delta_time = frame_duration
            .as_secs_f64()
            .clamp(1.0 / 10_000.0, 1.0 / 30.0);

        match runner.app.update(delta_time, &runner.ctx) {
            GameFlow::Continue => {
                runner.window.request_redraw();
            }
            GameFlow::Exit => {
                event_loop.exit();
                *self = AppRunner::Exited;
                return;
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

        {
            let mut input_events = runner.input.map_events(once(&window_event));
            while let Some(input_event) = input_events.next() {
                match runner
                    .app
                    .event(&GameEvent::Input(input_event), &runner.ctx)
                {
                    GameFlow::Continue => {
                        runner.window.request_redraw();
                    }
                    GameFlow::Exit => {
                        drop(input_events);
                        event_loop.exit();
                        *self = AppRunner::Exited;
                        return;
                    }
                }
            }
        }

        runner.fs_switch.event(&window_event, &runner.window);

        let event = match window_event {
            WindowEvent::RedrawRequested => {
                if let Ok(frame) = runner.surface.get_current_texture() {
                    runner
                        .app
                        .draw(&frame.texture.create_view(&Default::default()), &runner.ctx);

                    frame.present();
                };

                None
            }
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    runner.surface_config.width = size.width;
                    runner.surface_config.height = size.height;
                    runner
                        .surface
                        .configure(&runner.ctx.device, &runner.surface_config);
                }

                None
            }
            WindowEvent::CloseRequested => Some(GameEvent::ExitRequested),
            _ => None,
        };

        if let Some(event) = event {
            match runner.app.event(&event, &runner.ctx) {
                GameFlow::Continue => {}
                GameFlow::Exit => {
                    event_loop.exit();
                    *self = AppRunner::Exited;
                }
            }
        }
    }
}

impl<'a, A: Game> InitAppRunner<'a, A> {
    fn new(event_loop: &ActiveEventLoop) -> Self {
        let window = event_loop.create_window(A::window_attrs()).unwrap();

        let gpu = GPU::default();
        let adapter_options = GPURequestAdapterOptions::default();
        let adapter = pollster::block_on(gpu.request_adapter(&adapter_options))
            .expect("adapter request failed");

        let device_desc = GPUDeviceDesc::default();
        let (device, queue) =
            pollster::block_on(adapter.request_device(&device_desc, None)).unwrap();

        let window = Arc::new(window);
        let queue = Arc::new(queue);
        let device = Arc::new(device);
        let ctx = GPUContext { device, queue };

        let surface = gpu
            .create_surface(unsafe { transmute::<&Window, &Window>(&window) })
            .unwrap();

        let mut surface_config = surface
            .get_default_config(
                &adapter,
                window.inner_size().width,
                window.inner_size().height,
            )
            .unwrap();

        surface_config.present_mode = GPUPresentMode::AutoNoVsync;

        surface.configure(&ctx.device, &surface_config);

        let input = InputProvider::new();
        let fs_switch = FullscreenSwitch::new();

        let app = A::new(&ctx);

        let instant = Instant::now();

        Self {
            app,
            ctx,
            surface,
            surface_config,
            input,
            fs_switch,
            instant,
            window,
        }
    }
}
