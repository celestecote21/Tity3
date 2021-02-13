use crate::container::*;
use crate::layout::*;
use crate::size_utilis::*;
use std::fs::File;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

//TODO: make a new struct just for the interne in the thread because functions take to much args

pub struct Split {
    id: String,
    next_id: usize,
    stdio_master: File,
    parent_com: Sender<ChildToParent>,
    rect: Rect,
    intern_com: Sender<ChildToParent>,
    direction: Direction,
}

impl Split {
    /// need to make only on new
    pub fn new(
        stdio_master: File,
        parent_com: Sender<ChildToParent>,
        rect: Rect,
        id: String,
        direction: Direction,
        child: Option<Container>,
    ) -> Result<Split, ContainerError> {
        let (intern_com_tx, intern_com_rx) = mpsc::channel();
        let rect_clone = rect.clone();
        let intern_com_tx_clone = intern_com_tx.clone();
        thread::spawn(move || {
            split_thread(intern_com_rx, intern_com_tx, rect_clone, direction, child);
        });
        let nw_split = Split {
            next_id: 1,
            stdio_master,
            parent_com,
            rect,
            id,
            intern_com: intern_com_tx_clone,
            direction: Direction::Horizontal,
        };
        Ok(nw_split)
    }

    /// the Split struct contains multiple other contenaire that can ben pane os other Split
    /// So the draw fonction will call all the draw fonction of the child
    /// but because the handleling of the child is inside a thread
    /// it send the refresh commande
    pub fn draw(&self) {
        match self.intern_com.send(ChildToParent::Refresh) {
            Err(_) => self
                .parent_com
                .send(ChildToParent::DestroyChild(self.get_id()))
                .unwrap(),
            _ => (),
        }
    }

    pub fn get_input(&mut self, data: [u8; 4096], size: usize) {
        match self
            .intern_com
            .send(ChildToParent::GetInputData(data, size))
        {
            Err(_) => self
                .parent_com
                .send(ChildToParent::DestroyChild(self.get_id()))
                .unwrap(),
            _ => (),
        }
    }

    pub fn add_child(self, cont: Container) -> Result<Container, ContainerError> {
        match self.intern_com.send(ChildToParent::AddChild(cont)) {
            Err(_) => self
                .parent_com
                .send(ChildToParent::DestroyChild(self.get_id()))
                .unwrap(),
            _ => (),
        }
        Ok(Container::Split(self))
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn identifi(&self, id_test: &String) -> bool {
        self.id.eq(id_test)
    }

    pub fn change_rect(&mut self, rect: &Rect) {
        todo!()
    }
}

fn split_thread(
    receiver: Receiver<ChildToParent>,
    sender_for_child: Sender<ChildToParent>,
    base_rect: Rect,
    direction: Direction,
    child: Option<Container>,
) {
    let mut layout = Layout::new(base_rect, direction);
    let mut focused = None;
    let mut list_child: ContainerList = Vec::new();

    if child.is_some() {
        list_child.push(child.unwrap());
        layout.add_child();
    }
    loop {
        let com = match receiver.recv() {
            Ok(data) => data,
            _ => break,
        };
        match com {
            ChildToParent::Refresh => redraw_child(&mut list_child),
            ChildToParent::DestroyChild(id) => destroy_child(&mut list_child, id, &mut layout),
            ChildToParent::AddChild(cont) => {
                match add_child_split(
                    &mut list_child,
                    cont,
                    sender_for_child.clone(),
                    &mut layout,
                    &mut focused,
                ) {
                    Err(_) => break,
                    _ => (),
                }
            }
            ChildToParent::GetInputData(input, size) => {
                send_input_to_child(&mut list_child, &mut focused, input, size)
            }
        }
    }
    //TODO: supresse this container
}

fn redraw_child(list_child: &mut ContainerList) {
    list_child.iter_mut().for_each(|cont| {
        draw_container(cont);
    });
}

fn destroy_child(list_child: &mut ContainerList, id: String, layout: &mut Layout) {
    let pos_child = list_child.iter().position(|child| match child {
        Container::Pane(pa) => pa.identifi(&id),
        Container::Split(sp) => sp.identifi(&id),
        _ => panic!("this can't have other type of child"),
    });

    if pos_child.is_some() {
        list_child.remove(pos_child.unwrap());
        layout.del_child();
        //TODO: recalculate the size of childs and set it
        redraw_child(list_child);
    }
}

fn add_child_split(
    list_child: &mut ContainerList,
    cont: Container,
    parent_com: Sender<ChildToParent>,
    layout: &mut Layout,
    focused: &mut Option<usize>,
) -> Result<(), ContainerError> {
    let mut rect_child = layout.add_child();
    let nw_cont = match cont {
        Container::MiniCont(mini) => mini.complet(Some(parent_com), Some(rect_child.clone()))?,
        other => other,
    };
    if focused.is_some() {
        let id = get_id_container(list_child.get(focused.unwrap()).unwrap());
        let pos_child = list_child.iter().position(|child| match child {
            Container::Pane(pa) => pa.identifi(&id),
            Container::Split(sp) => sp.identifi(&id),
            _ => panic!("this can't have other type of child"),
        });
        if pos_child.is_some() {
            let focused_child = list_child.remove(focused.unwrap());
            list_child.insert(
                pos_child.unwrap(),
                add_child_container(focused_child, nw_cont)?,
            );
            return Ok(());
        }
    }
    list_child.push(nw_cont);
    *focused = Some(list_child.len() - 1);
    //TODO: handle with the layout
    let direction = layout.get_direction();
    for ch in list_child.iter_mut() {
        change_rect_container(&rect_child, ch);
        match direction {
            Direction::Horizontal =>  rect_child.x += rect_child.w,
            Direction::Vertical =>  rect_child.y += rect_child.h,
        }
    }
    Ok(())
}

fn send_input_to_child(
    list_child: &mut ContainerList,
    focused: &mut Option<usize>,
    input: [u8; 4096],
    size: usize,
) {
    if focused.is_none() {
        return;
    }
    let focused_child = match list_child.get_mut(focused.unwrap()) {
        Some(child) => Some(child),
        None => get_focused_child(list_child, None, focused),
    };
    if focused_child.is_none() {
        return;
    }
    get_input_container(input, size, focused_child.unwrap());
}

fn get_focused_child<'a>(
    list_child: &'a mut ContainerList,
    new_focus: Option<usize>,
    focused: &mut Option<usize>,
) -> Option<&'a mut Container> {
    if new_focus.is_some() {
        *focused = Some(new_focus.unwrap());
        return list_child.get_mut(focused.unwrap());
    }
    if list_child.is_empty() {
        *focused = None;
        return None;
    }
    if focused.is_none() {
        return None;
    }
    let mut tmp = focused.unwrap();
    while list_child.get(tmp).is_none() {
        tmp -= 1;
    }
    *focused = Some(tmp);
    list_child.get_mut(tmp)
}
