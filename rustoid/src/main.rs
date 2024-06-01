use rand::Rng;
use std::io;

fn main() {
    println!("Willkommen bei Rustoid, dem verrückten Text-Abenteuer!");

    let mut player_position = (0, 0);
    let goal_position = (rand::thread_rng().gen_range(-10..10), rand::thread_rng().gen_range(-10..10));

    loop {
        println!("Du bist bei Position: {:?}", player_position);
        println!("Gib eine Richtung ein (N, S, O, W):");

        let mut direction = String::new();
        io::stdin().read_line(&mut direction).expect("Fehler beim Lesen der Eingabe");
        let direction = direction.trim().to_uppercase();

        match direction.as_str() {
            "N" => player_position.1 += 1,
            "S" => player_position.1 -= 1,
            "O" => player_position.0 += 1,
            "W" => player_position.0 -= 1,
            _ => println!("Ungültige Richtung!"),
        }

        if player_position == goal_position {
            println!("Herzlichen Glückwunsch! Du hast das Ziel bei {:?} erreicht!", goal_position);
            break;
        }

        let challenge: u8 = rand::thread_rng().gen_range(0..100);
        if challenge < 20 {
            println!("Du triffst auf einen wilden Gegner! Kämpfe oder fliehe! (K/F)");

            let mut action = String::new();
            io::stdin().read_line(&mut action).expect("Fehler beim Lesen der Eingabe");
            let action = action.trim().to_uppercase();

            if action == "K" {
                if rand::thread_rng().gen_bool(0.5) {
                    println!("Du hast den Kampf gewonnen!");
                } else {
                    println!("Du wurdest besiegt! Das Spiel ist vorbei.");
                    break;
                }
            } else {
                println!("Du bist erfolgreich geflohen!");
            }
        }
    }
}

