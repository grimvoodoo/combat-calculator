use rand::Rng;

#[derive(Debug, Clone)]
struct Model {
    name: String,
    movement: i32,
    toughness: i32,
    save: i32,
    invulnerable: Option<i32>,
    wounds: i32,
    leadership: i32,
    objective_control: i32,
    weapons: Vec<Weapon>,
}

#[derive(Debug, Clone)]
struct Squad {
    name: String,
    models: Vec<Model>,
}

#[derive(Debug, Clone)]
struct Weapon {
    name: String,
    range: Option<i32>,
    attacks: i32,
    skill: i32,
    strength: i32,
    penetration: i32,
    damage: i32,
    attributes: Attributes,
}

#[derive(Debug, Default, Clone, Copy)]
struct Attributes {
    anti: bool,
    assault: bool,
    blast: bool,
    conversion: bool,
    devastating_wounds: bool,
    extra_attacks: bool,
    hazardous: bool,
    heavy: bool,
    indirect_fire: bool,
    ignores_cover: bool,
    lance: bool,
    lethal_hits: bool,
    linked_fire: bool,
    melta: bool,
    pistol: bool,
    precision: bool,
    psychic: bool,
    rapid_fire: bool,
    sustained_hits: bool,
    torrent: bool,
    twin_linked: bool,
}

fn main() {
    // Set battlefield size
    let distance = 6;
    // Build squads

    // Create the model

    let orc_model = Model {
        name: "Orcs".to_string(),
        movement: 6,
        toughness: 5,
        save: 5,
        invulnerable: None,
        wounds: 1,
        leadership: 7,
        objective_control: 2,
        weapons: Vec::new(),
    };

    let termagant_model = Model {
        name: "Termagant".to_string(),
        movement: 6,
        toughness: 3,
        save: 5,
        invulnerable: None,
        wounds: 1,
        leadership: 8,
        objective_control: 2,
        weapons: Vec::new(),
    };

    // Create the weapons

    let slugga = Weapon {
        name: "Slugga".to_string(),
        range: Some(12),
        attacks: 1,
        skill: 5,
        strength: 4,
        penetration: 0,
        damage: 1,
        attributes: Attributes {
            pistol: true,
            ..Attributes::default()
        },
    };

    let choppa = Weapon {
        name: "Choppa".to_string(),
        range: None,
        attacks: 3,
        skill: 3,
        strength: 4,
        penetration: 1,
        damage: 1,
        attributes: Attributes {
            sustained_hits: true,
            ..Attributes::default()
        },
    };

    let rokkit_launcher = Weapon {
        name: "Rokkit Launcher".to_string(),
        range: Some(24),
        attacks: d3(),
        skill: 5,
        strength: 9,
        penetration: 2,
        damage: 3,
        attributes: Attributes {
            blast: true,
            ..Attributes::default()
        },
    };

    let chitinous_claws_and_teeth = Weapon {
        name: "Chitinous claws and teeth".to_string(),
        range: None,
        attacks: 1,
        skill: 4,
        strength: 3,
        penetration: 0,
        damage: 1,
        attributes: Attributes {
            ..Attributes::default()
        },
    };

    let fleshborer = Weapon {
        name: "Fleshborer".to_string(),
        range: Some(18),
        attacks: 1,
        skill: 4,
        strength: 5,
        penetration: 0,
        damage: 1,
        attributes: Attributes {
            assault: true,
            ..Attributes::default()
        },
    };

    let shardlauncher = Weapon {
        name: "Shardlauncher".to_string(),
        range: Some(18),
        attacks: d3(),
        skill: 4,
        strength: 5,
        penetration: 0,
        damage: 1,
        attributes: Attributes {
            blast: true,
            heavy: true,
            ..Attributes::default()
        },
    };

    // Create empty squad

    let mut orc_boys = Squad {
        name: "Orc Boys".to_string(),
        models: Vec::new(),
    };

    let mut termagants = Squad {
        name: "Termagants".to_string(),
        models: Vec::new(),
    };

    // Assign model to squad

    for _ in 0..20 {
        termagants.models.push(create_model_with_weapons(
            &termagant_model,
            [fleshborer.clone(), chitinous_claws_and_teeth.clone()].to_vec(),
        ));
    }

    for _ in 0..20 {
        orc_boys.models.push(create_model_with_weapons(
            &orc_model,
            [slugga.clone(), choppa.clone()].to_vec(),
        ));
    }

    // fight
    (termagants, orc_boys) = turn(termagants, orc_boys, distance);
    println!("Surviving defenders: {:?}", orc_boys.models.len());
    println!("surviving attackers: {:?}", termagants.models.len());
}

