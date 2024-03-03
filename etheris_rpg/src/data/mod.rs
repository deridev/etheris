mod reward;
pub use reward::*;
pub mod enemies;
pub mod finishers;
pub mod weapons;

#[test]
fn countenemies() {
    use etheris_data::world::regions::WorldRegion;
    use std::collections::HashMap;

    let mut counter: HashMap<WorldRegion, u32> = HashMap::new();

    for enemy in enemies::ALL_ENEMIES.iter() {
        for (region, ..) in enemy.regions {
            counter.entry(*region).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    for (region, count) in counter {
        println!("{region} has {count} enemies.");
    }
}

#[test]
fn enemies_pl() {
    for enemy in enemies::ALL_ENEMIES.iter() {
        let pl = enemy.power_level();
        println!("{} -> {} PL", enemy.name, pl);
    }
}
