extern crate ears;
use ears::{Sound, AudioController};
use std::collections::HashMap;

fn main () {
    let mut sounds = HashMap::new();

    let name:&str = "sample1";
    let path:String = "src/sounds/".to_string() + name + ".ogg";

    // Create a new Sound.
    sounds.insert(name, Sound::new(&path).unwrap());

    // Play the Sound
    match sounds.get_mut(name) {
        Some(mut sound) => {sound.play(); while sound.is_playing() {}},
        _ => println!("couldn't play {}", name)
    }
}
