use crate::container::*;
use crate::container_action::*;
use crate::layout::*;
use crate::size_utilis::*;
use std::str;
use std::sync::mpsc::Sender;

//TODO: make a new struct just for the interne in the thread because functions take to much args

pub struct Split {
    parent_com: Sender<ChildToParent>,
    list_child: ContainerList,
    layout: Layout,
    focused: Option<usize>,
    id: String,
}

impl Split {
    pub fn new(
        parent_com: Sender<ChildToParent>,
        rect: Rect,
        id: String,
        direction: Direction,
        child: Option<Container>,
    ) -> Result<Split, ContainerError> {
        let rect_clone = rect.clone();
        let layout = Layout::new(rect_clone, direction);
        let list_child: ContainerList = Vec::new();

        let mut nw_split = Split {
            parent_com,
            id,
            list_child,
            layout,
            focused: None,
        };
        if child.is_some() {
            nw_split = match nw_split.add_child(child.unwrap()) {
                Ok(Container::Split(c)) => (c),
                _ => return Err(ContainerError::CreationError),
            }
        }
        Ok(nw_split)
    }

    /// the Split struct contains multiple other contenaire that can ben pane os other Split
    /// So the draw fonction will call all the draw fonction of the child
    pub fn draw(&mut self, id: &str) {
        // TODO: see with the id how to handle
        let selfid_len = self.id.len();
        for cont in self.list_child.iter_mut() {
            let id_tmp = get_id_container(cont);
            if id_tmp.get(selfid_len..selfid_len + 1).is_some()
                == id.get(selfid_len..selfid_len + 1).is_some()
            {
                draw_container(cont, id);
            }
        }
    }

    pub fn get_input(&mut self, data: [u8; 4096], size: usize) {
        if self.focused.is_none() {
            return;
        }
        let focused_child = match self.list_child.get_mut(self.focused.unwrap()) {
            Some(child) => Some(child),
            None => get_focused_child(self, None),
        };
        if focused_child.is_none() {
            return;
        }
        get_input_container(data, size, focused_child.unwrap());
    }

    pub fn add_child(mut self, cont: Container) -> Result<Container, ContainerError> {
        let mut rect_child = self.layout.add_child();
        let nw_cont = match cont {
            Container::MiniCont(mini) => {
                let mut nw_id = self.id.to_owned();
                nw_id.push_str(&self.layout.get_next_id().to_string());
                mini.complet(
                    Some(self.parent_com.clone()),
                    Some(rect_child.clone()),
                    Some(nw_id),
                )?
            }
            other => other,
        };
        if self.focused.is_some() {
            let cont_type = get_container_type(self.list_child.get(self.focused.unwrap()).unwrap());
            if cont_type != ContainerType::Pane {
                let focused_child = self.list_child.remove(self.focused.unwrap());
                self.list_child.insert(
                    self.focused.unwrap(),
                    add_child_container(focused_child, nw_cont)?,
                );
                return Ok(Container::Split(self));
            }
        }
        self.list_child.push(nw_cont);
        self.focused = Some(self.list_child.len() - 1);
        self.update_rect_child(&mut rect_child);
        Ok(Container::Split(self))
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_type(&self) -> ContainerType {
        match self.layout.get_direction() {
            Direction::Horizontal => ContainerType::SSplit,
            Direction::Vertical => ContainerType::VSplit,
        }
    }

    pub fn identifi(&self, id_test: &str) -> bool {
        self.id.eq(id_test)
    }

    pub fn change_focus(&mut self, dir: &MoveDir) {
        let focused_child = match self.list_child.get_mut(self.focused.unwrap()) {
            Some(child) => Some(child),
            None => get_focused_child(self, None),
        };
        if focused_child.is_none() {
            return;
        }
    }

    pub fn destroy(&mut self, id: &str) -> Result<(), ()> {
        let mut i = 0;

        // if the the container to destroy is self or id == -1 => this mean detroy all child
        if self.id == id || id == "-1"{
            for cont in self.list_child.iter_mut() {
                destroy_container(cont, "-1");
            }
            self.list_child.clear();
            return Ok(());
        }
        if self.list_child.is_empty() {
            return Err(());
        }
        // supress the pane who is focused
        if id == "-2" {

        }
        for cont in self.list_child.iter_mut() {
            match destroy_container(cont, id) {
                Ok(_) => {
                    self.list_child.remove(i);
                    let mut rect_child = self.layout.del_child();
                    self.update_rect_child(&mut rect_child);
                    return Ok(());
                },
                Err(_) => (),
            }
            i += 1;
        }
        Err(())
    }

    fn update_rect_child(&mut self, rect_child: &mut Rect) {
        let direction = self.layout.get_direction();
        for ch in self.list_child.iter_mut() {
            change_rect_container(&rect_child, ch);
            match direction {
                Direction::Horizontal => rect_child.x += rect_child.w,
                Direction::Vertical => rect_child.y += rect_child.h,
            }
        }
    }

    pub fn change_rect(&mut self, _rect: &Rect) {
        // TODO
        todo!()
    }
}

fn get_focused_child<'a>(
    intern: &'a mut Split,
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
