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

    fn draw(&self) {
        match self.intern_com.send(ChildToParent::Refresh) {
            Err(_) => self.parent_com.send(
                ChildToParent::DestroyChild(self.get_id())).unwrap(),
            _ => (),
        }
    }

    fn get_input(&mut self, data: &[u8], size: usize) -> io::Result<()>{
        todo!()
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
    fn new_split(stdio_master: File,
           parent_com: Sender<ChildToParent>,
           rect: Rect,
           id: String,
           direction: Direction)
        -> Split
    {
        let (intern_com_tx, intern_com_rx) = mpsc::channel();
        let rect_clone = rect.clone();
        thread::spawn( move || {
            split_thread(intern_com_rx, intern_com_tx.clone(), rect_clone, direction);
        });
        Split::new(stdio_master, parent_com, rect, id)
    }
}

fn split_thread(receiver: Receiver<ChildToParent>,
                sender_for_child: Sender<ChildToParent>,
                base_rect: Rect,
                direction: Direction)
{
    let mut list_child: ContainerList = Vec::new();
    let mut layout = Layout::new(base_rect, direction);

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
                                &mut layout) {
                    Err(_) => break,
                    _ => (),
                }
            },
            _ => (),
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
    let pos_child = pos_child.unwrap();
    list_child.remove(pos_child);
    redraw_child(list_child);
}

fn add_child_split(list_child: &mut ContainerList,
                   transition_cont: ContainerTran,
                   parent_com: Sender<ChildToParent>,
                   layout: &mut Layout)
    -> Result<(), ContainerError>
{
    let mut rect_child = layout.add_child();
    let mut new_cont = match transition_cont.1 {
        Pane => transition_cont.0.to_pane(Some(parent_com), Some(rect_child.clone()))?,
    };
    list_child.push(Box::new(new_cont));
    Ok(())
}
