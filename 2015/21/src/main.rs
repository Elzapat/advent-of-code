#[derive(Debug, Clone, Copy, PartialEq)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

fn main() {
    const BOSS_HP: i32 = 103;
    const BOSS_DMG: i32 = 9;
    const BOSS_ARMOR: i32 = 2;
    const MC_HP: i32 = 100;

    let weapons: Vec<Item> = vec![
        Item {
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ];

    let armors: Vec<Item> = vec![
        Item {
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ];

    let rings: Vec<Item> = vec![
        Item {
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ];

    let mut armors: Vec<Option<Item>> = armors.into_iter().map(|a| Some(a)).collect();
    armors.push(None);
    let mut rings: Vec<Option<Item>> = rings.into_iter().map(|r| Some(r)).collect();
    rings.push(None);

    let mut min_cost = i32::MAX;
    let mut max_cost = 0;

    for weapon in &weapons {
        for armor in &armors {
            for ring1 in &rings {
                for ring2 in &rings {
                    if ring1 == ring2 {
                        continue;
                    }

                    let mut mc_hp = MC_HP;
                    let mut mc_dmg = 0;
                    let mut mc_armor = 0;

                    let mut total_cost = 0;

                    let mut boss_hp = BOSS_HP;

                    mc_dmg += weapon.damage;
                    total_cost += weapon.cost;

                    if let Some(armor) = armor {
                        mc_armor += armor.armor;
                        total_cost += armor.cost;
                    }

                    if let Some(ring) = ring1 {
                        mc_dmg += ring.damage;
                        mc_armor += ring.armor;
                        total_cost += ring.cost;
                    }

                    if let Some(ring) = ring2 {
                        mc_dmg += ring.damage;
                        mc_armor += ring.armor;
                        total_cost += ring.cost;
                    }

                    loop {
                        // MC turn
                        boss_hp -= (mc_dmg - BOSS_ARMOR).max(1);

                        if boss_hp <= 0 {
                            min_cost = min_cost.min(total_cost);
                            break;
                        }

                        // Boss turn
                        mc_hp -= (BOSS_DMG - mc_armor).max(1);

                        if mc_hp <= 0 {
                            max_cost = max_cost.max(total_cost);
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {min_cost}");
    println!("Part 2: {max_cost}");
}
