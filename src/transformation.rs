use crate::container::*;
use crate::pane::*;
//use crate::split::*;
//use crate::layout::*;

pub fn window_child_transform(child: Container)
    -> Result<Container, ContainerError>
{
    /*match child.get_type() {
        ContainerType::Pane => {
            let pane: &Pane = match child.as_any().downcast_ref::<Pane>() {
                Some(b) => b,
                None => return Err(ContainerError::BadTransform),
            };
            //let nw_split = child.to_mini_container();
            /*let nw_split = nw_split.to_split(None,
                                             None,
                                             Direction::Horizontal,
                                             Some(ContainerMover::PaneCont(pane)))?;*/
            //Ok(Box::new(pane.to_split(Direction::Horizontal)?))
            todo!()
        }
        _ => return Err(ContainerError::BadTransform),
    }*/
    todo!()
}
