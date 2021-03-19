use crate::container::*;
use crate::layout::*;
use crate::windows::WindowsConf;
use std::cmp::PartialEq;

pub enum Action {
    AddPane,
    DeletePane,
    MoveFocus,
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
            Action::DeletePane => {
                config
                    .get_sender()
                    .send(ChildToParent::DestroyChild("-2".to_string()))
                    .unwrap();
            }
            Action::MoveFocus => {
                config
                    .get_sender()
                    .send(ChildToParent::MoveFocus(MoveDir::Left))
                    .unwrap();
            }
        }
    }
}

impl PartialEq for KeyAction {
    fn eq(&self, other: &Self) -> bool {
        self.keycode == other.keycode
    }
}
