extern crate core;

use std::env;
use std::fs::File;
use std::io::Read;
use serde_json;
use std::process;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Clip {
    input_filename: String,
    output_filename: String,
    start_timestamp: i32,
    end_timestamp: i32
}

#[derive(Serialize, Deserialize)]
struct Job {
    output_args: String,
    clips: Vec<Clip>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if !args.last().unwrap().ends_with(".json") {
        panic!("Please specify an input file!");
    }
    let file = File::open(&args.last().unwrap());
    let mut contents = String::new();
    file.unwrap().read_to_string(&mut contents);
    let input: Job = serde_json::from_str(&contents).unwrap();
    let output_args = input.output_args;
    for clip in &input.clips {
        let input_file = &clip.input_filename;
        let output_file = &clip.output_filename;
        let start_time = &clip.start_timestamp;
        let end_time = &clip.end_timestamp;
        let command = format!("-loglevel quiet -i {input_file} -ss {start_time} -to {end_time} {output_args} {output_file}");
        println!("ffmpeg {}", command);
        let output =process::Command::new("ffmpeg").args(command.split(" ")).spawn();
        println!("{}", output.unwrap().wait().unwrap())
    }
}
