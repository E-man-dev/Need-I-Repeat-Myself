use godot::prelude::*;
use crate::output::Output;
use crate::go_button::GoButton;
use godot::classes::INode;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Coordinator {
    base: Base<Node>,
}

#[godot_api]
impl INode for Coordinator {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        let mut go_button = self.base().get_node_as::<GoButton>("../HBoxContainer/VBoxContainerL/HBoxContainer/GoButton");
        let output = self.base().get_node_as::<Output>("../HBoxContainer/ScrollContainer/VBoxContainerR/Output");

        go_button.connect(
            "pressed",
            &output.callable("on_button_pressed"),
        );
    }
}
