use crate::input::key_input::KeyInput;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputEvent {
    Wheel { delta_y: f32 },
    Key(KeyInput),
    Click { x: f32, y: f32 },
    SecondaryClick { x: f32, y: f32 },
    Press { x: f32, y: f32 },
    PointerMoved { x: f32, y: f32 },
    Character(char),
}
