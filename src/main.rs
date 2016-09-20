extern crate ears;
use ears::{Sound, AudioController};
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::{thread, time};


struct Track {
    name: String,
    data: Vec<i32>
}

impl Track {
    // Create the Track instance from a line like "kick: ---+-++--"
    pub fn from_trackfile_line(line: &str) -> Track {
        let mut splitted = line.split(":");

        let name: String = splitted.next().unwrap().to_string();

        let mut data: Vec<i32> = Vec::new();

        let data_str = splitted.next().unwrap().trim();

        for sub in data_str.chars() {
            if sub == '+' {
                data.push(1);
            } else {
                data.push(0);
            }
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

    let mut tracks: Vec<Track> = Vec::new();

    for line in contents.lines() {
        tracks.push(Track::from_trackfile_line(line));
    }

    let mut sounds = HashMap::new();

    let mut max_track_len = 0;

    for track in &tracks {
        let path: String = "sounds/".to_string() + &(track.name).clone() + ".ogg";
        sounds.insert(track.name.clone(), Sound::new(&path).unwrap());
        if track.data.len() >= max_track_len {
            max_track_len = track.data.len();
        }
    } 

    let delay = time::Duration::from_millis(150);
    
    loop {
        for i in 0..max_track_len {
            for track in &tracks { 
                if track.data.len() > i && track.data[i] != 0 {
                    sounds.get_mut(&track.name).unwrap().play()
                }
            }
            thread::sleep(delay);
        }
    }
}
