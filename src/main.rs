extern crate ears;
use ears::{Sound, AudioController};
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::{thread, time};


#[derive(Debug)]
struct Track {
    name: String,
    data: Vec<i32>
}

impl Track {
    pub fn from_trackfile_line(line: &str) -> Track{
        let mut splitted = line.split_whitespace();
        let name: String = splitted.next().unwrap().to_string();
        let mut data: Vec<i32> = Vec::new();
        for _ in splitted {
            data.push(1);
            data.push(0);
        }
        return Track {name: name, data: data};
    }
}

fn main () {
    let mut trackfile = match File::open(&Path::new("tracks.txt")) {
        Err(why) => panic!("couldn't open tracks.txt: {}", why.description()),
        Ok(file) => file
    };

    let mut contents: String = String::new(); 
    match trackfile.read_to_string(&mut contents) {
        Err(_) => panic!("Cannot read trackfile"),
        Ok(string) => string
    };

    let track: Track = Track::from_trackfile_line(&contents.lines().next().unwrap());

    let mut sounds = HashMap::new();

    let path: String = "sounds/".to_string() + &track.name + ".ogg";

    sounds.insert(&track.name, Sound::new(&path).unwrap());

    let ten_millis = time::Duration::from_millis(500);

    for i in 0..track.data.len() {
        if track.data[i] != 0 {
            match sounds.get_mut(&track.name) {
                Some(mut sound) => {sound.play()},
                _ => println!("couldn't play {}", &track.name)
            }
        }
        thread::sleep(ten_millis);
    }

}
