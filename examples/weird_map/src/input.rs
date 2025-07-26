use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, InputType)]
pub struct Input {
    pub x: Axis,
    pub y: Axis,
    pub jump: Button,
}

impl Input {
    pub fn default_bindings() -> Bindings<Self> {
        Bindings::<Self> {
            x: Bindings::<Axis> {
                positive: Bindings::<Value> {
                    flat: Bindings::<Button> {
                        keys: Set64::from_iter([KeyCode::ArrowRight, KeyCode::KeyD]),
                        buttons: Set64::from_iter([ButtonCode::DpadRight]),
                        ..Default::default()
                    },
                    values: Set64::from_iter([ValueCode::LeftStickRight]),
                },
                negative: Bindings::<Value> {
                    flat: Bindings::<Button> {
                        keys: Set64::from_iter([KeyCode::ArrowLeft, KeyCode::KeyA]),
                        buttons: Set64::from_iter([ButtonCode::DpadLeft]),
                        ..Default::default()
                    },
                    values: Set64::from_iter([ValueCode::LeftStickLeft]),
                },
            },

            y: Bindings::<Axis> {
                positive: Bindings::<Value> {
                    flat: Bindings::<Button> {
                        keys: Set64::from_iter([KeyCode::ArrowUp, KeyCode::KeyW]),
                        buttons: Set64::from_iter([ButtonCode::DpadUp]),
                        ..Default::default()
                    },
                    values: Set64::from_iter([ValueCode::LeftStickUp]),
                },
                negative: Bindings::<Value> {
                    flat: Bindings::<Button> {
                        keys: Set64::from_iter([KeyCode::ArrowDown, KeyCode::KeyS]),
                        buttons: Set64::from_iter([ButtonCode::DpadDown]),
                        ..Default::default()
                    },
                    values: Set64::from_iter([ValueCode::LeftStickDown]),
                },
            },

            jump: Bindings::<Button> {
                keys: Set64::from_iter([KeyCode::Space]),
                buttons: Set64::from_iter([ButtonCode::South, ButtonCode::East]),
                ..Default::default()
            },
        }
    }
}
