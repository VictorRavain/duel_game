mod game;
mod objectives;
mod player;

use clap::Parser;
use game::{apply_poison, play_turn};
use player::Player;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value = "Michel")]
    name1: String,
    #[arg(long, default_value = "Jacque")]
    name2: String,
    #[arg(long, default_value = "50")]
    vitality: i32,
    #[arg(long, default_value = "5")]
    objectifs: usize,
}

fn main() {
    let args = Args::parse();

    env_logger::init();

    let mut player1 = Player::new(&args.name1, args.vitality, 50, 50);
    let mut player2 = Player::new(&args.name2, args.vitality, 50, 50);

    let mut round = 1;
    while player1.vitality > 0 && player2.vitality > 0 {
        println!("##### Démarrage de la partie #####");
        println!("## Manche {} ##", round);

        let score1 = play_turn(&mut player1, args.objectifs);
        let score2 = play_turn(&mut player2, args.objectifs);

        if score1 < score2 {
            apply_poison(&player1, &mut player2, score2 - score1);
        } else {
            apply_poison(&player2, &mut player1, score1 - score2);
        }

        println!("## FIN Manche {} ##\n", round);
        round += 1;
    }

    if player1.vitality > 0 {
        println!("🏆 {} remporte la partie !", player1.name);
    } else {
        println!("🏆 {} remporte la partie !", player2.name);
    }
}
