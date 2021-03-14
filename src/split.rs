use crate::container::*;
use crate::container_action::*;
use crate::layout::*;
use crate::size_utilis::*;
use std::fs::File;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

//TODO: make a new struct just for the interne in the thread because functions take to much args

struct InternSplit {
    receiver: Receiver<ChildToParent>,
    com_parent: Sender<ChildToParent>,
    list_child: ContainerList,
    layout: Arc<Mutex<Layout>>,
    focused: Option<usize>,
    id: String,
}

pub struct Split {
    id: String,
    stdio_master: File,
    parent_com: Sender<ChildToParent>,
    intern_com: Sender<ChildToParent>,
    intern: InternSplit,
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
        let layout_mut = Arc::new(Mutex::new(Layout::new(rect_clone, direction)));
        let c_layout_mut = Arc::clone(&layout_mut);
        let list_child: ContainerList = Vec::new();

        let intern = InternSplit {
            receiver: intern_com_rx,
            com_parent: intern_com_tx,
            list_child,
            layout: c_layout_mut,
            focused: None,
            id: intern_id ,
        };

        let mut nw_split = Split {
            stdio_master,
            parent_com,
            id,
            intern_com: intern_com_tx_clone,
            intern,
        };
        if child.is_some() {
            nw_split = match nw_split.add_child(
                child.unwrap(),
            ) {
                Ok(Container::Split(c)) => (c),
                _ => return Err(ContainerError::CreationError),
            }
        }
        Ok(nw_split)
    }

    /// the Split struct contains multiple other contenaire that can ben pane os other Split
    /// So the draw fonction will call all the draw fonction of the child
    pub fn draw(&mut self) {
        for cont in self.intern.list_child.iter_mut() {
            draw_container(cont);
        }
    }

    pub fn get_input(&mut self, data: [u8; 4096], size: usize) {
        if self.intern.focused.is_none() {
            return;
        }
        let focused_child = match self.intern.list_child.get_mut(self.intern.focused.unwrap()) {
            Some(child) => Some(child),
            None => get_focused_child(&mut self.intern, None),
        };
        if focused_child.is_none() {
            return;
        }
        get_input_container(data, size, focused_child.unwrap());
    }

    pub fn add_child(mut self, cont: Container) -> Result<Container, ContainerError> {
        let mut rect_child = self.intern.layout.lock().unwrap().add_child();
        let nw_cont = match cont {
            Container::MiniCont(mini) => {
                let mut nw_id = self.intern.id.to_owned();
                nw_id.push_str(&self.intern.layout.lock().unwrap().get_next_id().to_string());
                mini.complet(Some(self.intern.com_parent.clone()), Some(rect_child.clone()), Some(nw_id))?
            }
            other => other,
        };
        if self.intern.focused.is_some() {
            let cont_type = get_container_type(self.intern.list_child.get(self.intern.focused.unwrap()).unwrap());
            if cont_type != ContainerType::Pane {
                let focused_child = self.intern.list_child.remove(self.intern.focused.unwrap());
                self.intern.list_child.insert(
                    self.intern.focused.unwrap(),
                    add_child_container(focused_child, nw_cont)?,
                );
                //redraw_child(list_child);
                return Ok(Container::Split(self));
            }
        }
        self.intern.list_child.push(nw_cont);
        self.intern.focused = Some(self.intern.list_child.len() - 1);
        //TODO: handle with the layout
        let direction = self.intern.layout.lock().unwrap().get_direction();
        for ch in self.intern.list_child.iter_mut() {
            change_rect_container(&rect_child, ch);
            match direction {
                Direction::Horizontal => rect_child.x += rect_child.w,
                Direction::Vertical => rect_child.y += rect_child.h,
            }
        }
        //redraw_child(list_child);
        Ok(Container::Split(self))
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_type(&self) -> ContainerType {
        match self.intern.layout.lock().unwrap().get_direction() {
            Direction::Horizontal => ContainerType::SSplit,
            Direction::Vertical => ContainerType::VSplit,
        }
    }

    pub fn identifi(&self, id_test: &String) -> bool {
        self.id.eq(id_test)
    }

    pub fn change_focus(&mut self, dir: &MoveDir) {
        let focused_child = match self.intern.list_child.get_mut(self.intern.focused.unwrap()) {
            Some(child) => Some(child),
            None => get_focused_child(&mut self.intern, None),
        };
        if focused_child.is_none() {
            return;
        }
    }

    pub fn change_rect(&mut self, _rect: &Rect) {
        // TODO
        todo!()
    }
}

fn get_focused_child<'a>(
    intern: &'a mut InternSplit,
    new_focus: Option<usize>,
) -> Option<&'a mut Container> {
    if new_focus.is_some() {
        intern.focused = Some(new_focus.unwrap());
        return intern.list_child.get_mut(intern.focused.unwrap());
    }
    if intern.list_child.is_empty() {
        intern.focused = None;
        return None;
    }
    if intern.focused.is_none() {
        return None;
    }
    let mut tmp = intern.focused.unwrap();
    while intern.list_child.get(tmp).is_none() {
        tmp -= 1;
    }
    intern.focused = Some(tmp);
    intern.list_child.get_mut(tmp)
}
