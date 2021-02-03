use crate::container::*;
use crate::pane::*;
use crate::size_utilis::*;
use crate::transformation::*;
use std::fs::File;
use std::sync::mpsc;

pub fn start_wind(stdio_master: File, rect: Rect, id: String) -> Result<(), ContainerError> {
    let mut id_child = id.clone();
    id_child.push_str("1");
    let (parent_com_tx, parent_com_rx) = mpsc::channel();
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
    loop {
        let com = match parent_com_rx.recv() {
            Ok(data) => data,
            _ => break,
        };
        match com {
            ChildToParent::Refresh => draw_container(&mut child),
            ChildToParent::AddChild(cont) => {
                child = add_child_container(child, cont)?;
            }
            _ => (),
        }
    }
    Ok(())
}
