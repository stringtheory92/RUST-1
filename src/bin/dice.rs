use rand::prelude::*;

#[derive(Debug)]
enum Message {
    Quit, 
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let dice_roll = roll_dice();
    handle_roll(dice_roll);
}

fn roll_dice() -> u8 {
    println!("Rolling...");
    let mut rng = thread_rng();
    let num = rng.gen_range(1..10);
    println!("You rolled a {}", num);
    num
}

fn handle_roll(roll: u8) {
    match roll {
        3 => {
            add_fancy_hat();
            handle_roll(roll_dice());
        }
        7 => {
            remove_fancy_hat();
            handle_roll(roll_dice());
        }
        1 => {
            println!("You rolled a 1 and your turn has ended")
        }
        other => {
            move_player(other);
            handle_roll(roll_dice());
        }
    }
}

fn add_fancy_hat() {
    println!("Fancy hat added");
}
fn remove_fancy_hat() {
    println!("Fancy hat removed");
}
fn move_player(num_spaces: u8) {
    println!("Moving player {} spaces.", num_spaces);
    println!("re-rolling...");
}











