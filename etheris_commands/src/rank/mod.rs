use crate::CommandMap;

mod rank_orbs;
mod rank_pl;

pub fn register_commands(map: &mut CommandMap) {
    register_command!(map, rank_orbs::Rank_orbsCommand);
    register_command!(map, rank_pl::Rank_plCommand);
}
