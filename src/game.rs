use crate::objectives::generate_objectives;
use crate::player::Player;
use std::io::{self, Write};
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

pub fn play_turn(player: &mut Player, objectives_count: usize) -> i32 {
    println!(
        "Au tour de {} (Vitalité={}, Vitesse={}, Force={})",
        player.name, player.vitality, player.speed, player.strength
    );

    let objectives = generate_objectives(objectives_count);
    println!("Objectifs : {:?}", objectives);

    let total_score = Arc::new(Mutex::new(0));

    println!("→ Appuyez sur ENTREE pour démarrer...");

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap(); // Blocks until ENTER

    for &target in &objectives {
        // Variable declaration
        let counter = Arc::new(Mutex::new(0));
        let miss = Arc::new(Mutex::new(0));
        let running = Arc::new(AtomicBool::new(true));

        let counter_clone = Arc::clone(&counter);
        let miss_clone = Arc::clone(&miss);
        let running_clone = Arc::clone(&running);

        // Thread 1: Counter thread (increments every player.speed ms)
        let speed = player.speed;
        let counter_thread = thread::spawn(move || {
            while running_clone.load(Ordering::Relaxed) {
                let mut counter = counter_clone.lock().unwrap();
                let mut miss = miss_clone.lock().unwrap();

                *counter += 1;
                if *counter >= 100 {
                    *counter = 0;
                    *miss += 1;
                }

                drop(counter);
                drop(miss);
                thread::sleep(Duration::from_millis(speed)); // Controlled by player.speed
            }
        });

        // Thread 2: Display thread (updates every 30ms)
        let counter_clone = Arc::clone(&counter);
        let miss_clone = Arc::clone(&miss);
        let running_clone = Arc::clone(&running);

        let display_thread = thread::spawn(move || {
            while running_clone.load(Ordering::Relaxed) {
                let counter = counter_clone.lock().unwrap();
                let miss = miss_clone.lock().unwrap();

                print!(
                    "\rObjectif: {} - Compteur: {}  |  Miss: {} ",
                    target, *counter, *miss
                );
                std::io::Write::flush(&mut std::io::stdout()).unwrap();

                drop(counter);
                drop(miss);
                thread::sleep(Duration::from_millis(30)); // Fixed refresh rate
            }
        });

        // Main thread: Wait for ENTER (does not block counter/display)
        // wait_for_enter();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap(); // Blocks until ENTER
        running.store(false, Ordering::SeqCst); // Stop both threads

        // Ensure threads finish
        counter_thread.join().unwrap();
        display_thread.join().unwrap();

        let final_counter = *counter.lock().unwrap();
        let final_miss = *miss.lock().unwrap();
        print!(
            "\rObjectif: {} - Compteur: {}  |  Miss: {} ",
            target, final_counter, final_miss
        );

        let score = calculate_score(final_counter, final_miss, target, player.strength);
        *total_score.lock().unwrap() += score;
    }

    let avg_score = *total_score.lock().unwrap() / objectives.len() as i32;
    println!("\rScore moyen de {} : {}", player.name, avg_score);
    avg_score
}

pub fn apply_poison(winner: &Player, loser: &mut Player, diff: i32) {
    println!(
        "{} gagne la manche. {} perd {} points de vitalité.",
        winner.name, loser.name, diff
    );
    loser.vitality -= diff;

    println!("Veuillez choisir une option :");
    println!("1: -5 speed");
    println!("2: -5 strength");

    let choice = handle_input();
    loser.apply_poison(choice);

    match choice {
        1 => {
            println!(
                "Le joueur {} a perdu 5 de vitesse (Speed = {}) et passe au tour suivant...",
                loser.name, loser.speed
            );
        }
        2 => {
            println!(
                "Le joueur {} a perdu 5 de force (Strength = {}) et passe au tour suivant...",
                loser.name, loser.strength
            );
        }
        _ => unreachable!(), // Should never happen because handle_input only returns 1 or 2
    }
}

pub fn calculate_score(counter: i32, miss: i32, target: i32, strength: i32) -> i32 {
    let raw_diff = (counter - target).abs();
    let diff = raw_diff.min(100 - raw_diff); // Handle wraparound
    let miss_factor = miss + 1; // Avoid division by zero

    let base_score = match diff {
        0 => 100,
        1..=5 => 80,
        6..=10 => 60,
        11..=20 => 40,
        21..=50 => 20,
        _ => 0,
    };

    (base_score + strength) / miss_factor
}

fn handle_input() -> u8 {
    loop {
        print!("\rChoisissez une option (1 ou 2): ");
        io::stdout().flush().unwrap(); // Ensure prompt appears

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Remove any trailing whitespace (like \n or \r\n)
        let trimmed = input.trim();

        match trimmed {
            "1" => return 1,
            "2" => return 2,
            _ => {
                // Invalid input, erase the line and prompt again
                // print!("\rEntrée invalide, veuillez choisir 1 ou 2.\x1B[K"); // \x1B[K clears line
                io::stdout().flush().unwrap();
            }
        }
    }
}
