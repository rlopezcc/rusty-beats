extern crate ears;
use ears::{Sound, AudioController};
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::env;
use std::io::Read;
use std::process;
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
    let args: Vec<String> = env::args().collect();
    let delay_arg: u64 = match &args.get(1) {
        &Some(v) => v.parse::<u64>().unwrap(),
        &None => 250,
    };
    let delay = time::Duration::from_millis(delay_arg);
    let mut trackfile = match File::open(&Path::new("tracks.txt")) {
        Err(why) => {
            println!("couldn't open tracks.txt: {}", why.description());
            process::exit(1);
        }
        Ok(file) => file
    };

    let mut contents: String = String::new(); 
    match trackfile.read_to_string(&mut contents) {
        Err(_) => {
            println!("Cannot read trackfile.");
            process::exit(1);
        },
        Ok(string) => string
    };

    let mut tracks: Vec<Track> = Vec::new();
    for line in contents.lines() {
        if !line.starts_with("#") {
            tracks.push(Track::from_trackfile_line(line));
        }
    }

    let mut sounds = HashMap::new();

    let mut max_track_len = 0;

    // Load sounds from tracks
    for track in &tracks {
        let path: String = "sounds/".to_string() + &(track.name).clone() + ".ogg";
        match Sound::new(&path) {
            Some(mut sound) => {
                sound.set_volume(0.3);
                sounds.insert(track.name.clone(), sound);
                if track.data.len() >= max_track_len {
                    max_track_len = track.data.len();
                }
            }
            None => {
                println!("Sound file not found {}", path);
                process::exit(1);
            }
        }

    } 

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
