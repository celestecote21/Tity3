use crate::container::*;
use std::sync::mpsc::Sender;
use std::cmp::PartialEq;


pub enum Action {
    AddPane,
    DeletePane,
}

pub struct KeyAction {
    keycode: u16,
    action: Action,
}

impl KeyAction {
    pub fn take_action(&self,
        command_sender: &Sender<ChildToParent>,
        base: &MiniContainer,
    ) {
        match self.action {
            Action::AddPane => {
                let minicont = base.duplic(ContainerType::Pane).unwrap();
                command_sender
                    .send(ChildToParent::AddChild(Container::MiniCont(minicont)))
                    .unwrap();
            }
            Action::DeletePane => {

            }
        }
    }
}

impl PartialEq for KeyAction {
    fn eq(&self, other: &Self) -> bool {
        self.keycode == other.keycode
    }
}
