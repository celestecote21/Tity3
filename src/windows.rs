use crate::container::*;
use crate::keyboard::parse_input;
use crate::pane::*;
use crate::size_utilis::*;
use std::fs::File;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};

pub fn start_wind(
    stdio_master: File,
    rect: Rect,
    id: String,
) -> Result<(Sender<ChildToParent>, JoinHandle<()>), ContainerError> {
    let mut id_child = id.clone();
    id_child.push_str("1");
    let (parent_com_tx, parent_com_rx) = mpsc::channel();
    let com_clone = parent_com_tx.clone();
    let com_clone_tx = parent_com_tx.clone();
    let stdio_clone = match stdio_master.try_clone() {
        Ok(f) => f,
        Err(_) => return Err(ContainerError::BadTransform),
    };
    let mut child = Container::Pane(Pane::new(
        stdio_clone,
        parent_com_tx,
        rect.clone(),
        id_child,
    )?);
    let thread_hand = thread::spawn(move || {
        loop {
            let com = match parent_com_rx.recv() {
                Ok(data) => data,
                _ => break,
            };
            match com {
                ChildToParent::Refresh => draw_container(&mut child),
                ChildToParent::AddChild(cont) => {
                    child = match add_child_container(child, cont) {
                        Ok(ch) => ch,
                        Err(_) => break,
                    }
                }
                ChildToParent::GetInputData(data, size) => {
                    let (data, size) = parse_input(data, size, &com_clone_tx);
                }
                _ => (),
            }
        }
        drop(parent_com_rx);
    });
    Ok((com_clone, thread_hand))
}