fn d6() -> i32 {
    rand::thread_rng().gen_range(1..6)
}

fn d3() -> i32 {
    rand::thread_rng().gen_range(1..3)
}

fn create_model_with_weapons(base_model: &Model, weapons: Vec<Weapon>) -> Model {
    let mut model = base_model.clone();
    model.weapons = weapons;
    model
}

fn turn(mut attackers: Squad, mut defenders: Squad, starting_distance: i32) -> (Squad, Squad) {
    // Move
    let mut distance = starting_distance - attackers.models[0].movement;

    // Shoot

    let shooting_damage = attack(&attackers, &defenders, distance);

    defenders = apply_damage(defenders, shooting_damage);

    // Charge

    let charge: bool = if distance <= 12 {
        if d6() + d6() <= distance {
            distance = 0;
            true
        } else {
            false
        }
    } else {
        false
    };

    // Fight
    if distance == 0 {
        if charge {
            let attacker_damage = attack(&attackers, &defenders, distance);
            defenders = apply_damage(defenders, attacker_damage);
            let defender_damage = attack(&defenders, &attackers, distance);
            attackers = apply_damage(attackers, defender_damage);
        } else {
            let defender_damage = attack(&attackers, &defenders, distance);
            attackers = apply_damage(attackers, defender_damage);
            let attacker_damage = attack(&defenders, &attackers, distance);
            defenders = apply_damage(defenders, attacker_damage);
        }
    }

    // Retaliate

    (attackers, defenders)
}

fn attack(squad: &Squad, target: &Squad, distance: i32) -> Vec<i32> {
    let enemy_squad = target.clone();
    let mut total_hits = 0;
    let mut total_wounds = 0;
    let mut total_failed_saves = 0;
    let mut total_damage: Vec<i32> = Vec::new();
    for model in &squad.models {
        for weapon in &model.weapons {
            let mut hit_count = 0;
            // Check if the weapon has a range and if it's greater than or equal
            // to the distance to the target
            if let Some(range) = weapon.range {
                if range >= distance {
                    // Roll to hit
                    let hits = hit(weapon);
                    if hits.0 {
                        hit_count += 1;
                        total_hits += 1;
                    }
                } else {
                    continue;
                }
            } else if weapon.range.is_none() {
                let hits = hit(weapon);
                if hits.0 {
                    hit_count += 1;
                    total_hits += 1;
                }
            } else {
                continue;
            }

            // Roll to wound
            let mut wound_count = 0;
            for _ in 0..hit_count {
                let wound = d6();

                // Check if the wound roll is successful
                let wound_threshold = match weapon.strength {
                    str if str * 2 <= enemy_squad.models[0].toughness => 6,
                    str if str < enemy_squad.models[0].toughness => 5,
                    str if str == enemy_squad.models[0].toughness => 4,
                    str if str / 2 > enemy_squad.models[0].toughness => 2,
                    _ => 3,
                };
                if wound >= wound_threshold {
                    wound_count += 1;
                }
            }
            total_wounds += wound_count;

            let mut failed_saves = 0;
            // Roll for saves
            for _ in 0..wound_count {
                let save_roll = d6();
                let modified_save = enemy_squad.models[0].save + weapon.penetration;
                let saving_throw = enemy_squad.models[0]
                    .invulnerable
                    .unwrap_or(i32::MAX)
                    .min(modified_save);
                if save_roll < saving_throw {
                    failed_saves += 1;
                    total_damage.push(weapon.damage);
                    // println!("Failed save")
                }
            }
            total_failed_saves += failed_saves;
        }
    }
    println!("Hit Count: {:?}", total_hits);
    println!("Wound Count: {:?}", total_wounds);
    println!("Failed Saves: {:?}", total_failed_saves);
    total_damage
}

fn hit(weapon: &Weapon) -> (bool, Vec<i32>) {
    let mut results = Vec::new();
    let mut hits = false;

    for _ in 0..weapon.attacks {
        let roll = d6();
        if roll >= weapon.skill {
            results.push(roll);
            hits = true;
        }
    }
    (hits, results)
}

fn apply_damage(mut target: Squad, damage: Vec<i32>) -> Squad {
    let mut damage_iter = damage.into_iter();
    target.models.retain_mut(|model| {
        if let Some(dmg) = damage_iter.next() {
            model.wounds = model.wounds.saturating_sub(dmg);
        }
        model.wounds > 0
    });
    target
}
