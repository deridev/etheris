use crate::CommandMap;

mod send_item;
mod send_orbs;

pub fn register_commands(map: &mut CommandMap) {
    register_command!(map, send_orbs::Send_orbsCommand);
    register_command!(map, send_item::Send_itemCommand);
}
