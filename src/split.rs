use std::thread;
use std::io;
use std::sync::mpsc::{self, Sender, Receiver};
use std::fs::File;
use crate::size_utilis::*;
use crate::container::*;
use crate::layout::*;

pub struct Split {
    id: String,
    next_id: usize,
    stdio_master: File,
    parent_com: Sender<ChildToParent>,
    rect: Rect,
    intern_com: Sender<ChildToParent>,
    direction: Direction,
}

impl Container for Split {
    fn new(stdio_master: File,
           parent_com: Sender<ChildToParent>,
           rect: Rect,
           id: String)
        -> Split
        {
            let useless = mpsc::channel();
            Split {
                next_id: 1,
                stdio_master,
                parent_com,
                rect,
                id,
                intern_com: useless.0,
                direction: Direction::Horizontal,
            }
    }

    /// the Split struct contains multiple other contenaire that can ben pane os other Split
    /// So the draw fonction will call all the draw fonction of the child
    /// but because the handleling of the child is inside a thread
    /// it send the refresh commande
    fn draw(&self) {
        match self.intern_com.send(ChildToParent::Refresh) {
            Err(_) => self.parent_com.send(
                ChildToParent::DestroyChild(self.get_id())).unwrap(),
            _ => (),
        }
    }

    fn get_input(&mut self, data: [u8; 4096], size: usize) -> io::Result<()>{
        match self.intern_com.send(ChildToParent::GetInputData(data, size)) {
            Err(_) => self.parent_com.send(
                ChildToParent::DestroyChild(self.get_id())).unwrap(),
            _ => (),
        }
        Ok(())
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn identifi(&self, id_test: &String) -> bool
    {
        self.id.eq(id_test)
    }
}

impl Split {
    /// because when can't have an other arg in the Container new
    /// we use this wrapper, it will create the thread and the Split
    pub fn new_split(stdio_master: File,
           parent_com: Sender<ChildToParent>,
           rect: Rect,
           id: String,
           direction: Direction)
        -> Split
    {
        let (intern_com_tx, intern_com_rx) = mpsc::channel();
        let rect_clone = rect.clone();
        let intern_com_tx_clone = intern_com_tx.clone();
        thread::spawn( move || {
            split_thread(intern_com_rx, intern_com_tx, rect_clone, direction);
        });
        let mut nw_split = Split::new(stdio_master, parent_com, rect, id);
        nw_split.intern_com = intern_com_tx_clone;
        nw_split
    }
}

fn split_thread(receiver: Receiver<ChildToParent>,
                sender_for_child: Sender<ChildToParent>,
                base_rect: Rect,
                direction: Direction)
{
    let mut list_child: ContainerList = Vec::new();
    let mut layout = Layout::new(base_rect, direction);
    let mut focused = None;

    loop {
        let com = match receiver.recv() {
            Ok(data) => data,
            _ => break,
        };
        match com {
            ChildToParent::Refresh => redraw_child(&mut list_child),
            ChildToParent::DestroyChild(id) => destroy_child(&mut list_child, id),
            ChildToParent::AddChild(cont, cont_type) => {
                match add_child_split(&mut list_child,
                                (cont, cont_type),
                                sender_for_child.clone(),
                                &mut layout,
                                &mut focused) {
                    Err(_) => break,
                    _ => (),
                }
            },
            ChildToParent::GetInputData(input, size) => {
                send_input_to_child(&mut list_child,
                                    &mut focused,
                                    input, size)
            }
        }
    }
    //TODO: supresse this container
}

fn redraw_child(list_child: &mut ContainerList)
{
    list_child.iter_mut().for_each(|pane| {
        pane.draw();
    });
}

fn destroy_child(list_child: &mut ContainerList, id: String)
{
    let pos_child = list_child.iter().position(|child| child.identifi(&id));

    if pos_child.is_none() {
        return;
    }
    list_child.remove(pos_child.unwrap());
    redraw_child(list_child);
}

fn add_child_split(list_child: &mut ContainerList,
                   transition_cont: ContainerTran,
                   parent_com: Sender<ChildToParent>,
                   layout: &mut Layout,
                   focused: &mut Option<usize>)
    -> Result<(), ContainerError>
{
    let rect_child = layout.add_child();
    let new_cont = match transition_cont.1 {
        ContainerType::Pane => transition_cont.0.to_pane(Some(parent_com), Some(rect_child.clone()))?,
        ContainerType::SSplit => transition_cont.0.to_split(Some(parent_com),
            Some(rect_child.clone()),
            Direction::Horizontal)?,
        ContainerType::VSplit => transition_cont.0.to_split(Some(parent_com),
            Some(rect_child.clone()),
            Direction::Vertical)?,
    };
    list_child.push(Box::new(new_cont));
    *focused = Some(list_child.len() - 1);
    Ok(())
}

fn send_input_to_child(list_child: &mut ContainerList,
                       focused: &mut Option<usize>,
                       input: [u8; 4096],
                       size: usize)
{
    if focused.is_none() {
        return;
    }
    let mut focused_child = match list_child.get_mut(focused.unwrap()) {
        Some(child) => Some(child),
        None => change_focused_child(list_child, None, focused)
    };
    if focused_child.is_none() {
        return;
    }
    focused_child.unwrap().get_input(input, size);
}

fn change_focused_child<'a>(list_child: &'a mut ContainerList,
                        new_focus: Option<usize>,
                        focused: &mut Option<usize>)
    -> Option<&'a mut Box<dyn Container>>
{
    if new_focus.is_some() {
        *focused = Some(new_focus.unwrap());
        return list_child.get_mut(focused.unwrap());
    }
    if list_child.is_empty() {
        *focused = None;
        return None;
    }
    let mut tmp = focused.unwrap();
    while list_child.get(tmp).is_none() {
        tmp -= 1;
    }
    list_child.get_mut(tmp)
}
