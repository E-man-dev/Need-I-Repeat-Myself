use godot::prelude::*;
use godot::classes::Button;

#[derive(GodotClass)]
#[class(init, base=Button)]
pub struct GoButton {
    base: Base<Button>,
}
