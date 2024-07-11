mod macros;
mod prelude;
pub mod util;

use etheris_framework::Command;
use once_cell::sync::Lazy;
use std::collections::HashMap;

type BoxedCommand = Box<(dyn Command + Send + Sync)>;

#[macro_export]
macro_rules! register_command {
    ($map:expr, $command_pat:expr) => {{
        use $crate::prelude::*;
        let cmd = $command_pat;
        $map.insert(
            // Build the command with a dummy ID just to get its name
            cmd.build_command(Id::new(12345678)).command.name,
            Box::new(cmd),
        );
    }};
}

mod adm;
mod allocate;
mod battle;
mod common;
mod consume;
mod craft;
mod daily;
mod deallocate;
mod equip;
mod explore;
mod hunt;
mod infos;
mod inventory;
mod learn;
mod meditate;
mod profile;
mod read;
mod register;
mod rest;
mod sell;
mod shop;
mod skills;
mod stats;
mod study;
mod train;
mod travel;
mod tutorial;
mod unequip;
mod usecmd;
mod work;

mod test;

mod rank;
mod send;
mod skill;

pub type CommandMap = HashMap<String, BoxedCommand>;

pub static COMMANDS: Lazy<CommandMap> = Lazy::new(|| {
    let mut map: CommandMap = HashMap::new();

    register_command!(map, test::TestCommand);

    register_command!(map, common::PingCommand);
    register_command!(map, tutorial::TutorialCommand);
    register_command!(map, register::RegisterCommand);
    register_command!(map, profile::ProfileCommand);
    register_command!(map, skills::SkillsCommand);
    register_command!(map, infos::WalletCommand);
    register_command!(map, train::TrainCommand);
    register_command!(map, allocate::AllocateCommand);
    register_command!(map, deallocate::DeallocateCommand);
    register_command!(map, daily::DailyCommand);
    register_command!(map, study::StudyCommand);
    register_command!(map, battle::BattleCommand);
    register_command!(map, travel::TravelCommand);
    register_command!(map, rest::RestCommand);
    register_command!(map, learn::LearnCommand);
    register_command!(map, work::WorkCommand);
    register_command!(map, stats::StatsCommand);
    register_command!(map, shop::ShopCommand);
    register_command!(map, hunt::HuntCommand);
    register_command!(map, usecmd::UsecmdCommand);
    //register_command!(map, infos::HealthCommand);
    register_command!(map, inventory::InventoryCommand);
    register_command!(map, equip::EquipCommand);
    register_command!(map, unequip::UnequipCommand);
    register_command!(map, read::ReadCommand);
    register_command!(map, explore::ExploreCommand);
    register_command!(map, consume::ConsumeCommand);
    register_command!(map, craft::CraftCommand);
    register_command!(map, adm::AdmCommand);
    register_command!(map, sell::SellCommand);
    register_command!(map, meditate::MeditateCommand);

    send::register_commands(&mut map);
    skill::register_commands(&mut map);
    rank::register_commands(&mut map);

    map
});
