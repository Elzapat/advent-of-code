use rand::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Spell<'a> {
    name: &'a str,
    dmg: i32,
    heal: i32,
    mana_cost: i32,
    effect: Option<Effect<'a>>,
}

impl std::cmp::PartialEq for Spell<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Copy, Clone)]
struct Effect<'a> {
    name: &'a str,
    timer: i32,
    dmg: i32,
    armor: i32,
    mana_regen: i32,
}

impl std::cmp::PartialEq for Effect<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn main() {
    const BOSS_HP: i32 = 58;
    const BOSS_DMG: i32 = 9;
    const MC_HP: i32 = 50;
    const MC_MANA: i32 = 500;

    let spells = vec![
        Spell {
            name: "Magic Missile",
            dmg: 4,
            heal: 0,
            mana_cost: 53,
            effect: None,
        },
        Spell {
            name: "Drain",
            dmg: 2,
            heal: 2,
            mana_cost: 73,
            effect: None,
        },
        Spell {
            name: "Shield",
            dmg: 0,
            heal: 0,
            mana_cost: 113,
            effect: Some(Effect {
                name: "Shield",
                timer: 6,
                dmg: 0,
                armor: 7,
                mana_regen: 0,
            }),
        },
        Spell {
            name: "Poison",
            dmg: 0,
            heal: 0,
            mana_cost: 173,
            effect: Some(Effect {
                name: "Poison",
                timer: 6,
                dmg: 3,
                armor: 0,
                mana_regen: 0,
            }),
        },
        Spell {
            name: "Recharge",
            dmg: 0,
            heal: 0,
            mana_cost: 229,
            effect: Some(Effect {
                name: "Recharge",
                timer: 5,
                dmg: 0,
                armor: 0,
                mana_regen: 101,
            }),
        },
    ];

    let mut rng = rand::thread_rng();
    let mut min_mana = i32::MAX;

    // Simulate 100000 battles
    for _ in 0..100000 {
        let mut boss_hp = BOSS_HP;
        let mut mc_hp = MC_HP;
        let mut mc_mana = MC_MANA;
        let mut mc_armor = 0;
        let mut mana_spent = 0;
        let mut current_effects: Vec<Effect> = vec![];
        let mut player_turn = true;

        loop {
            // Apply effects
            for effect in &mut current_effects {
                effect.timer -= 1;

                boss_hp -= effect.dmg;
                mc_mana += effect.mana_regen;

                if effect.timer <= 0 {
                    mc_armor -= effect.armor;
                }
            }

            current_effects = current_effects
                .into_iter()
                .filter(|effect| effect.timer > 0)
                .collect();

            if player_turn {
                mc_hp -= 1;
                if mc_hp <= 0 {
                    break;
                }

                let mut random_index = rng.gen_range(0..spells.len());

                while spells[random_index].effect.is_some()
                    && spells[random_index].mana_cost <= mc_mana
                    && current_effects.contains(&spells[random_index].effect.unwrap())
                {
                    random_index = rng.gen_range(0..spells.len());
                }

                let cast_spell = spells[random_index];

                if let Some(effect) = cast_spell.effect {
                    mc_armor += effect.armor;
                    current_effects.push(effect);
                }

                mc_mana -= cast_spell.mana_cost;
                mana_spent += cast_spell.mana_cost;

                if mc_mana < 0 {
                    break;
                }

                boss_hp -= cast_spell.dmg;
                mc_hp += cast_spell.heal;
            } else {
                mc_hp -= (BOSS_DMG - mc_armor).max(1);
            }

            if boss_hp <= 0 || mc_hp <= 0 {
                break;
            }

            player_turn = !player_turn;
        }

        if boss_hp <= 0 {
            min_mana = min_mana.min(mana_spent);
        }
    }

    println!("Part 2: {min_mana}");
}
