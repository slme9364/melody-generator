extern crate rand;

use rand::Rng;

fn make_sound() -> String {
    let sounds = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "休"];
    let mut rng = rand::thread_rng();
    let random_index = rng.gen::<usize>() % sounds.len();
    let sound_string = sounds[random_index].to_string();
    sound_string
}

fn main() {
    for i in 0..8 {
        print!("{} ", make_sound());
    }
    println!("");
}
