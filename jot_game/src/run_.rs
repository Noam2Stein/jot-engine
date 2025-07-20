use std::{iter::once, sync::Arc, time::Instant};

use super::*;

pub fn run<A: Game>() {
    let event_loop = EventLoop::new().unwrap();

    let mut game_runner = GameRunner::<A>::Uninit;

    event_loop.run_app(&mut game_runner).unwrap();
}

enum GameRunner<G: Game> {
    Uninit,
    Init(InitGameRunner<G>),
    Exited,
}
struct InitGameRunner<G: Game> {
    game: G,
    gpu: Gpu,
    window: Arc<Window>,
    fs_switch: FullscreenSwitch,
    surface: GpuSurface<'static>,
    input: InputProvider,
    instant: Instant,
}

impl<G: Game> ApplicationHandler for GameRunner<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match self {
            GameRunner::Uninit => *self = GameRunner::Init(InitGameRunner::new(event_loop)),
            GameRunner::Init(_) => {}
            GameRunner::Exited => {}
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
                    .game
                    .event(&GameEvent::Input(input_event), &runner.gpu)
                {
                    GameFlow::Continue => {
                        runner.window.request_redraw();
                    }
                    GameFlow::Exit => {
                        drop(input_events);
                        event_loop.exit();
                        *self = GameRunner::Exited;
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

        match runner.game.update(delta_time, &runner.gpu) {
            GameFlow::Continue => {
                runner.window.request_redraw();
            }
            GameFlow::Exit => {
                event_loop.exit();
                *self = GameRunner::Exited;
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
                    .game
                    .event(&GameEvent::Input(input_event), &runner.gpu)
                {
                    GameFlow::Continue => {
                        runner.window.request_redraw();
                    }
                    GameFlow::Exit => {
                        drop(input_events);
                        event_loop.exit();
                        *self = GameRunner::Exited;
                        return;
                    }
                }
            }
        }

        runner.fs_switch.event(&window_event, &runner.window);

        let event = match window_event {
            WindowEvent::RedrawRequested => {
                let frame = runner.surface.next_frame();

                runner.game.draw(&frame.texture(), &runner.gpu);

                None
            }
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    runner
                        .surface
                        .resize(vec2!(size.width, size.height), &runner.gpu);
                }

                None
            }
            WindowEvent::CloseRequested => Some(GameEvent::ExitRequested),
            _ => None,
        };

        if let Some(event) = event {
            match runner.game.event(&event, &runner.gpu) {
                GameFlow::Continue => {}
                GameFlow::Exit => {
                    event_loop.exit();
                    *self = GameRunner::Exited;
                }
            }
        }
    }
}

impl<G: Game> InitGameRunner<G> {
    fn new(event_loop: &ActiveEventLoop) -> Self {
        let window = event_loop
            .create_window(WindowAttributes::default().with_title(G::NAME))
            .unwrap();

        let window = Arc::new(window);
        let gpu = Gpu::any();

        let surface = gpu.create_surface(window.clone(), &G::surface_desc());

        let input = InputProvider::new();
        let fs_switch = FullscreenSwitch::new();

        let app = G::new(&gpu);

        let instant = Instant::now();

        Self {
            game: app,
            gpu,
            surface,
            input,
            fs_switch,
            instant,
            window,
        }
    }
}
