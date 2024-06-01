use rand::Rng;
use std::collections::HashMap;
use std::io;

struct Player {
    position: (i32, i32),
    health: i32,
    inventory: HashMap<String, i32>,
}

struct Enemy {
    position: (i32, i32),
    health: i32,
}

fn main() {
    let mut rng = rand::thread_rng();
    let goal_position = (rng.gen_range(-10..10), rng.gen_range(-10..10));
    let mut player = Player {
        position: (0, 0),
        health: 100,
        inventory: HashMap::new(),
    };
    let mut enemies: Vec<Enemy> = (0..5)
        .map(|_| Enemy {
            position: (rng.gen_range(-10..10), rng.gen_range(-10..10)),
            health: rng.gen_range(50..100),
        })
        .collect();

    println!("Willkommen bei Rustoid, dem verrückten Text-Abenteuer!");

    loop {
        println!("Du bist bei Position: {:?}", player.position);
        println!("Gib eine Richtung ein (N, S, O, W) oder 'I' für Inventar:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Fehler beim Lesen der Eingabe");
        let input = input.trim().to_uppercase();

        match input.as_str() {
            "N" => player.position.1 += 1,
            "S" => player.position.1 -= 1,
            "O" => player.position.0 += 1,
            "W" => player.position.0 -= 1,
            "I" => {
                println!("Inventar: {:?}", player.inventory);
                continue;
            }
            _ => {
                println!("Ungültige Richtung!");
                continue;
            }
        }

        if player.position == goal_position {
            println!("Herzlichen Glückwunsch! Du hast das Ziel bei {:?} erreicht!", goal_position);
            break;
        }

        // Check for enemies
        for enemy in &mut enemies {
            if player.position == enemy.position {
                println!("Du triffst auf einen Gegner! Kämpfe oder fliehe! (K/F)");
                let mut action = String::new();
                io::stdin().read_line(&mut action).expect("Fehler beim Lesen der Eingabe");
                let action = action.trim().to_uppercase();

                if action == "K" {
                    while player.health > 0 && enemy.health > 0 {
                        if rng.gen_bool(0.5) {
                            enemy.health -= rng.gen_range(10..20);
                            println!("Du hast den Gegner getroffen! Gegner Gesundheit: {}", enemy.health);
                        } else {
                            player.health -= rng.gen_range(10..20);
                            println!("Der Gegner hat dich getroffen! Deine Gesundheit: {}", player.health);
                        }
                    }
                    if player.health <= 0 {
                        println!("Du wurdest besiegt! Das Spiel ist vorbei.");
                        return;
                    }
                    if enemy.health <= 0 {
                        println!("Du hast den Gegner besiegt!");
                        player.inventory.insert("Gold".to_string(), rng.gen_range(10..50));
                        enemies.retain(|e| e.health > 0);
                    }
                } else {
                    println!("Du bist erfolgreich geflohen!");
                }
                break;
            }
        }

        // Random event: find item
        if rng.gen_bool(0.1) {
            let item = "Heiltrank".to_string();
            player.inventory.entry(item.clone()).and_modify(|e| *e += 1).or_insert(1);
            println!("Du hast einen {} gefunden!", item);
        }
    }
}

