use crate::container::*;
use crate::windows::WindowsConf;
use std::cmp::PartialEq;

pub enum Action {
    AddPane,
    DeletePane,
}

pub struct KeyAction {
    pub keycode: u8,
    pub action: Action,
}

impl KeyAction {
    pub fn take_action(&self, config: &WindowsConf) {
        match self.action {
            Action::AddPane => {
                let minicont = config.get_base().duplic(ContainerType::Pane).unwrap();
                config
                    .get_sender()
                    .send(ChildToParent::AddChild(Container::MiniCont(minicont)))
                    .unwrap();
            }
            Action::DeletePane => {}
        }
    }
}

impl PartialEq for KeyAction {
    fn eq(&self, other: &Self) -> bool {
        self.keycode == other.keycode
    }
}
