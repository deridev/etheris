use crate::CommandMap;

mod skill_analyze;
mod skill_equip;
mod skill_unequip;

pub fn register_commands(map: &mut CommandMap) {
    register_command!(map, skill_equip::Skill_equipCommand);
    register_command!(map, skill_unequip::Skill_unequipCommand);
    register_command!(map, skill_analyze::Skill_analyzeCommand);
}
