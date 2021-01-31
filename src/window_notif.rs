/*use crate::pane::*;
use crate::windows::*;

pub enum WindowNotif {
    Refresh,
    AddPane(Pane),
    SupressPane(PaneIdentifier),
}

pub fn handle_windows_notif(notif: WindowNotif,
                            list_pane: &mut Vec<Pane>,
                            internal_wind: &mut WindowsInterne)
{
    // TODO: add error handling
    match notif {
        WindowNotif::AddPane(pane) => add_pane(list_pane, pane, internal_wind),
        WindowNotif::Refresh => update_ui(list_pane),
        WindowNotif::SupressPane(id) => supress_pane(list_pane, id),
    }
}

fn add_pane(list_pane: &mut Vec<Pane>, pane: Pane, internal_wind: &mut WindowsInterne)
{
    list_pane.push(pane); // add the pane in the list
    internal_wind.set_focused(list_pane.len() - 1); // put the focused var in the internal info of wind
    //list_pane[internal_wind.get_focused().unwrap()].take_input().unwrap(); // the last pane take the control
    update_ui(list_pane); // print all the ui
}

fn update_ui(list_pane: &mut Vec<Pane>)
{
    /*match list_pane.first() {
        Some(pane) => pane.clear(),
        _ => return,
    }*/
    list_pane.iter_mut().rev().for_each(|pane| {
        pane.draw();
    });
}

fn supress_pane(list_pane: &mut Vec<Pane>, id: PaneIdentifier)
{
    let mut index_removed = None;

    for i in 0..list_pane.len() {
        if list_pane[i].id == id {
            index_removed = Some(i);
            list_pane.remove(i);
            break;
        }
    }
    update_ui(list_pane);
    /*if index_removed != None && index_removed != Some(0) {
        list_pane[index_removed.unwrap() - 1].take_input().unwrap();
    } else if index_removed != None && !list_pane.is_empty(){
        list_pane[index_removed.unwrap() - 1].take_input().unwrap();
    }*/
}*/
