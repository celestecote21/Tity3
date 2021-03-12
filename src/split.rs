use crate::container::*;
use crate::container_action::*;
use crate::layout::*;
use crate::size_utilis::*;
use std::fs::File;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

//TODO: make a new struct just for the interne in the thread because functions take to much args

pub struct Split {
    id: String,
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
        let intern_id = id.clone();
        thread::spawn(move || {
            split_thread(
                intern_com_rx,
                intern_com_tx,
                rect_clone,
                direction,
                child,
                intern_id,
            );
        });
        let nw_split = Split {
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

    pub fn change_focus(&self, dir: &MoveDir) {
        match self.intern_com.send(ChildToParent::MoveFocus(*dir)) {
            Err(_) => self
                .parent_com
                .send(ChildToParent::DestroyChild(self.get_id()))
                .unwrap(),
            _ => (),
        }
    }

    pub fn change_rect(&mut self, _rect: &Rect) {
        // TODO
        todo!()
    }
}

struct InternSplit {
    receiver: Receiver<ChildToParent>,
    com_parent: Sender<ChildToParent>,
    list_child: ContainerList,
    layout: Layout,
    focused: Option<usize>,
    id: String,
}

fn split_thread(
    receiver: Receiver<ChildToParent>,
    sender_for_child: Sender<ChildToParent>,
    base_rect: Rect,
    direction: Direction,
    child: Option<Container>,
    id: String,
) {
    let layout = Layout::new(base_rect, direction);
    let list_child: ContainerList = Vec::new();

    let mut intern = InternSplit {
        receiver,
        com_parent: sender_for_child,
        list_child,
        layout,
        focused: None,
        id,
    };

    if child.is_some() {
        match add_child_split(
            child.unwrap(),
            &mut intern
        ) {
            Err(_) => return,
            Ok(_) => (),
        }
    }
    loop {
        let com = match intern.receiver.recv() {
            Ok(data) => data,
            _ => break,
        };
        match com {
            ChildToParent::Refresh => redraw_child(&mut intern),
            ChildToParent::DestroyChild(id) => destroy_child(&mut intern, id),
            ChildToParent::AddChild(cont) => {
                match add_child_split(
                    cont,
                    &mut intern
                ) {
                    Err(_) => break,
                    Ok(_) => (),
                }
            }
            ChildToParent::GetInputData(input, size) => {
                send_input_to_child(&mut intern, input, size)
            }
            ChildToParent::MoveFocus(dir) => {
            }
        }
    }
    //TODO: supresse this container
}

fn redraw_child(intern: &mut InternSplit) {
    for cont in intern.list_child.iter_mut() {
        draw_container(cont);
    }
}

fn destroy_child(intern: &mut InternSplit, id: String) {
    let pos_child = intern.list_child.iter().position(|child| match child {
        Container::Pane(pa) => pa.identifi(&id),
        Container::Split(sp) => sp.identifi(&id),
        _ => panic!("this can't have other type of child"),
    });

    if pos_child.is_some() {
        intern.list_child.remove(pos_child.unwrap());
        intern.layout.del_child();
        //TODO: recalculate the size of childs and set it
        redraw_child(intern);
    }
}

fn add_child_split(
    cont: Container,
    intern: &mut InternSplit,
) -> Result<(), ContainerError> {
    let mut rect_child = intern.layout.add_child();
    let nw_cont = match cont {
        Container::MiniCont(mini) => {
            let mut nw_id = intern.id.to_owned();
            nw_id.push_str(&intern.layout.get_next_id().to_string());
            mini.complet(Some(intern.com_parent.clone()), Some(rect_child.clone()), Some(nw_id))?
        }
        other => other,
    };
    if intern.focused.is_some() {
        let cont_type = get_container_type(intern.list_child.get(intern.focused.unwrap()).unwrap());
        if cont_type != ContainerType::Pane {
            let focused_child = intern.list_child.remove(intern.focused.unwrap());
            intern.list_child.insert(
                intern.focused.unwrap(),
                add_child_container(focused_child, nw_cont)?,
            );
            //redraw_child(list_child);
            return Ok(());
        }
    }
    intern.list_child.push(nw_cont);
    intern.focused = Some(intern.list_child.len() - 1);
    //TODO: handle with the layout
    let direction = intern.layout.get_direction();
    for ch in intern.list_child.iter_mut() {
        change_rect_container(&rect_child, ch);
        match direction {
            Direction::Horizontal => rect_child.x += rect_child.w,
            Direction::Vertical => rect_child.y += rect_child.h,
        }
    }
    //redraw_child(list_child);
    Ok(())
}

fn send_input_to_child(
    intern: &mut InternSplit,
    input: [u8; 4096],
    size: usize,
) {
    if intern.focused.is_none() {
        return;
    }
    let focused_child = match intern.list_child.get_mut(intern.focused.unwrap()) {
        Some(child) => Some(child),
        None => get_focused_child(&mut intern.list_child, None, &mut intern.focused),
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
